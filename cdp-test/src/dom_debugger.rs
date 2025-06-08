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
pub type DomDebuggerGetEventListenersParams = ();
/// Returns event listeners of the given object.
pub type DomDebuggerGetEventListenersResults = ();
/// Removes DOM breakpoint that was set using `setDOMBreakpoint`.
pub type DomDebuggerRemoveDomBreakpointParams = ();
/// Removes DOM breakpoint that was set using `setDOMBreakpoint`.
pub type DomDebuggerRemoveDomBreakpointResults = ();
/// Removes breakpoint on particular DOM event.
pub type DomDebuggerRemoveEventListenerBreakpointParams = ();
/// Removes breakpoint on particular DOM event.
pub type DomDebuggerRemoveEventListenerBreakpointResults = ();
#[deprecated]
/// ⚠️ Experimental
/// Removes breakpoint on particular native event.
pub type DomDebuggerRemoveInstrumentationBreakpointParams = ();
#[deprecated]
/// ⚠️ Experimental
/// Removes breakpoint on particular native event.
pub type DomDebuggerRemoveInstrumentationBreakpointResults = ();
/// Removes breakpoint from XMLHttpRequest.
pub type DomDebuggerRemoveXhrBreakpointParams = ();
/// Removes breakpoint from XMLHttpRequest.
pub type DomDebuggerRemoveXhrBreakpointResults = ();
/// ⚠️ Experimental
/// Sets breakpoint on particular CSP violations.
pub type DomDebuggerSetBreakOnCspViolationParams = ();
/// ⚠️ Experimental
/// Sets breakpoint on particular CSP violations.
pub type DomDebuggerSetBreakOnCspViolationResults = ();
/// Sets breakpoint on particular operation with DOM.
pub type DomDebuggerSetDomBreakpointParams = ();
/// Sets breakpoint on particular operation with DOM.
pub type DomDebuggerSetDomBreakpointResults = ();
/// Sets breakpoint on particular DOM event.
pub type DomDebuggerSetEventListenerBreakpointParams = ();
/// Sets breakpoint on particular DOM event.
pub type DomDebuggerSetEventListenerBreakpointResults = ();
#[deprecated]
/// ⚠️ Experimental
/// Sets breakpoint on particular native event.
pub type DomDebuggerSetInstrumentationBreakpointParams = ();
#[deprecated]
/// ⚠️ Experimental
/// Sets breakpoint on particular native event.
pub type DomDebuggerSetInstrumentationBreakpointResults = ();
/// Sets breakpoint on XMLHttpRequest.
pub type DomDebuggerSetXhrBreakpointParams = ();
/// Sets breakpoint on XMLHttpRequest.
pub type DomDebuggerSetXhrBreakpointResults = ();
