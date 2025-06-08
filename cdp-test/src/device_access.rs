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
pub type DeviceAccessEnableResults = ();
/// Disable events in this domain.
pub type DeviceAccessDisableParams = ();
/// Disable events in this domain.
pub type DeviceAccessDisableResults = ();
/// Select a device in response to a DeviceAccess.deviceRequestPrompted event.
pub type DeviceAccessSelectPromptParams = ();
/// Select a device in response to a DeviceAccess.deviceRequestPrompted event.
pub type DeviceAccessSelectPromptResults = ();
/// Cancel a prompt in response to a DeviceAccess.deviceRequestPrompted event.
pub type DeviceAccessCancelPromptParams = ();
/// Cancel a prompt in response to a DeviceAccess.deviceRequestPrompted event.
pub type DeviceAccessCancelPromptResults = ();
