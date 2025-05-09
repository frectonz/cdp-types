/// Device request id.
/// <https://chromedevtools.github.io/devtools-protocol/tot/DeviceAccess/#type-RequestId>
pub struct DeviceAccessRequestId(String);
/// A device id.
/// <https://chromedevtools.github.io/devtools-protocol/tot/DeviceAccess/#type-DeviceId>
pub struct DeviceAccessDeviceId(String);
/// Device information displayed in a user prompt to select a device.
/// <https://chromedevtools.github.io/devtools-protocol/tot/DeviceAccess/#type-PromptDevice>
pub struct DeviceAccessPromptDevice {
    pub id: (),
    pub name: (),
}
