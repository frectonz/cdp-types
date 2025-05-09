use crate::io::*;
/// ⚠️ Experimental
/// Configuration for memory dump. Used only when "memory-infra" category is enabled.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Tracing/#type-MemoryDumpConfig>
pub struct TracingMemoryDumpConfig(serde_json::Map<String, serde_json::Value>);
/// <https://chromedevtools.github.io/devtools-protocol/tot/Tracing/#type-TraceConfig>
pub struct TracingTraceConfig {
    pub record_mode: String,
    pub trace_buffer_size_in_kb: u64,
    pub enable_sampling: (),
    pub enable_systrace: (),
    pub enable_argument_filter: (),
    pub included_categories: (),
    pub excluded_categories: (),
    pub synthetic_delays: (),
    pub memory_dump_config: (),
}
/// ⚠️ Experimental
/** Data format of a trace. Can be either the legacy JSON format or the
protocol buffer format. Note that the JSON format will be deprecated soon.*/
/// <https://chromedevtools.github.io/devtools-protocol/tot/Tracing/#type-StreamFormat>
pub enum TracingStreamFormat {
    Json,
    Proto,
}
/// ⚠️ Experimental
/// Compression type to use for traces returned via streams.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Tracing/#type-StreamCompression>
pub enum TracingStreamCompression {
    None,
    Gzip,
}
/// ⚠️ Experimental
/** Details exposed when memory request explicitly declared.
Keep consistent with memory_dump_request_args.h and
memory_instrumentation.mojom*/
/// <https://chromedevtools.github.io/devtools-protocol/tot/Tracing/#type-MemoryDumpLevelOfDetail>
pub enum TracingMemoryDumpLevelOfDetail {
    Background,
    Light,
    Detailed,
}
/// ⚠️ Experimental
/** Backend type to use for tracing. `chrome` uses the Chrome-integrated
tracing service and is supported on all platforms. `system` is only
supported on Chrome OS and uses the Perfetto system tracing service.
`auto` chooses `system` when the perfettoConfig provided to Tracing.start
specifies at least one non-Chrome data source; otherwise uses `chrome`.*/
/// <https://chromedevtools.github.io/devtools-protocol/tot/Tracing/#type-TracingBackend>
pub enum TracingBackend {
    Auto,
    Chrome,
    System,
}
