use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use criterion::{criterion_group, criterion_main, Criterion};
use duck::{
    fs::GmlWalker,
    gml::GmlCollection,
    parsing::{parser::Ast, statement::StatementBox, ParseError, Parser, TokenPilot},
    Duck, LintReport,
};
use tokio::sync::{mpsc::channel, Mutex};
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

    c.bench_function("Full Test", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(|| async {
                let duck = Duck::new();
                let mut lint_reports: Vec<(&str, &Path, Vec<LintReport>)> = vec![];
                let mut parse_errors: Vec<(String, PathBuf, ParseError)> = vec![];
                let mut io_errors: Vec<std::io::Error> = vec![];
                let mut collection = GmlCollection::new();
                let mut asts: Vec<(String, PathBuf, Ast)> = vec![];
                let duck_arc = Arc::new(Mutex::new(duck));
                let lint_reports_arc = Arc::new(Mutex::new(lint_reports));
                let parse_errors_arc = Arc::new(Mutex::new(parse_errors));
                let mut gml_walker = GmlWalker::new(Path::new(DEMO_PROJECT_PATH));
                let (path_sender, mut path_reciever) = channel::<PathBuf>(1);
                let (file_sender, mut file_reciever) = channel::<(PathBuf, String)>(1);
                let (ast_sender, mut ast_receiever) = channel::<StatementBox>(1);

                // Look for files
                let walker_handle = tokio::task::spawn(async move {
                    while let Some(path) = gml_walker.next().await {
                        path_sender.send(path).await.unwrap();
                    }
                });

                // Read files
                let file_handle = tokio::task::spawn(async move {
                    while let Some(path) = path_reciever.recv().await {
                        match tokio::fs::read_to_string(&path).await {
                            Ok(gml) => {
                                file_sender.send((path, gml)).await.unwrap();
                            }
                            Err(io_error) => todo!(),
                        };
                    }
                });

                // Parse files
                let lints_remaining_arc = Arc::new(Mutex::new(0));
                let lints_remaining = lints_remaining_arc.clone();
                let duck = duck_arc.clone();
                let parse_handle = tokio::task::spawn(async move {
                    while let Some((path, gml)) = file_reciever.recv().await {
                        match Duck::parse_gml(&gml, &path) {
                            Ok(ast) => {
                                *lints_remaining.lock().await += ast.len();
                                for statement in ast {
                                    let duck = duck.clone();
                                    let lint_reports = lint_reports_arc.clone();
                                    let lints_remaining = lints_remaining.clone();
                                    tokio::task::spawn(async move {
                                        let mut duck = duck.lock().await;
                                        let mut reports = lint_reports.lock().await;
                                        let mut gml_collection = GmlCollection::new(); // todo
                                        duck.process_statement_early(
                                            &statement,
                                            &mut gml_collection,
                                            &mut vec![],
                                        );
                                        *lints_remaining.lock().await -= 1;
                                    });
                                }
                            }
                            Err(parse_error) => {
                                parse_errors_arc.lock().await.push((gml, path, parse_error))
                            }
                        }
                    }
                });

                walker_handle.await.unwrap();
                file_handle.await.unwrap();
                parse_handle.await.unwrap();
                while *lints_remaining_arc.lock().await != 0 {
                    tokio::time::sleep(std::time::Duration::from_micros(100)).await;
                }
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
