use crate::common::*;
/// Memory pressure level.
pub enum PressureLevel {
    Moderate,
    Critical,
}
/// Heap profile sample.
pub struct SamplingProfileNode {
    pub size: u64,
    pub total: u64,
    pub stack: Vec<String>,
}
/// Array of heap profile samples.
pub struct SamplingProfile {
    pub samples: Vec<SamplingProfileNode>,
    pub modules: Vec<Module>,
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
pub type MemoryGetDomCounters = ();
pub type MemoryGetDomCountersForLeakDetection = ();
pub type MemoryPrepareForLeakDetection = ();
pub type MemoryForciblyPurgeJavaScriptMemory = ();
pub type MemorySetPressureNotificationsSuppressed = ();
pub type MemorySimulatePressureNotification = ();
pub type MemoryStartSampling = ();
pub type MemoryStopSampling = ();
pub type MemoryGetAllTimeSamplingProfile = ();
pub type MemoryGetBrowserSamplingProfile = ();
pub type MemoryGetSamplingProfile = ();
