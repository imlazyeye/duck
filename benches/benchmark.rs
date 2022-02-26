use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use criterion::{criterion_group, criterion_main, Criterion};
use duck::{
    fs::GmlWalker,
    gml::GmlCollection,
    parsing::{parser::Ast, statement::StatementBox, ParseError, Parser, TokenPilot},
    Config, Duck, LintReport,
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
                let config_arc = Arc::new(Config::default());

                // Look for files
                let (path_sender, mut path_reciever) = channel::<PathBuf>(1000);
                let walker_handle = tokio::task::spawn(async move {
                    let mut gml_walker = GmlWalker::new(Path::new(DEMO_PROJECT_PATH));
                    while let Some(path) = gml_walker.next().await {
                        path_sender.send(path).await.unwrap();
                    }
                });

                // Read files
                let (file_sender, mut file_reciever) = channel::<(PathBuf, String)>(1000);
                let file_handle = tokio::task::spawn(async move {
                    let mut io_errors: Vec<std::io::Error> = vec![];
                    while let Some(path) = path_reciever.recv().await {
                        match tokio::fs::read_to_string(&path).await {
                            Ok(gml) => {
                                file_sender.send((path, gml)).await.unwrap();
                            }
                            Err(io_error) => io_errors.push(io_error),
                        };
                    }
                    io_errors
                });

                // Parse files + early lint pass
                let (pass_one_sender, mut pass_one_reciever) = channel::<(
                    String,
                    PathBuf,
                    StatementBox,
                    GmlCollection,
                    Vec<LintReport>,
                )>(1000);
                let config = config_arc.clone();
                let pass_one_handle = tokio::task::spawn(async move {
                    let mut parse_errors: Vec<(String, PathBuf, ParseError)> = vec![];
                    while let Some((path, gml)) = file_reciever.recv().await {
                        match Duck::parse_gml(&gml, &path) {
                            Ok(ast) => {
                                for statement in ast {
                                    let config = config.clone();
                                    let gml = gml.clone();
                                    let path = path.clone();
                                    let sender = pass_one_sender.clone();
                                    tokio::task::spawn(async move {
                                        let mut reports = vec![];
                                        let mut gml_collection = GmlCollection::new();
                                        Duck::process_statement_early(
                                            config.as_ref(),
                                            &statement,
                                            &mut gml_collection,
                                            &mut reports,
                                        );
                                        sender
                                            .send((gml, path, statement, gml_collection, reports))
                                            .await
                                            .unwrap();
                                    });
                                }
                            }
                            Err(parse_error) => parse_errors.push((gml, path, parse_error)),
                        }
                    }
                    parse_errors
                });

                // Construct full collection
                let collection_handle = tokio::task::spawn(async move {
                    let mut pass_two_queue = vec![];
                    let mut master_collection = GmlCollection::new();
                    while let Some((gml, path, statement, gml_collection, reports)) =
                        pass_one_reciever.recv().await
                    {
                        master_collection.extend(gml_collection);
                        pass_two_queue.push((gml, path, statement, reports));
                    }
                    (pass_two_queue, master_collection)
                });

                // Wait for everything thus far to complete
                walker_handle.await.unwrap();
                let io_errors = file_handle.await.unwrap();
                let parse_errors = pass_one_handle.await.unwrap();
                let (pass_two_queue, master_collection) = collection_handle.await.unwrap();

                // Now we do pass two
                let config = config_arc.clone();
                let (lint_report_sender, mut lint_report_reciever) =
                    channel::<(String, PathBuf, Vec<LintReport>)>(1000);
                let pass_two_handle = tokio::task::spawn(async move {
                    let master_collection = Arc::new(master_collection);
                    for (gml, path, statement, mut lint_reports) in pass_two_queue {
                        let sender = lint_report_sender.clone();
                        let master_collection = master_collection.clone();
                        let config = config.clone();
                        tokio::task::spawn(async move {
                            Duck::process_statement_late(
                                config.as_ref(),
                                &statement,
                                master_collection.as_ref(),
                                &mut lint_reports,
                            );
                            sender.send((gml, path, lint_reports)).await.unwrap();
                        });
                    }
                });

                // Collect all the final reports
                let lint_report_handle = tokio::task::spawn(async move {
                    let mut lint_reports = vec![];
                    while let Some(values) = lint_report_reciever.recv().await {
                        lint_reports.push(values);
                    }
                    lint_reports
                });

                // We are done!
                pass_two_handle.await.unwrap();
                let lint_reports = lint_report_handle.await.unwrap();
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
