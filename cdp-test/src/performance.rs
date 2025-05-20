use crate::common::*;
/// Run-time execution metric.
pub struct Metric {
    pub name: String,
    pub value: u64,
}
pub type PerformanceDisable = ();
pub type PerformanceEnable = ();
pub type PerformanceSetTimeDomain = ();
pub type PerformanceGetMetrics = ();
