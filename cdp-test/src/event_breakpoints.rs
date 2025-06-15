use crate::common::*;
/// Sets breakpoint on particular native event.
pub struct EventBreakpointsSetInstrumentationBreakpointParams {
    pub event_name: String,
}
/// Sets breakpoint on particular native event.
pub type EventBreakpointsSetInstrumentationBreakpointReturns = ();
/// Removes breakpoint on particular native event.
pub struct EventBreakpointsRemoveInstrumentationBreakpointParams {
    pub event_name: String,
}
/// Removes breakpoint on particular native event.
pub type EventBreakpointsRemoveInstrumentationBreakpointReturns = ();
/// Removes all breakpoints
pub type EventBreakpointsDisableParams = ();
/// Removes all breakpoints
pub type EventBreakpointsDisableReturns = ();
