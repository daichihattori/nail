use criterion::{Criterion, Throughput, PlotConfiguration, AxisScale};

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

pub struct BenchmarkConfig {
    pub plot_config: PlotConfiguration,
    pub throughput: Throughput,
}

impl Default for BenchmarkConfig {
    fn default() -> Self {
        Self {
            plot_config: PlotConfiguration::default()
                .summary_scale(AxisScale::Logarithmic),
            throughput: Throughput::Elements(1),
        }
    }
}

impl BenchmarkConfig {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn with_throughput(mut self, throughput: Throughput) -> Self {
        self.throughput = throughput;
        self
    }
    
    pub fn with_linear_scale(mut self) -> Self {
        self.plot_config = PlotConfiguration::default()
            .summary_scale(AxisScale::Linear);
        self
    }
}