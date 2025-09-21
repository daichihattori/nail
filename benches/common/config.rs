use criterion::{PlotConfiguration, AxisScale, Throughput};

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