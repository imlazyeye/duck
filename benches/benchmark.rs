use criterion::{criterion_group, criterion_main, Criterion};
use duck::{gml::GmlCollection, parsing::Parser, Duck};

pub fn criterion_benchmark(c: &mut Criterion) {
    // Parse it all
    c.bench_function("FoM -> Ast (duck::fs)", |b| {
        b.iter(|| {
            duck::fs::visit_all_gml_files("../SwordAndField".into(), &mut vec![], |gml, path| {
                let mut source: &'static str = Box::leak(Box::new(gml));
                let _r = Parser::new(source, path).into_ast();
                unsafe {
                    drop(Box::from_raw(&mut source));
                }
            });
        });
    });

    c.bench_function("Ast -> Lint", |b| {
        let duck = Duck::new();
        let mut gml = vec![];
        duck::fs::collect_all_gml_files("../SwordAndField".into(), &mut vec![], &mut gml);
        let asts = gml
            .into_iter()
            .map(|(gml, path)| {
                let mut source: &'static str = Box::leak(Box::new(gml));
                let result = Parser::new(source, path).into_ast();
                unsafe {
                    drop(Box::from_raw(&mut source));
                }
                result.ok()
            })
            .flatten();
        b.iter(|| {
            let mut reports = vec![];
            let mut collection = GmlCollection::new();
            asts.clone().into_iter().flatten().for_each(|statement| {
                duck.process_statement_early(&statement, &mut collection, &mut reports);
            });
            asts.clone().into_iter().flatten().for_each(|statement| {
                duck.process_statement_late(&statement, &collection, &mut reports);
            });
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
