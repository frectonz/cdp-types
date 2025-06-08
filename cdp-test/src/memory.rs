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
/// Retruns current DOM object counters.
pub type MemoryGetDomCounters = ();
/// Retruns DOM object counters after preparing renderer for leak detection.
pub type MemoryGetDomCountersForLeakDetection = ();
/** Prepares for leak detection by terminating workers, stopping spellcheckers,
dropping non-essential internal caches, running garbage collections, etc.*/
pub type MemoryPrepareForLeakDetection = ();
/// Simulate OomIntervention by purging V8 memory.
pub type MemoryForciblyPurgeJavaScriptMemory = ();
/// Enable/disable suppressing memory pressure notifications in all processes.
pub type MemorySetPressureNotificationsSuppressed = ();
/// Simulate a memory pressure notification in all processes.
pub type MemorySimulatePressureNotification = ();
/// Start collecting native memory profile.
pub type MemoryStartSampling = ();
/// Stop collecting native memory profile.
pub type MemoryStopSampling = ();
/** Retrieve native memory allocations profile
collected since renderer process startup.*/
pub type MemoryGetAllTimeSamplingProfile = ();
/** Retrieve native memory allocations profile
collected since browser process startup.*/
pub type MemoryGetBrowserSamplingProfile = ();
/** Retrieve native memory allocations profile collected since last
`startSampling` call.*/
pub type MemoryGetSamplingProfile = ();
