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
pub struct MemoryGetDomCountersParams {
    pub documents: i64,
    pub nodes: i64,
    pub js_event_listeners: i64,
}
/// Retruns DOM object counters after preparing renderer for leak detection.
pub type MemoryGetDomCountersForLeakDetectionParams = ();
/// Retruns DOM object counters after preparing renderer for leak detection.
pub struct MemoryGetDomCountersForLeakDetectionParams {
    pub counters: Vec<DomCounter>,
}
/** Prepares for leak detection by terminating workers, stopping spellcheckers,
dropping non-essential internal caches, running garbage collections, etc.*/
pub type MemoryPrepareForLeakDetectionParams = ();
/** Prepares for leak detection by terminating workers, stopping spellcheckers,
dropping non-essential internal caches, running garbage collections, etc.*/
pub type MemoryPrepareForLeakDetectionReturns = ();
/// Simulate OomIntervention by purging V8 memory.
pub type MemoryForciblyPurgeJavaScriptMemoryParams = ();
/// Simulate OomIntervention by purging V8 memory.
pub type MemoryForciblyPurgeJavaScriptMemoryReturns = ();
/// Enable/disable suppressing memory pressure notifications in all processes.
pub struct MemorySetPressureNotificationsSuppressedParams {
    pub suppressed: bool,
}
/// Enable/disable suppressing memory pressure notifications in all processes.
pub type MemorySetPressureNotificationsSuppressedReturns = ();
/// Simulate a memory pressure notification in all processes.
pub struct MemorySimulatePressureNotificationParams {
    pub level: Box<PressureLevel>,
}
/// Simulate a memory pressure notification in all processes.
pub type MemorySimulatePressureNotificationReturns = ();
/// Start collecting native memory profile.
pub struct MemoryStartSamplingParams {
    pub sampling_interval: i64,
    pub suppress_randomness: bool,
}
/// Start collecting native memory profile.
pub type MemoryStartSamplingReturns = ();
/// Stop collecting native memory profile.
pub type MemoryStopSamplingParams = ();
/// Stop collecting native memory profile.
pub type MemoryStopSamplingReturns = ();
/** Retrieve native memory allocations profile
collected since renderer process startup.*/
pub type MemoryGetAllTimeSamplingProfileParams = ();
/** Retrieve native memory allocations profile
collected since renderer process startup.*/
pub struct MemoryGetAllTimeSamplingProfileParams {
    pub profile: Box<SamplingProfile>,
}
/** Retrieve native memory allocations profile
collected since browser process startup.*/
pub type MemoryGetBrowserSamplingProfileParams = ();
/** Retrieve native memory allocations profile
collected since browser process startup.*/
pub struct MemoryGetBrowserSamplingProfileParams {
    pub profile: Box<SamplingProfile>,
}
/** Retrieve native memory allocations profile collected since last
`startSampling` call.*/
pub type MemoryGetSamplingProfileParams = ();
/** Retrieve native memory allocations profile collected since last
`startSampling` call.*/
pub struct MemoryGetSamplingProfileParams {
    pub profile: Box<SamplingProfile>,
}
