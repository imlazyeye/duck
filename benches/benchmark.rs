use std::path::Path;

use criterion::{criterion_group, criterion_main, Criterion};
use duck::{
    fs::GmlWalker,
    gml::GmlCollection,
    parsing::{Parser, TokenPilot},
    Duck,
};
// use yy_boss::{Resource, YyResource, YypBoss};

const DEMO_PROJECT_PATH: &str = "../SwordAndField";

pub fn criterion_benchmark(c: &mut Criterion) {
    // c.bench_function("Project -> fs", |b| {
    //     b.iter(|| GmlWalker::new(Path::new(DEMO_PROJECT_PATH)).collect_all());
    // });

    // c.bench_function("[async] Project -> fs", |b| {
    //     b.to_async(tokio::runtime::Runtime::new().unwrap())
    //         .iter(|| async {
    //             GmlWalker::new(Path::new(DEMO_PROJECT_PATH)).collect_all()
    //         });
    // });

    // c.bench_function("FoM -> Tokens", |b| {
    //     let (project, _) = GmlWalker::new(Path::new(DEMO_PROJECT_PATH)).collect_all();
    //     b.iter(|| {
    //         project.clone().into_iter().for_each(|(_, gml)| {
    //             let mut pilot = TokenPilot::new(&gml);
    //             while pilot.take().is_ok() {}
    //         });
    //     });
    // });

    // c.bench_function("FoM -> Tokens -> Ast", |b| {
    //     b.to_async(tokio::runtime::Runtime::new().unwrap())
    //         .iter(|| async {
    //             let mut walker = GmlWalker::new(Path::new(DEMO_PROJECT_PATH));
    //             while let Some((path, gml)) = walker.next().await {
    //                 let mut source: &'static str = Box::leak(Box::new(gml));
    //                 Parser::new(source, path).into_ast().unwrap();
    //                 unsafe {
    //                     drop(Box::from_raw(&mut source));
    //                 }
    //             }
    //         });
    // });

    c.bench_function("FoM Ast -> Lint", |b| {
        let duck = Duck::new();
        let (project, _) = GmlWalker::new(Path::new(DEMO_PROJECT_PATH)).collect_all();
        let asts = project
            .into_iter()
            .map(|(path, gml)| {
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
            // asts.clone().into_iter().flatten().for_each(|statement| {
            //     duck.process_statement_late(&statement, &collection, &mut reports);
            // });
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

fn sample_code() -> &'static str {
    "#macro foo:FOO bar
enum Foo {
    Bar,
    Buzz = 10,
    Baz = Foo.Buzz,
}
global.foo = 0;
try {
    // foo
} catch (e) {
    // bar
} finally {
    // buzz
}
function foo(bar, buzz=1) {
    if 1 && 2 || 3 xor 4 and 5 or 6 {
        foo = 7 + 8 - 9 / 10 * 11 div 12 % 13 mod 14 & 15 | 16 ^ 17 >> 18 << 19;
        if 100 == 101 != 103 > 104 >= 105 < 106 <= 107 {
            foo += 20;
            foo -= 21;
            foo *= 22;
            foo ^= 23;
            foo |= 24;
            foo &= 25;
            foo ??= 26;
            foo %= 27;
            foo = ++foo + --foo + !foo + -foo + ~foo + foo++ + foo--;
            foo = true + false + undefined + NaN + infinity + pi + \"hello\" + 10.2 + $ffffff;
            foo = global.foo + self.foo + foo.foo + foo[0] + foo[? foo] + foo[# 0, 0] + foo[| 0] + foo[$ foo];
            foo = foo ? foo: foo;
            foo = foo ?? foo;
        }
    }
}
for (var i = 0; i < 28; i++) {
    with foo {
        repeat 29 {
            do {
                while true {
                    if foo {
                        switch bar {
                            case Foo.Bar: break;
                            case Foo.Buzz: continue;
                            default: exit;
                        }
                    } else {
                        foo();
                        return;
                    }
                }
            } until foo = 30;
        }
    }
}"
}
