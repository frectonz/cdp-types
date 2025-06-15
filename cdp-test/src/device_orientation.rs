use crate::common::*;
/// Clears the overridden Device Orientation.
pub type DeviceOrientationClearDeviceOrientationOverrideParams = ();
/// Clears the overridden Device Orientation.
pub type DeviceOrientationClearDeviceOrientationOverrideReturns = ();
/// Overrides the Device Orientation.
pub struct DeviceOrientationSetDeviceOrientationOverrideParams {
    pub alpha: u64,
    pub beta: u64,
    pub gamma: u64,
}
/// Overrides the Device Orientation.
pub type DeviceOrientationSetDeviceOrientationOverrideReturns = ();
