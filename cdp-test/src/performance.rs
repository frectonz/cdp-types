use crate::common::*;
/// Run-time execution metric.
pub struct Metric {
    pub name: Box<String>,
    pub value: Box<u64>,
}
