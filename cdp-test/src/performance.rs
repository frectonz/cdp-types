use crate::common::*;
/// Run-time execution metric.
pub struct Metric {
    pub name: String,
    pub value: u64,
}
/// Disable collecting and reporting metrics.
pub type PerformanceDisableParams = ();
/// Disable collecting and reporting metrics.
pub type PerformanceDisableResults = ();
/// Enable collecting and reporting metrics.
pub type PerformanceEnableParams = ();
/// Enable collecting and reporting metrics.
pub type PerformanceEnableResults = ();
#[deprecated]
/// ⚠️ Experimental
/** Sets time domain to use for collecting and reporting duration metrics.
Note that this must be called before enabling metrics collection. Calling
this method while metrics collection is enabled returns an error.*/
pub type PerformanceSetTimeDomainParams = ();
#[deprecated]
/// ⚠️ Experimental
/** Sets time domain to use for collecting and reporting duration metrics.
Note that this must be called before enabling metrics collection. Calling
this method while metrics collection is enabled returns an error.*/
pub type PerformanceSetTimeDomainResults = ();
/// Retrieve current values of run-time metrics.
pub type PerformanceGetMetricsParams = ();
/// Retrieve current values of run-time metrics.
pub type PerformanceGetMetricsResults = ();
