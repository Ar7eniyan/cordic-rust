use std::f64::consts::PI;

use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId, BatchSize};
use cordic::{precompute, sincos_faster};
use rand::prelude::*;

fn get_random_angle90() -> f64 {
    (random::<f64>() - 0.5f64) * PI
}

fn criterion_benchmark(c: &mut Criterion) {
    precompute();
    let mut group = c.benchmark_group("f64 sin and cos");

    for iterations in (5..=60).step_by(5) {
        group.bench_function(
            BenchmarkId::new("cordic", iterations.to_string() + " iterations"),
            |b| b.iter_batched(
                || get_random_angle90(),
                |angle| sincos_faster(angle, iterations),
                BatchSize::SmallInput
            )
        );
    }
    
    group.bench_function(
        "std",
        |b| b.iter_batched(
            || get_random_angle90(),
            |angle| (angle.sin(), angle.cos()),
            BatchSize::SmallInput
        )
    );

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);