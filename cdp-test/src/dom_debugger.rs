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
pub struct DomDebuggerGetEventListenersParams {
    test: (),
    test: (),
    test: (),
}
/// Returns event listeners of the given object.
pub type DomDebuggerGetEventListenersReturns = ();
/// Removes DOM breakpoint that was set using `setDOMBreakpoint`.
pub struct DomDebuggerRemoveDomBreakpointParams {
    test: (),
    test: (),
}
/// Removes DOM breakpoint that was set using `setDOMBreakpoint`.
pub type DomDebuggerRemoveDomBreakpointReturns = ();
/// Removes breakpoint on particular DOM event.
pub struct DomDebuggerRemoveEventListenerBreakpointParams {
    test: (),
    test: (),
}
/// Removes breakpoint on particular DOM event.
pub type DomDebuggerRemoveEventListenerBreakpointReturns = ();
#[deprecated]
/// ⚠️ Experimental
/// Removes breakpoint on particular native event.
pub struct DomDebuggerRemoveInstrumentationBreakpointParams {
    test: (),
}
#[deprecated]
/// ⚠️ Experimental
/// Removes breakpoint on particular native event.
pub type DomDebuggerRemoveInstrumentationBreakpointReturns = crate::event_breakpoints::EventBreakpointsRemoveInstrumentationBreakpointReturns;
/// Removes breakpoint from XMLHttpRequest.
pub struct DomDebuggerRemoveXhrBreakpointParams {
    test: (),
}
/// Removes breakpoint from XMLHttpRequest.
pub type DomDebuggerRemoveXhrBreakpointReturns = ();
/// ⚠️ Experimental
/// Sets breakpoint on particular CSP violations.
pub struct DomDebuggerSetBreakOnCspViolationParams {
    test: (),
}
/// ⚠️ Experimental
/// Sets breakpoint on particular CSP violations.
pub type DomDebuggerSetBreakOnCspViolationReturns = ();
/// Sets breakpoint on particular operation with DOM.
pub struct DomDebuggerSetDomBreakpointParams {
    test: (),
    test: (),
}
/// Sets breakpoint on particular operation with DOM.
pub type DomDebuggerSetDomBreakpointReturns = ();
/// Sets breakpoint on particular DOM event.
pub struct DomDebuggerSetEventListenerBreakpointParams {
    test: (),
    test: (),
}
/// Sets breakpoint on particular DOM event.
pub type DomDebuggerSetEventListenerBreakpointReturns = ();
#[deprecated]
/// ⚠️ Experimental
/// Sets breakpoint on particular native event.
pub struct DomDebuggerSetInstrumentationBreakpointParams {
    test: (),
}
#[deprecated]
/// ⚠️ Experimental
/// Sets breakpoint on particular native event.
pub type DomDebuggerSetInstrumentationBreakpointReturns = crate::event_breakpoints::EventBreakpointsSetInstrumentationBreakpointReturns;
/// Sets breakpoint on XMLHttpRequest.
pub struct DomDebuggerSetXhrBreakpointParams {
    test: (),
}
/// Sets breakpoint on XMLHttpRequest.
pub type DomDebuggerSetXhrBreakpointReturns = ();
