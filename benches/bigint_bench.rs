use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput, PlotConfiguration, AxisScale};
use num_bigint::BigUint;
use rug::Integer;

fn create_biguint(bits: usize) -> BigUint {
    let bytes = bits / 8;
    let mut data = vec![0u8; bytes];
    for (i, byte) in data.iter_mut().enumerate() {
        *byte = (i * 7 + 123) as u8;
    }
    BigUint::from_bytes_le(&data)
}

fn create_rug_integer(bits: usize) -> Integer {
    let bytes = bits / 8;
    let mut data = vec![0u8; bytes];
    for (i, byte) in data.iter_mut().enumerate() {
        *byte = (i * 7 + 123) as u8;
    }
    Integer::from_digits(&data, rug::integer::Order::Lsf)
}

fn create_test_u64() -> u64 {
    0x123456789ABCDEF0u64
}

fn create_test_u128() -> u128 {
    0x123456789ABCDEF0123456789ABCDEF0u128
}

fn bench_primitive_addition(c: &mut Criterion) {
    let mut group = c.benchmark_group("Primitive Addition");
    
    let u64_a = create_test_u64();
    let u64_b = create_test_u64();
    let u128_a = create_test_u128();
    let u128_b = create_test_u128();
    
    group.bench_function("u64", |b| {
        b.iter(|| {
            let result = black_box(u64_a).wrapping_add(black_box(u64_b));
            black_box(result)
        })
    });
    
    group.bench_function("u128", |b| {
        b.iter(|| {
            let result = black_box(u128_a).wrapping_add(black_box(u128_b));
            black_box(result)
        })
    });
    
    group.finish();
}

fn bench_addition_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("Addition Comparison");
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));
    
    for bits in [64, 128, 256, 512, 1024].iter() {
        let bigint_a = create_biguint(*bits);
        let bigint_b = create_biguint(*bits);
        let rug_a = create_rug_integer(*bits);
        let rug_b = create_rug_integer(*bits);
        
        group.throughput(Throughput::Elements(1));
        group.bench_with_input(
            BenchmarkId::new("num-bigint", bits),
            bits,
            |b, _| {
                b.iter(|| {
                    let result = black_box(&bigint_a) + black_box(&bigint_b);
                    black_box(result)
                })
            },
        );
        
        group.bench_with_input(
            BenchmarkId::new("rug-gmp", bits),
            bits,
            |b, _| {
                b.iter(|| {
                    let result = black_box(&rug_a) + black_box(&rug_b);
                    black_box(result)
                })
            },
        );
    }
    group.finish();
}

fn bench_primitive_multiplication(c: &mut Criterion) {
    let mut group = c.benchmark_group("Primitive Multiplication");
    
    let u64_a = create_test_u64();
    let u64_b = create_test_u64();
    let u128_a = create_test_u128();
    let u128_b = create_test_u128();
    
    group.bench_function("u64", |b| {
        b.iter(|| {
            let result = black_box(u64_a).wrapping_mul(black_box(u64_b));
            black_box(result)
        })
    });
    
    group.bench_function("u128", |b| {
        b.iter(|| {
            let result = black_box(u128_a).wrapping_mul(black_box(u128_b));
            black_box(result)
        })
    });
    
    group.finish();
}

fn bench_multiplication_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("Multiplication Comparison");
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));
    
    for bits in [64, 128, 256, 512, 1024].iter() {
        let bigint_a = create_biguint(*bits);
        let bigint_b = create_biguint(*bits);
        let rug_a = create_rug_integer(*bits);
        let rug_b = create_rug_integer(*bits);
        
        group.throughput(Throughput::Elements(1));
        group.bench_with_input(
            BenchmarkId::new("num-bigint", bits),
            bits,
            |b, _| {
                b.iter(|| {
                    let result = black_box(&bigint_a) * black_box(&bigint_b);
                    black_box(result)
                })
            },
        );
        
        group.bench_with_input(
            BenchmarkId::new("rug-gmp", bits),
            bits,
            |b, _| {
                b.iter(|| {
                    let result = black_box(&rug_a) * black_box(&rug_b);
                    black_box(result)
                })
            },
        );
    }
    group.finish();
}

fn bench_bigint_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("BigInt Scaling");
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));
    
    for bits in [64, 128, 256, 512, 1024, 2048].iter() {
        let bigint_a = create_biguint(*bits);
        let bigint_b = create_biguint(*bits);
        let rug_a = create_rug_integer(*bits);
        let rug_b = create_rug_integer(*bits);
        
        group.throughput(Throughput::Bytes(*bits as u64 / 8));
        
        group.bench_with_input(
            BenchmarkId::new("num-bigint-add", bits),
            bits,
            |bench, _| {
                bench.iter(|| {
                    let result = black_box(&bigint_a) + black_box(&bigint_b);
                    black_box(result)
                })
            },
        );
        
        group.bench_with_input(
            BenchmarkId::new("num-bigint-mul", bits),
            bits,
            |bench, _| {
                bench.iter(|| {
                    let result = black_box(&bigint_a) * black_box(&bigint_b);
                    black_box(result)
                })
            },
        );
        
        group.bench_with_input(
            BenchmarkId::new("rug-gmp-add", bits),
            bits,
            |bench, _| {
                bench.iter(|| {
                    let result = black_box(&rug_a) + black_box(&rug_b);
                    black_box(result)
                })
            },
        );
        
        group.bench_with_input(
            BenchmarkId::new("rug-gmp-mul", bits),
            bits,
            |bench, _| {
                bench.iter(|| {
                    let result = black_box(&rug_a) * black_box(&rug_b);
                    black_box(result)
                })
            },
        );
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_primitive_addition,
    bench_primitive_multiplication,
    bench_addition_comparison,
    bench_multiplication_comparison,
    bench_bigint_scaling
);
criterion_main!(benches);