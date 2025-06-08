use crate::common::*;
/// Run-time execution metric.
pub struct Metric {
    pub name: String,
    pub value: u64,
}
/// Disable collecting and reporting metrics.
pub type PerformanceDisable = ();
/// Enable collecting and reporting metrics.
pub type PerformanceEnable = ();
/** Sets time domain to use for collecting and reporting duration metrics.
Note that this must be called before enabling metrics collection. Calling
this method while metrics collection is enabled returns an error.*/
pub type PerformanceSetTimeDomain = ();
/// Retrieve current values of run-time metrics.
pub type PerformanceGetMetrics = ();
