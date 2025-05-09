use crate::dom::*;
use crate::page::*;
/// ⚠️ Experimental
/// <https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#type-SafeAreaInsets>
pub struct EmulationSafeAreaInsets {
    pub top: i64,
    pub top_max: i64,
    pub left: i64,
    pub left_max: i64,
    pub bottom: i64,
    pub bottom_max: i64,
    pub right: i64,
    pub right_max: i64,
}
/// Screen orientation.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#type-ScreenOrientation>
pub struct EmulationScreenOrientation {
    pub _type: String,
    pub angle: i64,
}
/// <https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#type-DisplayFeature>
pub struct EmulationDisplayFeature {
    pub orientation: String,
    pub offset: i64,
    pub mask_length: i64,
}
/// <https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#type-DevicePosture>
pub struct EmulationDevicePosture {
    pub _type: String,
}
/// <https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#type-MediaFeature>
pub struct EmulationMediaFeature {
    pub name: String,
    pub value: String,
}
/// ⚠️ Experimental
/** advance: If the scheduler runs out of immediate work, the virtual time base may fast forward to
allow the next delayed task (if any) to run; pause: The virtual time base may not advance;
pauseIfNetworkFetchesPending: The virtual time base may not advance if there are any pending
resource fetches.*/
/// <https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#type-VirtualTimePolicy>
pub enum EmulationVirtualTimePolicy {
    Advance,
    Pause,
    PauseIfNetworkFetchesPending,
}
/// ⚠️ Experimental
/// Used to specify User Agent Client Hints to emulate. See https://wicg.github.io/ua-client-hints
/// <https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#type-UserAgentBrandVersion>
pub struct EmulationUserAgentBrandVersion {
    pub brand: String,
    pub version: String,
}
/// ⚠️ Experimental
/** Used to specify User Agent Client Hints to emulate. See https://wicg.github.io/ua-client-hints
Missing optional values will be filled in by the target with what it would normally use.*/
/// <https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#type-UserAgentMetadata>
pub struct EmulationUserAgentMetadata {
    pub brands: (),
    pub full_version_list: (),
    pub full_version: String,
    pub platform: String,
    pub platform_version: String,
    pub architecture: String,
    pub model: String,
    pub mobile: (),
    pub bitness: String,
    pub wow64: (),
}
/// ⚠️ Experimental
/** Used to specify sensor types to emulate.
See https://w3c.github.io/sensors/#automation for more information.*/
/// <https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#type-SensorType>
pub enum EmulationSensorType {
    AbsoluteOrientation,
    Accelerometer,
    AmbientLight,
    Gravity,
    Gyroscope,
    LinearAcceleration,
    Magnetometer,
    RelativeOrientation,
}
/// ⚠️ Experimental
/// <https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#type-SensorMetadata>
pub struct EmulationSensorMetadata {
    pub available: (),
    pub minimum_frequency: u64,
    pub maximum_frequency: u64,
}
/// ⚠️ Experimental
/// <https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#type-SensorReadingSingle>
pub struct EmulationSensorReadingSingle {
    pub value: u64,
}
/// ⚠️ Experimental
/// <https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#type-SensorReadingXYZ>
pub struct EmulationSensorReadingXyz {
    pub x: u64,
    pub y: u64,
    pub z: u64,
}
/// ⚠️ Experimental
/// <https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#type-SensorReadingQuaternion>
pub struct EmulationSensorReadingQuaternion {
    pub x: u64,
    pub y: u64,
    pub z: u64,
    pub w: u64,
}
/// ⚠️ Experimental
/// <https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#type-SensorReading>
pub struct EmulationSensorReading {
    pub single: (),
    pub xyz: (),
    pub quaternion: (),
}
/// ⚠️ Experimental
/// <https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#type-PressureSource>
pub enum EmulationPressureSource {
    Cpu,
}
/// ⚠️ Experimental
/// <https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#type-PressureState>
pub enum EmulationPressureState {
    Nominal,
    Fair,
    Serious,
    Critical,
}
/// ⚠️ Experimental
/// <https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#type-PressureMetadata>
pub struct EmulationPressureMetadata {
    pub available: (),
}
/// ⚠️ Experimental
/// Enum of image types that can be disabled.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#type-DisabledImageType>
pub enum EmulationDisabledImageType {
    Avif,
    Webp,
}
