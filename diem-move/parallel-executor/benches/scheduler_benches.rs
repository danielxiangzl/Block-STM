// Copyright (c) The Diem Core Contributors
// SPDX-License-Identifier: Apache-2.0

use criterion::{criterion_group, criterion_main, Criterion};
// Run this bencher via `cargo bench --features fuzzing`.
use diem_parallel_executor::proptest_types::bencher::Bencher;
use proptest::prelude::*;

//
// Transaction benchmarks
//

fn random_benches(c: &mut Criterion) {
    let mut group = c.benchmark_group("random_benches-size-example");
    group.sample_size(10); // number of iterations the benchmark will run
    group.bench_function("random_benches", |b| {
        let bencher = Bencher::<[u8; 32], [u8; 32]>::new(10000, 1000);
        bencher.bench(&any::<[u8; 32]>(), b)
    });
}

criterion_group!(benches, random_benches);

criterion_main!(benches);
