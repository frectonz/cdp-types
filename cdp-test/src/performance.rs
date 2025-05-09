/// Run-time execution metric.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Performance/#type-Metric>
pub struct PerformanceMetric {
    pub name: String,
    pub value: u64,
}
