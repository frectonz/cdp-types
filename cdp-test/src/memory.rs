/// Memory pressure level.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Memory/#type-PressureLevel>
pub enum MemoryPressureLevel {
    Moderate,
    Critical,
}
/// Heap profile sample.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Memory/#type-SamplingProfileNode>
pub struct MemorySamplingProfileNode {
    pub size: u64,
    pub total: u64,
    pub stack: (),
}
/// Array of heap profile samples.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Memory/#type-SamplingProfile>
pub struct MemorySamplingProfile {
    pub samples: (),
    pub modules: (),
}
/// Executable module information
/// <https://chromedevtools.github.io/devtools-protocol/tot/Memory/#type-Module>
pub struct MemoryModule {
    pub name: String,
    pub uuid: String,
    pub base_address: String,
    pub size: u64,
}
/// DOM object counter data.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Memory/#type-DOMCounter>
pub struct MemoryDomCounter {
    pub name: String,
    pub count: i64,
}
