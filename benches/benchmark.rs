use criterion::{criterion_group, criterion_main, Criterion};
use duck::{parsing::TokenPilot, Config, Duck, DuckTask};
use std::{path::Path, sync::Arc};

const DEMO_PROJECT_PATH: &str = "../SwordAndField";

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("10k Samples Lex", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(|| {
                tokio::task::spawn(async {
                    let mut pilot = TokenPilot::new(sample_code());
                    while pilot.take().is_ok() {}
                })
            })
    });

    c.bench_function("Demo Full Process", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(|| async {
                let config_arc = Arc::new(Config::default());
                let (path_receiver, walker_handle) =
                    DuckTask::start_gml_discovery(Path::new(DEMO_PROJECT_PATH));
                let (file_receiver, file_handle) = DuckTask::start_file_load(path_receiver);
                let (parse_receiver, parse_handle) = DuckTask::start_parse(file_receiver);
                let (early_receiever, early_handle) =
                    DuckTask::start_early_pass(config_arc.clone(), parse_receiver);
                let assembly_handle = DuckTask::start_environment_assembly(early_receiever);

                // Wait for everything thus far to complete...
                walker_handle.await.unwrap();
                let _io_errors = file_handle.await.unwrap();
                let _parse_errors = parse_handle.await.unwrap();
                early_handle.await.unwrap();
                let (iterations, global_environment) = assembly_handle.await.unwrap();

                // Run the final pass...
                DuckTask::start_late_pass(config_arc.clone(), iterations, global_environment)
                    .await
                    .unwrap();
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
