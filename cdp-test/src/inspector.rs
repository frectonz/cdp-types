use crate::common::*;
/// Disables inspector domain notifications.
pub type InspectorDisableParams = ();
/// Disables inspector domain notifications.
pub type InspectorDisableReturns = ();
/// Enables inspector domain notifications.
pub type InspectorEnableParams = ();
/// Enables inspector domain notifications.
pub type InspectorEnableReturns = ();
/// Fired when remote debugging connection is about to be terminated. Contains detach reason.
pub struct InspectorDetachedEvent {
    pub reason: String,
}
/// Fired when debugging target has crashed
pub type InspectorTargetCrashedEvent = String;
/// Fired when debugging target has reloaded after crash
pub type InspectorTargetReloadedAfterCrashEvent = String;
