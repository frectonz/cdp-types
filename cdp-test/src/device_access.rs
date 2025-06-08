use crate::common::*;
/// A device id.
pub struct DeviceId(String);
/// Device information displayed in a user prompt to select a device.
pub struct PromptDevice {
    pub id: Box<DeviceId>,
    pub name: String,
}
/// Enable events in this domain.
pub type DeviceAccessEnable = ();
/// Disable events in this domain.
pub type DeviceAccessDisable = ();
/// Select a device in response to a DeviceAccess.deviceRequestPrompted event.
pub type DeviceAccessSelectPrompt = ();
/// Cancel a prompt in response to a DeviceAccess.deviceRequestPrompted event.
pub type DeviceAccessCancelPrompt = ();
