/// Memory pressure level.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Memory/#type-PressureLevel>
pub enum MemoryPressureLevel {
    Moderate,
    Critical,
}
/// Heap profile sample.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Memory/#type-SamplingProfileNode>
pub struct MemorySamplingProfileNode {
    pub size: (),
    pub total: (),
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
    pub name: (),
    pub uuid: (),
    pub base_address: (),
    pub size: (),
}
/// DOM object counter data.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Memory/#type-DOMCounter>
pub struct MemoryDomCounter {
    pub name: (),
    pub count: (),
}
