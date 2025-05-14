pub use crate::common::*;
/// A device id.
pub struct DeviceId(String);
/// Device information displayed in a user prompt to select a device.
pub struct PromptDevice {
    pub id: (),
    pub name: String,
}
