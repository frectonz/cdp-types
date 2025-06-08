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
pub type MemoryGetDomCountersParams = ();
/// Retruns current DOM object counters.
pub type MemoryGetDomCountersResults = ();
/// Retruns DOM object counters after preparing renderer for leak detection.
pub type MemoryGetDomCountersForLeakDetectionParams = ();
/// Retruns DOM object counters after preparing renderer for leak detection.
pub type MemoryGetDomCountersForLeakDetectionResults = ();
/** Prepares for leak detection by terminating workers, stopping spellcheckers,
dropping non-essential internal caches, running garbage collections, etc.*/
pub type MemoryPrepareForLeakDetectionParams = ();
/** Prepares for leak detection by terminating workers, stopping spellcheckers,
dropping non-essential internal caches, running garbage collections, etc.*/
pub type MemoryPrepareForLeakDetectionResults = ();
/// Simulate OomIntervention by purging V8 memory.
pub type MemoryForciblyPurgeJavaScriptMemoryParams = ();
/// Simulate OomIntervention by purging V8 memory.
pub type MemoryForciblyPurgeJavaScriptMemoryResults = ();
/// Enable/disable suppressing memory pressure notifications in all processes.
pub type MemorySetPressureNotificationsSuppressedParams = ();
/// Enable/disable suppressing memory pressure notifications in all processes.
pub type MemorySetPressureNotificationsSuppressedResults = ();
/// Simulate a memory pressure notification in all processes.
pub type MemorySimulatePressureNotificationParams = ();
/// Simulate a memory pressure notification in all processes.
pub type MemorySimulatePressureNotificationResults = ();
/// Start collecting native memory profile.
pub type MemoryStartSamplingParams = ();
/// Start collecting native memory profile.
pub type MemoryStartSamplingResults = ();
/// Stop collecting native memory profile.
pub type MemoryStopSamplingParams = ();
/// Stop collecting native memory profile.
pub type MemoryStopSamplingResults = ();
/** Retrieve native memory allocations profile
collected since renderer process startup.*/
pub type MemoryGetAllTimeSamplingProfileParams = ();
/** Retrieve native memory allocations profile
collected since renderer process startup.*/
pub type MemoryGetAllTimeSamplingProfileResults = ();
/** Retrieve native memory allocations profile
collected since browser process startup.*/
pub type MemoryGetBrowserSamplingProfileParams = ();
/** Retrieve native memory allocations profile
collected since browser process startup.*/
pub type MemoryGetBrowserSamplingProfileResults = ();
/** Retrieve native memory allocations profile collected since last
`startSampling` call.*/
pub type MemoryGetSamplingProfileParams = ();
/** Retrieve native memory allocations profile collected since last
`startSampling` call.*/
pub type MemoryGetSamplingProfileResults = ();
