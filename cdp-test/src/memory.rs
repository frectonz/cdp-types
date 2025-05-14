use crate::common::*;
/// Memory pressure level.
pub enum PressureLevel {
    Moderate,
    Critical,
}
/// Heap profile sample.
pub struct SamplingProfileNode {
    pub size: Box<u64>,
    pub total: Box<u64>,
    pub stack: (),
}
/// Array of heap profile samples.
pub struct SamplingProfile {
    pub samples: (),
    pub modules: (),
}
/// Executable module information
pub struct Module {
    pub name: Box<String>,
    pub uuid: Box<String>,
    pub base_address: Box<String>,
    pub size: Box<u64>,
}
/// DOM object counter data.
pub struct DomCounter {
    pub name: Box<String>,
    pub count: Box<i64>,
}
