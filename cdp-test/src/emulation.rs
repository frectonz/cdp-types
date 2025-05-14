use crate::common::*;
use crate::dom::*;
use crate::page::*;
/// ⚠️ Experimental
pub struct SafeAreaInsets {
    pub top: Box<i64>,
    pub top_max: Box<i64>,
    pub left: Box<i64>,
    pub left_max: Box<i64>,
    pub bottom: Box<i64>,
    pub bottom_max: Box<i64>,
    pub right: Box<i64>,
    pub right_max: Box<i64>,
}
/// Screen orientation.
pub struct ScreenOrientation {
    pub _type: Box<String>,
    pub angle: Box<i64>,
}
pub struct DisplayFeature {
    pub orientation: Box<String>,
    pub offset: Box<i64>,
    pub mask_length: Box<i64>,
}
pub struct DevicePosture {
    pub _type: Box<String>,
}
pub struct MediaFeature {
    pub name: Box<String>,
    pub value: Box<String>,
}
/// ⚠️ Experimental
/** advance: If the scheduler runs out of immediate work, the virtual time base may fast forward to
allow the next delayed task (if any) to run; pause: The virtual time base may not advance;
pauseIfNetworkFetchesPending: The virtual time base may not advance if there are any pending
resource fetches.*/
pub enum VirtualTimePolicy {
    Advance,
    Pause,
    PauseIfNetworkFetchesPending,
}
/// ⚠️ Experimental
/// Used to specify User Agent Client Hints to emulate. See https://wicg.github.io/ua-client-hints
pub struct UserAgentBrandVersion {
    pub brand: Box<String>,
    pub version: Box<String>,
}
/// ⚠️ Experimental
/** Used to specify User Agent Client Hints to emulate. See https://wicg.github.io/ua-client-hints
Missing optional values will be filled in by the target with what it would normally use.*/
pub struct UserAgentMetadata {
    pub brands: (),
    pub full_version_list: (),
    pub full_version: Box<String>,
    pub platform: Box<String>,
    pub platform_version: Box<String>,
    pub architecture: Box<String>,
    pub model: Box<String>,
    pub mobile: (),
    pub bitness: Box<String>,
    pub wow64: (),
}
/// ⚠️ Experimental
/** Used to specify sensor types to emulate.
See https://w3c.github.io/sensors/#automation for more information.*/
pub enum SensorType {
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
pub struct SensorMetadata {
    pub available: (),
    pub minimum_frequency: Box<u64>,
    pub maximum_frequency: Box<u64>,
}
/// ⚠️ Experimental
pub struct SensorReadingSingle {
    pub value: Box<u64>,
}
/// ⚠️ Experimental
pub struct SensorReadingXyz {
    pub x: Box<u64>,
    pub y: Box<u64>,
    pub z: Box<u64>,
}
/// ⚠️ Experimental
pub struct SensorReadingQuaternion {
    pub x: Box<u64>,
    pub y: Box<u64>,
    pub z: Box<u64>,
    pub w: Box<u64>,
}
/// ⚠️ Experimental
pub struct SensorReading {
    pub single: Box<SensorReadingSingle>,
    pub xyz: Box<SensorReadingXyz>,
    pub quaternion: Box<SensorReadingQuaternion>,
}
/// ⚠️ Experimental
pub enum PressureSource {
    Cpu,
}
/// ⚠️ Experimental
pub enum PressureState {
    Nominal,
    Fair,
    Serious,
    Critical,
}
/// ⚠️ Experimental
pub struct PressureMetadata {
    pub available: (),
}
/// ⚠️ Experimental
/// Enum of image types that can be disabled.
pub enum DisabledImageType {
    Avif,
    Webp,
}
