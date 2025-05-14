use crate::common::*;
use crate::io::*;
/// ⚠️ Experimental
/// Configuration for memory dump. Used only when "memory-infra" category is enabled.
pub struct MemoryDumpConfig(serde_json::Map<String, serde_json::Value>);
pub struct TraceConfig {
    pub record_mode: Box<String>,
    pub trace_buffer_size_in_kb: Box<u64>,
    pub enable_sampling: (),
    pub enable_systrace: (),
    pub enable_argument_filter: (),
    pub included_categories: (),
    pub excluded_categories: (),
    pub synthetic_delays: (),
    pub memory_dump_config: Box<MemoryDumpConfig>,
}
/// ⚠️ Experimental
/** Data format of a trace. Can be either the legacy JSON format or the
protocol buffer format. Note that the JSON format will be deprecated soon.*/
pub enum StreamFormat {
    Json,
    Proto,
}
/// ⚠️ Experimental
/// Compression type to use for traces returned via streams.
pub enum StreamCompression {
    None,
    Gzip,
}
/// ⚠️ Experimental
/** Details exposed when memory request explicitly declared.
Keep consistent with memory_dump_request_args.h and
memory_instrumentation.mojom*/
pub enum MemoryDumpLevelOfDetail {
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
pub enum TracingBackend {
    Auto,
    Chrome,
    System,
}
