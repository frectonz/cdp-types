use crate::dom::*;
/// DOM breakpoint type.
/// <https://chromedevtools.github.io/devtools-protocol/tot/DOMDebugger/#type-DOMBreakpointType>
pub enum DomDebuggerDomBreakpointType {
    SubtreeModified,
    AttributeModified,
    NodeRemoved,
}
/// ⚠️ Experimental
/// CSP Violation type.
/// <https://chromedevtools.github.io/devtools-protocol/tot/DOMDebugger/#type-CSPViolationType>
pub enum DomDebuggerCspViolationType {
    TrustedtypeSinkViolation,
    TrustedtypePolicyViolation,
}
/// Object event listener.
/// <https://chromedevtools.github.io/devtools-protocol/tot/DOMDebugger/#type-EventListener>
pub struct DomDebuggerEventListener {
    pub _type: (),
    pub use_capture: (),
    pub passive: (),
    pub once: (),
    pub script_id: (),
    pub line_number: (),
    pub column_number: (),
    pub handler: (),
    pub original_handler: (),
    pub backend_node_id: (),
}
