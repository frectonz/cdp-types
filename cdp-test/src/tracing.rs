use crate::common::*;
use crate::io::*;
/// ⚠️ Experimental
/// Configuration for memory dump. Used only when "memory-infra" category is enabled.
pub struct MemoryDumpConfig(serde_json::Map<String, serde_json::Value>);
pub struct TraceConfig {
    pub record_mode: String,
    pub trace_buffer_size_in_kb: u64,
    pub enable_sampling: bool,
    pub enable_systrace: bool,
    pub enable_argument_filter: bool,
    pub included_categories: Vec<String>,
    pub excluded_categories: Vec<String>,
    pub synthetic_delays: Vec<String>,
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
pub type TracingEnd = ();
pub type TracingGetCategories = ();
pub type TracingRecordClockSyncMarker = ();
pub type TracingRequestMemoryDump = ();
pub type TracingStart = ();
