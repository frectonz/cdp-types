pub use crate::common::*;
use crate::dom::*;
use crate::page::*;
/// ⚠️ Experimental
pub struct SafeAreaInsets {
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
pub struct ScreenOrientation {
    pub _type: String,
    pub angle: i64,
}
pub struct DisplayFeature {
    pub orientation: String,
    pub offset: i64,
    pub mask_length: i64,
}
pub struct DevicePosture {
    pub _type: String,
}
pub struct MediaFeature {
    pub name: String,
    pub value: String,
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
    pub brand: String,
    pub version: String,
}
/// ⚠️ Experimental
/** Used to specify User Agent Client Hints to emulate. See https://wicg.github.io/ua-client-hints
Missing optional values will be filled in by the target with what it would normally use.*/
pub struct UserAgentMetadata {
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
    pub minimum_frequency: u64,
    pub maximum_frequency: u64,
}
/// ⚠️ Experimental
pub struct SensorReadingSingle {
    pub value: u64,
}
/// ⚠️ Experimental
pub struct SensorReadingXyz {
    pub x: u64,
    pub y: u64,
    pub z: u64,
}
/// ⚠️ Experimental
pub struct SensorReadingQuaternion {
    pub x: u64,
    pub y: u64,
    pub z: u64,
    pub w: u64,
}
/// ⚠️ Experimental
pub struct SensorReading {
    pub single: (),
    pub xyz: (),
    pub quaternion: (),
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
