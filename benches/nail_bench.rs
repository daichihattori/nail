use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use nail::Uint;

mod common;
use common::*;

// Nail-specific test data creation functions
fn create_fixed_array_uint_64() -> Uint<1> {
    Uint::<1>::from_u64(0x123456789ABCDEF0u64)
}

fn create_fixed_array_uint_128() -> Uint<2> {
    let mut uint = Uint::<2>::zero();
    uint.limbs[0] = 0x123456789ABCDEF0u64;
    uint.limbs[1] = 0x123456789ABCDEF0u64;
    uint
}

fn create_fixed_array_uint_256() -> Uint<4> {
    let mut uint = Uint::<4>::zero();
    uint.limbs[0] = 0x123456789ABCDEF0u64;
    uint.limbs[1] = 0x123456789ABCDEF0u64;
    uint.limbs[2] = 0x123456789ABCDEF0u64;
    uint.limbs[3] = 0x123456789ABCDEF0u64;
    uint
}

fn create_fixed_array_uint_512() -> Uint<8> {
    let mut uint = Uint::<8>::zero();
    for i in 0..8 {
        uint.limbs[i] = 0x123456789ABCDEF0u64;
    }
    uint
}

fn create_fixed_array_uint_1024() -> Uint<16> {
    let mut uint = Uint::<16>::zero();
    for i in 0..16 {
        uint.limbs[i] = 0x123456789ABCDEF0u64;
    }
    uint
}

fn bench_nail_addition(c: &mut Criterion) {
    let mut group = setup_benchmark_group("Nail Addition", c);
    
    for bits in get_extended_bit_sizes() {
        match bits {
            64 => {
                let a = create_fixed_array_uint_64();
                let b = create_fixed_array_uint_64();
                group.bench_with_input(
                    BenchmarkId::new("nail", bits),
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
                    BenchmarkId::new("nail", bits),
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
                    BenchmarkId::new("nail", bits),
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
                    BenchmarkId::new("nail", bits),
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
                    BenchmarkId::new("nail", bits),
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

fn bench_nail_multiplication(c: &mut Criterion) {
    let mut group = setup_benchmark_group("Nail Multiplication", c);
    
    for bits in get_extended_bit_sizes() {
        match bits {
            64 => {
                let a = create_fixed_array_uint_64();
                let b = create_fixed_array_uint_64();
                group.bench_with_input(
                    BenchmarkId::new("nail", bits),
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
                    BenchmarkId::new("nail", bits),
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
                    BenchmarkId::new("nail", bits),
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
                    BenchmarkId::new("nail", bits),
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
                    BenchmarkId::new("nail", bits),
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

fn bench_nail_modular(c: &mut Criterion) {
    let mut group = setup_benchmark_group("Nail Modular", c);
    
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
    bench_nail_addition,
    bench_nail_multiplication,
    bench_nail_modular
);
criterion_main!(benches);