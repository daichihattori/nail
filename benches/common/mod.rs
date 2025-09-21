use criterion::{Criterion, Throughput, PlotConfiguration, AxisScale};

pub use self::test_data::*;

pub mod test_data;
pub mod config;

pub fn setup_benchmark_group<'a>(name: &str, c: &'a mut Criterion) -> criterion::BenchmarkGroup<'a, criterion::measurement::WallTime> {
    let mut group = c.benchmark_group(name);
    group.plot_config(PlotConfiguration::default()
        .summary_scale(AxisScale::Logarithmic));
    group.throughput(Throughput::Elements(1));
    group
}

pub fn get_test_bit_sizes() -> Vec<usize> {
    vec![64, 128, 256]
}

pub fn get_extended_bit_sizes() -> Vec<usize> {
    vec![64, 128, 256, 512, 1024]
}