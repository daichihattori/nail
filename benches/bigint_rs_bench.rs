use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};

mod common;
use common::*;

fn bench_bigint_rs_addition(c: &mut Criterion) {
    let mut group = setup_benchmark_group("BigInt-RS Addition", c);
    
    for bits in get_extended_bit_sizes() {
        match bits {
            64 => {
                let a = create_fixed_array_uint_64();
                let b = create_fixed_array_uint_64();
                group.bench_with_input(
                    BenchmarkId::new("bigint-rs", bits),
                    &bits,
                    |bench, _| {
                        bench.iter(|| {
                            let result = black_box(&a).add(black_box(&b));
                            black_box(result)
                        })
                    },
                );
            },
            128 => {
                let a = create_fixed_array_uint_128();
                let b = create_fixed_array_uint_128();
                group.bench_with_input(
                    BenchmarkId::new("bigint-rs", bits),
                    &bits,
                    |bench, _| {
                        bench.iter(|| {
                            let result = black_box(&a).add(black_box(&b));
                            black_box(result)
                        })
                    },
                );
            },
            256 => {
                let a = create_fixed_array_uint_256();
                let b = create_fixed_array_uint_256();
                group.bench_with_input(
                    BenchmarkId::new("bigint-rs", bits),
                    &bits,
                    |bench, _| {
                        bench.iter(|| {
                            let result = black_box(&a).add(black_box(&b));
                            black_box(result)
                        })
                    },
                );
            },
            512 => {
                let a = create_fixed_array_uint_512();
                let b = create_fixed_array_uint_512();
                group.bench_with_input(
                    BenchmarkId::new("bigint-rs", bits),
                    &bits,
                    |bench, _| {
                        bench.iter(|| {
                            let result = black_box(&a).add(black_box(&b));
                            black_box(result)
                        })
                    },
                );
            },
            1024 => {
                let a = create_fixed_array_uint_1024();
                let b = create_fixed_array_uint_1024();
                group.bench_with_input(
                    BenchmarkId::new("bigint-rs", bits),
                    &bits,
                    |bench, _| {
                        bench.iter(|| {
                            let result = black_box(&a).add(black_box(&b));
                            black_box(result)
                        })
                    },
                );
            },
            _ => unreachable!()
        }
    }
    group.finish();
}

fn bench_bigint_rs_multiplication(c: &mut Criterion) {
    let mut group = setup_benchmark_group("BigInt-RS Multiplication", c);
    
    for bits in get_extended_bit_sizes() {
        match bits {
            64 => {
                let a = create_fixed_array_uint_64();
                let b = create_fixed_array_uint_64();
                group.bench_with_input(
                    BenchmarkId::new("bigint-rs", bits),
                    &bits,
                    |bench, _| {
                        bench.iter(|| {
                            let result = black_box(&a).mul(black_box(&b));
                            black_box(result)
                        })
                    },
                );
            },
            128 => {
                let a = create_fixed_array_uint_128();
                let b = create_fixed_array_uint_128();
                group.bench_with_input(
                    BenchmarkId::new("bigint-rs", bits),
                    &bits,
                    |bench, _| {
                        bench.iter(|| {
                            let result = black_box(&a).mul(black_box(&b));
                            black_box(result)
                        })
                    },
                );
            },
            256 => {
                let a = create_fixed_array_uint_256();
                let b = create_fixed_array_uint_256();
                group.bench_with_input(
                    BenchmarkId::new("bigint-rs", bits),
                    &bits,
                    |bench, _| {
                        bench.iter(|| {
                            let result = black_box(&a).mul(black_box(&b));
                            black_box(result)
                        })
                    },
                );
            },
            512 => {
                let a = create_fixed_array_uint_512();
                let b = create_fixed_array_uint_512();
                group.bench_with_input(
                    BenchmarkId::new("bigint-rs", bits),
                    &bits,
                    |bench, _| {
                        bench.iter(|| {
                            let result = black_box(&a).mul(black_box(&b));
                            black_box(result)
                        })
                    },
                );
            },
            1024 => {
                let a = create_fixed_array_uint_1024();
                let b = create_fixed_array_uint_1024();
                group.bench_with_input(
                    BenchmarkId::new("bigint-rs", bits),
                    &bits,
                    |bench, _| {
                        bench.iter(|| {
                            let result = black_box(&a).mul(black_box(&b));
                            black_box(result)
                        })
                    },
                );
            },
            _ => unreachable!()
        }
    }
    group.finish();
}

fn bench_bigint_rs_modular(c: &mut Criterion) {
    let mut group = setup_benchmark_group("BigInt-RS Modular", c);
    
    for bits in get_test_bit_sizes() {
        match bits {
            64 => {
                let a = create_fixed_array_uint_64();
                let b = create_fixed_array_uint_64();
                group.bench_with_input(
                    BenchmarkId::new("addmod_bits", bits),
                    &bits,
                    |bench, &bits| {
                        bench.iter(|| {
                            let result = black_box(&a).addmod_bits(black_box(&b), bits);
                            black_box(result)
                        })
                    },
                );
                group.bench_with_input(
                    BenchmarkId::new("mulmod_bits", bits),
                    &bits,
                    |bench, &bits| {
                        bench.iter(|| {
                            let result = black_box(&a).mulmod_bits(black_box(&b), bits);
                            black_box(result)
                        })
                    },
                );
            },
            128 => {
                let a = create_fixed_array_uint_128();
                let b = create_fixed_array_uint_128();
                group.bench_with_input(
                    BenchmarkId::new("addmod_bits", bits),
                    &bits,
                    |bench, &bits| {
                        bench.iter(|| {
                            let result = black_box(&a).addmod_bits(black_box(&b), bits);
                            black_box(result)
                        })
                    },
                );
                group.bench_with_input(
                    BenchmarkId::new("mulmod_bits", bits),
                    &bits,
                    |bench, &bits| {
                        bench.iter(|| {
                            let result = black_box(&a).mulmod_bits(black_box(&b), bits);
                            black_box(result)
                        })
                    },
                );
            },
            256 => {
                let a = create_fixed_array_uint_256();
                let b = create_fixed_array_uint_256();
                group.bench_with_input(
                    BenchmarkId::new("addmod_bits", bits),
                    &bits,
                    |bench, &bits| {
                        bench.iter(|| {
                            let result = black_box(&a).addmod_bits(black_box(&b), bits);
                            black_box(result)
                        })
                    },
                );
                group.bench_with_input(
                    BenchmarkId::new("mulmod_bits", bits),
                    &bits,
                    |bench, &bits| {
                        bench.iter(|| {
                            let result = black_box(&a).mulmod_bits(black_box(&b), bits);
                            black_box(result)
                        })
                    },
                );
            },
            _ => unreachable!()
        }
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_bigint_rs_addition,
    bench_bigint_rs_multiplication,
    bench_bigint_rs_modular
);
criterion_main!(benches);