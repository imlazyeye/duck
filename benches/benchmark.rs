use criterion::{criterion_group, criterion_main, Criterion};
use duck::{
    parsing::{statement::StatementBox, Parser},
    Duck,
};
use yy_boss::{Resource, YyResource, YypBoss};

pub fn criterion_benchmark(c: &mut Criterion) {
    let boss = YypBoss::with_startup_injest(
        "../SwordAndField/FieldsOfMistria.yyp",
        &[Resource::Script, Resource::Object],
    )
    .unwrap();

    // Parse it all
    let gml = boss
        .scripts
        .into_iter()
        .map(|script| {
            (
                script.associated_data.clone().unwrap(),
                script
                    .yy_resource
                    .relative_yy_directory()
                    .join(format!("{}.gml", &script.yy_resource.resource_data.name)),
            )
        })
        .chain(boss.objects.into_iter().flat_map(|object| {
            object
                .associated_data
                .as_ref()
                .unwrap()
                .iter()
                .map(|(event_type, gml_content)| {
                    (
                        gml_content.to_string(),
                        object
                            .yy_resource
                            .relative_yy_directory()
                            .join(format!("{}.gml", event_type.filename_simple())),
                    )
                })
        }));

    c.bench_function("FoM -> Ast", |b| {
        b.iter(|| {
            for (gml_file, path) in gml.clone() {
                let mut source: &'static str = Box::leak(Box::new(gml_file));
                Parser::new(source, path.to_path_buf()).into_ast().unwrap();
                unsafe {
                    drop(Box::from_raw(&mut source));
                }
            }
        });
    });

    let duck = Duck::new();
    let statements = gml
        .into_iter()
        .flat_map(|(gml_file, path)| {
            Parser::new(Box::leak(Box::new(gml_file)), path)
                .into_ast()
                .unwrap()
        })
        .collect::<Vec<StatementBox>>();
    c.bench_function("Ast -> Lint", |b| {
        b.iter(|| {
            let mut reports = vec![];
            statements.clone().into_iter().for_each(|statement| {
                duck.lint_statement(&statement, &mut reports);
            })
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
