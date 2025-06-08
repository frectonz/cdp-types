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
/// Returns event listeners of the given object.
pub type DomDebuggerGetEventListeners = ();
/// Removes DOM breakpoint that was set using `setDOMBreakpoint`.
pub type DomDebuggerRemoveDomBreakpoint = ();
/// Removes breakpoint on particular DOM event.
pub type DomDebuggerRemoveEventListenerBreakpoint = ();
/// Removes breakpoint on particular native event.
pub type DomDebuggerRemoveInstrumentationBreakpoint = ();
/// Removes breakpoint from XMLHttpRequest.
pub type DomDebuggerRemoveXhrBreakpoint = ();
/// Sets breakpoint on particular CSP violations.
pub type DomDebuggerSetBreakOnCspViolation = ();
/// Sets breakpoint on particular operation with DOM.
pub type DomDebuggerSetDomBreakpoint = ();
/// Sets breakpoint on particular DOM event.
pub type DomDebuggerSetEventListenerBreakpoint = ();
/// Sets breakpoint on particular native event.
pub type DomDebuggerSetInstrumentationBreakpoint = ();
/// Sets breakpoint on XMLHttpRequest.
pub type DomDebuggerSetXhrBreakpoint = ();
