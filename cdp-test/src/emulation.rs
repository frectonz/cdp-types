use crate::common::*;
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
    pub brands: Vec<UserAgentBrandVersion>,
    pub full_version_list: Vec<UserAgentBrandVersion>,
    pub full_version: String,
    pub platform: String,
    pub platform_version: String,
    pub architecture: String,
    pub model: String,
    pub mobile: bool,
    pub bitness: String,
    pub wow64: bool,
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
    pub available: bool,
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
    pub available: bool,
}
/// ⚠️ Experimental
/// Enum of image types that can be disabled.
pub enum DisabledImageType {
    Avif,
    Webp,
}
/// Tells whether emulation is supported.
pub type EmulationCanEmulate = ();
/// Clears the overridden device metrics.
pub type EmulationClearDeviceMetricsOverride = ();
/// Clears the overridden Geolocation Position and Error.
pub type EmulationClearGeolocationOverride = ();
/// Requests that page scale factor is reset to initial values.
pub type EmulationResetPageScaleFactor = ();
/// Enables or disables simulating a focused and active page.
pub type EmulationSetFocusEmulationEnabled = ();
/// Automatically render all web contents using a dark theme.
pub type EmulationSetAutoDarkModeOverride = ();
/// Enables CPU throttling to emulate slow CPUs.
pub type EmulationSetCpuThrottlingRate = ();
/** Sets or clears an override of the default background color of the frame. This override is used
if the content does not specify one.*/
pub type EmulationSetDefaultBackgroundColorOverride = ();
/** Overrides the values for env(safe-area-inset-*) and env(safe-area-max-inset-*). Unset values will cause the
respective variables to be undefined, even if previously overridden.*/
pub type EmulationSetSafeAreaInsetsOverride = ();
/** Overrides the values of device screen dimensions (window.screen.width, window.screen.height,
window.innerWidth, window.innerHeight, and "device-width"/"device-height"-related CSS media
query results).*/
pub type EmulationSetDeviceMetricsOverride = ();
/** Start reporting the given posture value to the Device Posture API.
This override can also be set in setDeviceMetricsOverride().*/
pub type EmulationSetDevicePostureOverride = ();
/** Clears a device posture override set with either setDeviceMetricsOverride()
or setDevicePostureOverride() and starts using posture information from the
platform again.
Does nothing if no override is set.*/
pub type EmulationClearDevicePostureOverride = ();
/** Start using the given display features to pupulate the Viewport Segments API.
This override can also be set in setDeviceMetricsOverride().*/
pub type EmulationSetDisplayFeaturesOverride = ();
/** Clears the display features override set with either setDeviceMetricsOverride()
or setDisplayFeaturesOverride() and starts using display features from the
platform again.
Does nothing if no override is set.*/
pub type EmulationClearDisplayFeaturesOverride = ();
pub type EmulationSetScrollbarsHidden = ();
pub type EmulationSetDocumentCookieDisabled = ();
pub type EmulationSetEmitTouchEventsForMouse = ();
/// Emulates the given media type or media feature for CSS media queries.
pub type EmulationSetEmulatedMedia = ();
/// Emulates the given vision deficiency.
pub type EmulationSetEmulatedVisionDeficiency = ();
/** Overrides the Geolocation Position or Error. Omitting latitude, longitude or
accuracy emulates position unavailable.*/
pub type EmulationSetGeolocationOverride = ();
pub type EmulationGetOverriddenSensorInformation = ();
/** Overrides a platform sensor of a given type. If |enabled| is true, calls to
Sensor.start() will use a virtual sensor as backend rather than fetching
data from a real hardware sensor. Otherwise, existing virtual
sensor-backend Sensor objects will fire an error event and new calls to
Sensor.start() will attempt to use a real sensor instead.*/
pub type EmulationSetSensorOverrideEnabled = ();
/** Updates the sensor readings reported by a sensor type previously overridden
by setSensorOverrideEnabled.*/
pub type EmulationSetSensorOverrideReadings = ();
/** Overrides a pressure source of a given type, as used by the Compute
Pressure API, so that updates to PressureObserver.observe() are provided
via setPressureStateOverride instead of being retrieved from
platform-provided telemetry data.*/
pub type EmulationSetPressureSourceOverrideEnabled = ();
/** Provides a given pressure state that will be processed and eventually be
delivered to PressureObserver users. |source| must have been previously
overridden by setPressureSourceOverrideEnabled.*/
pub type EmulationSetPressureStateOverride = ();
/// Overrides the Idle state.
pub type EmulationSetIdleOverride = ();
/// Clears Idle state overrides.
pub type EmulationClearIdleOverride = ();
/// Overrides value returned by the javascript navigator object.
pub type EmulationSetNavigatorOverrides = ();
/// Sets a specified page scale factor.
pub type EmulationSetPageScaleFactor = ();
/// Switches script execution in the page.
pub type EmulationSetScriptExecutionDisabled = ();
/// Enables touch on platforms which do not support them.
pub type EmulationSetTouchEmulationEnabled = ();
/** Turns on virtual time for all frames (replacing real-time with a synthetic time source) and sets
the current virtual time policy.  Note this supersedes any previous time budget.*/
pub type EmulationSetVirtualTimePolicy = ();
/// Overrides default host system locale with the specified one.
pub type EmulationSetLocaleOverride = ();
/// Overrides default host system timezone with the specified one.
pub type EmulationSetTimezoneOverride = ();
/** Resizes the frame/viewport of the page. Note that this does not affect the frame's container
(e.g. browser window). Can be used to produce screenshots of the specified size. Not supported
on Android.*/
pub type EmulationSetVisibleSize = ();
pub type EmulationSetDisabledImageTypes = ();
pub type EmulationSetHardwareConcurrencyOverride = ();
/** Allows overriding user agent with the given string.
`userAgentMetadata` must be set for Client Hint headers to be sent.*/
pub type EmulationSetUserAgentOverride = ();
/// Allows overriding the automation flag.
pub type EmulationSetAutomationOverride = ();
/** Allows overriding the difference between the small and large viewport sizes, which determine the
value of the `svh` and `lvh` unit, respectively. Only supported for top-level frames.*/
pub type EmulationSetSmallViewportHeightDifferenceOverride = ();
