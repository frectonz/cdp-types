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
#[deprecated]
/// Tells whether emulation is supported.
pub type EmulationCanEmulateParams = ();
#[deprecated]
/// Tells whether emulation is supported.
pub struct EmulationCanEmulateParams {
    pub result: bool,
}
/// Clears the overridden device metrics.
pub type EmulationClearDeviceMetricsOverrideParams = ();
/// Clears the overridden device metrics.
pub type EmulationClearDeviceMetricsOverrideReturns = ();
/// Clears the overridden Geolocation Position and Error.
pub type EmulationClearGeolocationOverrideParams = ();
/// Clears the overridden Geolocation Position and Error.
pub type EmulationClearGeolocationOverrideReturns = ();
/// ⚠️ Experimental
/// Requests that page scale factor is reset to initial values.
pub type EmulationResetPageScaleFactorParams = ();
/// ⚠️ Experimental
/// Requests that page scale factor is reset to initial values.
pub type EmulationResetPageScaleFactorReturns = ();
/// ⚠️ Experimental
/// Enables or disables simulating a focused and active page.
pub struct EmulationSetFocusEmulationEnabledParams {
    pub enabled: bool,
}
/// ⚠️ Experimental
/// Enables or disables simulating a focused and active page.
pub type EmulationSetFocusEmulationEnabledReturns = ();
/// ⚠️ Experimental
/// Automatically render all web contents using a dark theme.
pub struct EmulationSetAutoDarkModeOverrideParams {
    pub enabled: bool,
}
/// ⚠️ Experimental
/// Automatically render all web contents using a dark theme.
pub type EmulationSetAutoDarkModeOverrideReturns = ();
/// Enables CPU throttling to emulate slow CPUs.
pub struct EmulationSetCpuThrottlingRateParams {
    pub rate: u64,
}
/// Enables CPU throttling to emulate slow CPUs.
pub type EmulationSetCpuThrottlingRateReturns = ();
/** Sets or clears an override of the default background color of the frame. This override is used
if the content does not specify one.*/
pub struct EmulationSetDefaultBackgroundColorOverrideParams {
    pub color: Box<Rgba>,
}
/** Sets or clears an override of the default background color of the frame. This override is used
if the content does not specify one.*/
pub type EmulationSetDefaultBackgroundColorOverrideReturns = ();
/// ⚠️ Experimental
/** Overrides the values for env(safe-area-inset-*) and env(safe-area-max-inset-*). Unset values will cause the
respective variables to be undefined, even if previously overridden.*/
pub struct EmulationSetSafeAreaInsetsOverrideParams {
    pub insets: Box<SafeAreaInsets>,
}
/// ⚠️ Experimental
/** Overrides the values for env(safe-area-inset-*) and env(safe-area-max-inset-*). Unset values will cause the
respective variables to be undefined, even if previously overridden.*/
pub type EmulationSetSafeAreaInsetsOverrideReturns = ();
/** Overrides the values of device screen dimensions (window.screen.width, window.screen.height,
window.innerWidth, window.innerHeight, and "device-width"/"device-height"-related CSS media
query results).*/
pub struct EmulationSetDeviceMetricsOverrideParams {
    pub width: i64,
    pub height: i64,
    pub device_scale_factor: u64,
    pub mobile: bool,
    pub scale: u64,
    pub screen_width: i64,
    pub screen_height: i64,
    pub position_x: i64,
    pub position_y: i64,
    pub dont_set_visible_size: bool,
    pub screen_orientation: Box<ScreenOrientation>,
    pub viewport: Box<Viewport>,
    pub display_feature: Box<DisplayFeature>,
    pub device_posture: Box<DevicePosture>,
}
/** Overrides the values of device screen dimensions (window.screen.width, window.screen.height,
window.innerWidth, window.innerHeight, and "device-width"/"device-height"-related CSS media
query results).*/
pub type EmulationSetDeviceMetricsOverrideReturns = ();
/// ⚠️ Experimental
/** Start reporting the given posture value to the Device Posture API.
This override can also be set in setDeviceMetricsOverride().*/
pub struct EmulationSetDevicePostureOverrideParams {
    pub posture: Box<DevicePosture>,
}
/// ⚠️ Experimental
/** Start reporting the given posture value to the Device Posture API.
This override can also be set in setDeviceMetricsOverride().*/
pub type EmulationSetDevicePostureOverrideReturns = ();
/// ⚠️ Experimental
/** Clears a device posture override set with either setDeviceMetricsOverride()
or setDevicePostureOverride() and starts using posture information from the
platform again.
Does nothing if no override is set.*/
pub type EmulationClearDevicePostureOverrideParams = ();
/// ⚠️ Experimental
/** Clears a device posture override set with either setDeviceMetricsOverride()
or setDevicePostureOverride() and starts using posture information from the
platform again.
Does nothing if no override is set.*/
pub type EmulationClearDevicePostureOverrideReturns = ();
/// ⚠️ Experimental
/** Start using the given display features to pupulate the Viewport Segments API.
This override can also be set in setDeviceMetricsOverride().*/
pub struct EmulationSetDisplayFeaturesOverrideParams {
    pub features: Vec<DisplayFeature>,
}
/// ⚠️ Experimental
/** Start using the given display features to pupulate the Viewport Segments API.
This override can also be set in setDeviceMetricsOverride().*/
pub type EmulationSetDisplayFeaturesOverrideReturns = ();
/// ⚠️ Experimental
/** Clears the display features override set with either setDeviceMetricsOverride()
or setDisplayFeaturesOverride() and starts using display features from the
platform again.
Does nothing if no override is set.*/
pub type EmulationClearDisplayFeaturesOverrideParams = ();
/// ⚠️ Experimental
/** Clears the display features override set with either setDeviceMetricsOverride()
or setDisplayFeaturesOverride() and starts using display features from the
platform again.
Does nothing if no override is set.*/
pub type EmulationClearDisplayFeaturesOverrideReturns = ();
/// ⚠️ Experimental
pub struct EmulationSetScrollbarsHiddenParams {
    pub hidden: bool,
}
/// ⚠️ Experimental
pub type EmulationSetScrollbarsHiddenReturns = ();
/// ⚠️ Experimental
pub struct EmulationSetDocumentCookieDisabledParams {
    pub disabled: bool,
}
/// ⚠️ Experimental
pub type EmulationSetDocumentCookieDisabledReturns = ();
/// ⚠️ Experimental
pub struct EmulationSetEmitTouchEventsForMouseParams {
    pub enabled: bool,
    pub configuration: String,
}
/// ⚠️ Experimental
pub type EmulationSetEmitTouchEventsForMouseReturns = ();
/// Emulates the given media type or media feature for CSS media queries.
pub struct EmulationSetEmulatedMediaParams {
    pub media: String,
    pub features: Vec<MediaFeature>,
}
/// Emulates the given media type or media feature for CSS media queries.
pub type EmulationSetEmulatedMediaReturns = ();
/// Emulates the given vision deficiency.
pub struct EmulationSetEmulatedVisionDeficiencyParams {
    pub _type: String,
}
/// Emulates the given vision deficiency.
pub type EmulationSetEmulatedVisionDeficiencyReturns = ();
/** Overrides the Geolocation Position or Error. Omitting latitude, longitude or
accuracy emulates position unavailable.*/
pub struct EmulationSetGeolocationOverrideParams {
    pub latitude: u64,
    pub longitude: u64,
    pub accuracy: u64,
    pub altitude: u64,
    pub altitude_accuracy: u64,
    pub heading: u64,
    pub speed: u64,
}
/** Overrides the Geolocation Position or Error. Omitting latitude, longitude or
accuracy emulates position unavailable.*/
pub type EmulationSetGeolocationOverrideReturns = ();
/// ⚠️ Experimental
pub struct EmulationGetOverriddenSensorInformationParams {
    pub _type: Box<SensorType>,
}
/// ⚠️ Experimental
pub struct EmulationGetOverriddenSensorInformationParams {
    pub requested_sampling_frequency: u64,
}
/// ⚠️ Experimental
/** Overrides a platform sensor of a given type. If |enabled| is true, calls to
Sensor.start() will use a virtual sensor as backend rather than fetching
data from a real hardware sensor. Otherwise, existing virtual
sensor-backend Sensor objects will fire an error event and new calls to
Sensor.start() will attempt to use a real sensor instead.*/
pub struct EmulationSetSensorOverrideEnabledParams {
    pub enabled: bool,
    pub _type: Box<SensorType>,
    pub metadata: Box<SensorMetadata>,
}
/// ⚠️ Experimental
/** Overrides a platform sensor of a given type. If |enabled| is true, calls to
Sensor.start() will use a virtual sensor as backend rather than fetching
data from a real hardware sensor. Otherwise, existing virtual
sensor-backend Sensor objects will fire an error event and new calls to
Sensor.start() will attempt to use a real sensor instead.*/
pub type EmulationSetSensorOverrideEnabledReturns = ();
/// ⚠️ Experimental
/** Updates the sensor readings reported by a sensor type previously overridden
by setSensorOverrideEnabled.*/
pub struct EmulationSetSensorOverrideReadingsParams {
    pub _type: Box<SensorType>,
    pub reading: Box<SensorReading>,
}
/// ⚠️ Experimental
/** Updates the sensor readings reported by a sensor type previously overridden
by setSensorOverrideEnabled.*/
pub type EmulationSetSensorOverrideReadingsReturns = ();
/// ⚠️ Experimental
/** Overrides a pressure source of a given type, as used by the Compute
Pressure API, so that updates to PressureObserver.observe() are provided
via setPressureStateOverride instead of being retrieved from
platform-provided telemetry data.*/
pub struct EmulationSetPressureSourceOverrideEnabledParams {
    pub enabled: bool,
    pub source: Box<PressureSource>,
    pub metadata: Box<PressureMetadata>,
}
/// ⚠️ Experimental
/** Overrides a pressure source of a given type, as used by the Compute
Pressure API, so that updates to PressureObserver.observe() are provided
via setPressureStateOverride instead of being retrieved from
platform-provided telemetry data.*/
pub type EmulationSetPressureSourceOverrideEnabledReturns = ();
/// ⚠️ Experimental
/** Provides a given pressure state that will be processed and eventually be
delivered to PressureObserver users. |source| must have been previously
overridden by setPressureSourceOverrideEnabled.*/
pub struct EmulationSetPressureStateOverrideParams {
    pub source: Box<PressureSource>,
    pub state: Box<PressureState>,
}
/// ⚠️ Experimental
/** Provides a given pressure state that will be processed and eventually be
delivered to PressureObserver users. |source| must have been previously
overridden by setPressureSourceOverrideEnabled.*/
pub type EmulationSetPressureStateOverrideReturns = ();
/// Overrides the Idle state.
pub struct EmulationSetIdleOverrideParams {
    pub is_user_active: bool,
    pub is_screen_unlocked: bool,
}
/// Overrides the Idle state.
pub type EmulationSetIdleOverrideReturns = ();
/// Clears Idle state overrides.
pub type EmulationClearIdleOverrideParams = ();
/// Clears Idle state overrides.
pub type EmulationClearIdleOverrideReturns = ();
#[deprecated]
/// ⚠️ Experimental
/// Overrides value returned by the javascript navigator object.
pub struct EmulationSetNavigatorOverridesParams {
    pub platform: String,
}
#[deprecated]
/// ⚠️ Experimental
/// Overrides value returned by the javascript navigator object.
pub type EmulationSetNavigatorOverridesReturns = ();
/// ⚠️ Experimental
/// Sets a specified page scale factor.
pub struct EmulationSetPageScaleFactorParams {
    pub page_scale_factor: u64,
}
/// ⚠️ Experimental
/// Sets a specified page scale factor.
pub type EmulationSetPageScaleFactorReturns = ();
/// Switches script execution in the page.
pub struct EmulationSetScriptExecutionDisabledParams {
    pub value: bool,
}
/// Switches script execution in the page.
pub type EmulationSetScriptExecutionDisabledReturns = ();
/// Enables touch on platforms which do not support them.
pub struct EmulationSetTouchEmulationEnabledParams {
    pub enabled: bool,
    pub max_touch_points: i64,
}
/// Enables touch on platforms which do not support them.
pub type EmulationSetTouchEmulationEnabledReturns = ();
/// ⚠️ Experimental
/** Turns on virtual time for all frames (replacing real-time with a synthetic time source) and sets
the current virtual time policy.  Note this supersedes any previous time budget.*/
pub struct EmulationSetVirtualTimePolicyParams {
    pub policy: Box<VirtualTimePolicy>,
    pub budget: u64,
    pub max_virtual_time_task_starvation_count: i64,
    pub initial_virtual_time: Box<NetworkTimeSinceEpoch>,
}
/// ⚠️ Experimental
/** Turns on virtual time for all frames (replacing real-time with a synthetic time source) and sets
the current virtual time policy.  Note this supersedes any previous time budget.*/
pub struct EmulationSetVirtualTimePolicyParams {
    pub virtual_time_ticks_base: u64,
}
/// ⚠️ Experimental
/// Overrides default host system locale with the specified one.
pub struct EmulationSetLocaleOverrideParams {
    pub locale: String,
}
/// ⚠️ Experimental
/// Overrides default host system locale with the specified one.
pub type EmulationSetLocaleOverrideReturns = ();
/// Overrides default host system timezone with the specified one.
pub struct EmulationSetTimezoneOverrideParams {
    pub timezone_id: String,
}
/// Overrides default host system timezone with the specified one.
pub type EmulationSetTimezoneOverrideReturns = ();
#[deprecated]
/// ⚠️ Experimental
/** Resizes the frame/viewport of the page. Note that this does not affect the frame's container
(e.g. browser window). Can be used to produce screenshots of the specified size. Not supported
on Android.*/
pub struct EmulationSetVisibleSizeParams {
    pub width: i64,
    pub height: i64,
}
#[deprecated]
/// ⚠️ Experimental
/** Resizes the frame/viewport of the page. Note that this does not affect the frame's container
(e.g. browser window). Can be used to produce screenshots of the specified size. Not supported
on Android.*/
pub type EmulationSetVisibleSizeReturns = ();
/// ⚠️ Experimental
pub struct EmulationSetDisabledImageTypesParams {
    pub image_types: Vec<DisabledImageType>,
}
/// ⚠️ Experimental
pub type EmulationSetDisabledImageTypesReturns = ();
/// ⚠️ Experimental
pub struct EmulationSetHardwareConcurrencyOverrideParams {
    pub hardware_concurrency: i64,
}
/// ⚠️ Experimental
pub type EmulationSetHardwareConcurrencyOverrideReturns = ();
/** Allows overriding user agent with the given string.
`userAgentMetadata` must be set for Client Hint headers to be sent.*/
pub struct EmulationSetUserAgentOverrideParams {
    pub user_agent: String,
    pub accept_language: String,
    pub platform: String,
    pub user_agent_metadata: Box<UserAgentMetadata>,
}
/** Allows overriding user agent with the given string.
`userAgentMetadata` must be set for Client Hint headers to be sent.*/
pub type EmulationSetUserAgentOverrideReturns = ();
/// ⚠️ Experimental
/// Allows overriding the automation flag.
pub struct EmulationSetAutomationOverrideParams {
    pub enabled: bool,
}
/// ⚠️ Experimental
/// Allows overriding the automation flag.
pub type EmulationSetAutomationOverrideReturns = ();
/// ⚠️ Experimental
/** Allows overriding the difference between the small and large viewport sizes, which determine the
value of the `svh` and `lvh` unit, respectively. Only supported for top-level frames.*/
pub struct EmulationSetSmallViewportHeightDifferenceOverrideParams {
    pub difference: i64,
}
/// ⚠️ Experimental
/** Allows overriding the difference between the small and large viewport sizes, which determine the
value of the `svh` and `lvh` unit, respectively. Only supported for top-level frames.*/
pub type EmulationSetSmallViewportHeightDifferenceOverrideReturns = ();
