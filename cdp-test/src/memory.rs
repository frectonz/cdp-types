pub use crate::common::*;
/// Memory pressure level.
pub enum PressureLevel {
    Moderate,
    Critical,
}
/// Heap profile sample.
pub struct SamplingProfileNode {
    pub size: u64,
    pub total: u64,
    pub stack: (),
}
/// Array of heap profile samples.
pub struct SamplingProfile {
    pub samples: (),
    pub modules: (),
}
/// Executable module information
pub struct Module {
    pub name: String,
    pub uuid: String,
    pub base_address: String,
    pub size: u64,
}
/// DOM object counter data.
pub struct DomCounter {
    pub name: String,
    pub count: i64,
}
