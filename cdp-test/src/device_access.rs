use crate::common::*;
/// A device id.
pub struct DeviceId(String);
/// Device information displayed in a user prompt to select a device.
pub struct PromptDevice {
    pub id: Box<DeviceId>,
    pub name: String,
}
/// Enable events in this domain.
pub type DeviceAccessEnableParams = ();
/// Enable events in this domain.
pub type DeviceAccessEnableReturns = ();
/// Disable events in this domain.
pub type DeviceAccessDisableParams = ();
/// Disable events in this domain.
pub type DeviceAccessDisableReturns = ();
/// Select a device in response to a DeviceAccess.deviceRequestPrompted event.
pub struct DeviceAccessSelectPromptParams {
    pub id: Box<NetworkRequestId>,
    pub device_id: Box<DeviceId>,
}
/// Select a device in response to a DeviceAccess.deviceRequestPrompted event.
pub type DeviceAccessSelectPromptReturns = ();
/// Cancel a prompt in response to a DeviceAccess.deviceRequestPrompted event.
pub struct DeviceAccessCancelPromptParams {
    pub id: Box<NetworkRequestId>,
}
/// Cancel a prompt in response to a DeviceAccess.deviceRequestPrompted event.
pub type DeviceAccessCancelPromptReturns = ();
