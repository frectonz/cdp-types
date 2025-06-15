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
    pub object_id: Box<()>,
    pub depth: i64,
    pub pierce: bool,
}
/// Returns event listeners of the given object.
pub struct DomDebuggerGetEventListenersParams {
    pub listeners: Vec<EventListener>,
}
/// Removes DOM breakpoint that was set using `setDOMBreakpoint`.
pub struct DomDebuggerRemoveDomBreakpointParams {
    pub node_id: Box<NodeId>,
    pub _type: Box<DomBreakpointType>,
}
/// Removes DOM breakpoint that was set using `setDOMBreakpoint`.
pub type DomDebuggerRemoveDomBreakpointReturns = ();
/// Removes breakpoint on particular DOM event.
pub struct DomDebuggerRemoveEventListenerBreakpointParams {
    pub event_name: String,
    pub target_name: String,
}
/// Removes breakpoint on particular DOM event.
pub type DomDebuggerRemoveEventListenerBreakpointReturns = ();
#[deprecated]
/// ⚠️ Experimental
/// Removes breakpoint on particular native event.
pub struct DomDebuggerRemoveInstrumentationBreakpointParams {
    pub event_name: String,
}
#[deprecated]
/// ⚠️ Experimental
/// Removes breakpoint on particular native event.
pub type DomDebuggerRemoveInstrumentationBreakpointReturns = crate::event_breakpoints::EventBreakpointsRemoveInstrumentationBreakpointReturns;
/// Removes breakpoint from XMLHttpRequest.
pub struct DomDebuggerRemoveXhrBreakpointParams {
    pub url: String,
}
/// Removes breakpoint from XMLHttpRequest.
pub type DomDebuggerRemoveXhrBreakpointReturns = ();
/// ⚠️ Experimental
/// Sets breakpoint on particular CSP violations.
pub struct DomDebuggerSetBreakOnCspViolationParams {
    pub violation_types: Vec<CspViolationType>,
}
/// ⚠️ Experimental
/// Sets breakpoint on particular CSP violations.
pub type DomDebuggerSetBreakOnCspViolationReturns = ();
/// Sets breakpoint on particular operation with DOM.
pub struct DomDebuggerSetDomBreakpointParams {
    pub node_id: Box<NodeId>,
    pub _type: Box<DomBreakpointType>,
}
/// Sets breakpoint on particular operation with DOM.
pub type DomDebuggerSetDomBreakpointReturns = ();
/// Sets breakpoint on particular DOM event.
pub struct DomDebuggerSetEventListenerBreakpointParams {
    pub event_name: String,
    pub target_name: String,
}
/// Sets breakpoint on particular DOM event.
pub type DomDebuggerSetEventListenerBreakpointReturns = ();
#[deprecated]
/// ⚠️ Experimental
/// Sets breakpoint on particular native event.
pub struct DomDebuggerSetInstrumentationBreakpointParams {
    pub event_name: String,
}
#[deprecated]
/// ⚠️ Experimental
/// Sets breakpoint on particular native event.
pub type DomDebuggerSetInstrumentationBreakpointReturns = crate::event_breakpoints::EventBreakpointsSetInstrumentationBreakpointReturns;
/// Sets breakpoint on XMLHttpRequest.
pub struct DomDebuggerSetXhrBreakpointParams {
    pub url: String,
}
/// Sets breakpoint on XMLHttpRequest.
pub type DomDebuggerSetXhrBreakpointReturns = ();
