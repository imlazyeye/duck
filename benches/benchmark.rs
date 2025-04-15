use criterion::{Criterion, criterion_group, criterion_main};
use duck::Duck;
use std::path::Path;

const DEMO_PROJECT_PATH: &str = "../SwordAndField";

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Demo Full Process", |b| {
        b.iter(|| {
            let duck = Duck::default();
            duck.run_blocking(Path::new(DEMO_PROJECT_PATH)).unwrap();
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
