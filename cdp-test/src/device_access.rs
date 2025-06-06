use crate::common::*;
/// A device id.
pub struct DeviceId(String);
/// Device information displayed in a user prompt to select a device.
pub struct PromptDevice {
    pub id: Box<DeviceId>,
    pub name: String,
}
pub type DeviceAccessEnable = ();
pub type DeviceAccessDisable = ();
pub type DeviceAccessSelectPrompt = ();
pub type DeviceAccessCancelPrompt = ();
