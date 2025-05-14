pub use crate::common::*;
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
    pub _type: String,
    pub use_capture: (),
    pub passive: (),
    pub once: (),
    pub script_id: (),
    pub line_number: i64,
    pub column_number: i64,
    pub handler: (),
    pub original_handler: (),
    pub backend_node_id: (),
}
