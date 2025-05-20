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
    pub _type: String,
    pub use_capture: bool,
    pub passive: bool,
    pub once: bool,
    pub script_id: Box<()>,
    pub line_number: i64,
    pub column_number: i64,
    pub handler: Box<()>,
    pub original_handler: Box<()>,
    pub backend_node_id: Box<BackendNodeId>,
}
pub type DomDebuggerGetEventListeners = ();
pub type DomDebuggerRemoveDomBreakpoint = ();
pub type DomDebuggerRemoveEventListenerBreakpoint = ();
pub type DomDebuggerRemoveInstrumentationBreakpoint = ();
pub type DomDebuggerRemoveXhrBreakpoint = ();
pub type DomDebuggerSetBreakOnCspViolation = ();
pub type DomDebuggerSetDomBreakpoint = ();
pub type DomDebuggerSetEventListenerBreakpoint = ();
pub type DomDebuggerSetInstrumentationBreakpoint = ();
pub type DomDebuggerSetXhrBreakpoint = ();
