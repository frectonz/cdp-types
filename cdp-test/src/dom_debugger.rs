use crate::common::*;
use crate::dom::*;
/// DOM breakpoint type.
pub enum DomBreakpointType {
    SubtreeModified,
    AttributeModified,
    NodeRemoved,
}
/// ⚠️ Experimental
/// CSP Violation type.
pub enum CspViolationType {
    TrustedtypeSinkViolation,
    TrustedtypePolicyViolation,
}
/// Object event listener.
pub struct EventListener {
    pub _type: Box<String>,
    pub use_capture: (),
    pub passive: (),
    pub once: (),
    pub script_id: Box<ScriptId>,
    pub line_number: Box<i64>,
    pub column_number: Box<i64>,
    pub handler: Box<RemoteObject>,
    pub original_handler: Box<RemoteObject>,
    pub backend_node_id: Box<BackendNodeId>,
}
