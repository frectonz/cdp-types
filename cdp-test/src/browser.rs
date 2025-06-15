use crate::common::*;
/// ⚠️ Experimental
pub struct BrowserContextId(String);
/// ⚠️ Experimental
pub struct WindowId(i64);
/// ⚠️ Experimental
/// Browser window bounds information
pub struct Bounds {
    pub left: i64,
    pub top: i64,
    pub width: i64,
    pub height: i64,
    pub window_state: Box<BrowserWindowState>,
}
/// ⚠️ Experimental
pub enum PermissionType {
    Ar,
    AudioCapture,
    AutomaticFullscreen,
    BackgroundFetch,
    BackgroundSync,
    CameraPanTiltZoom,
    CapturedSurfaceControl,
    ClipboardReadWrite,
    ClipboardSanitizedWrite,
    DisplayCapture,
    DurableStorage,
    Geolocation,
    HandTracking,
    IdleDetection,
    KeyboardLock,
    LocalFonts,
    LocalNetworkAccess,
    Midi,
    MidiSysex,
    Nfc,
    Notifications,
    PaymentHandler,
    PeriodicBackgroundSync,
    PointerLock,
    ProtectedMediaIdentifier,
    Sensors,
    SmartCard,
    SpeakerSelection,
    StorageAccess,
    TopLevelStorageAccess,
    VideoCapture,
    Vr,
    WakeLockScreen,
    WakeLockSystem,
    WebAppInstallation,
    WebPrinting,
    WindowManagement,
}
/// ⚠️ Experimental
pub enum PermissionSetting {
    Granted,
    Denied,
    Prompt,
}
/// ⚠️ Experimental
/** Definition of PermissionDescriptor defined in the Permissions API:
https://w3c.github.io/permissions/#dom-permissiondescriptor.*/
pub struct PermissionDescriptor {
    pub name: String,
    pub sysex: bool,
    pub user_visible_only: bool,
    pub allow_without_sanitization: bool,
    pub allow_without_gesture: bool,
    pub pan_tilt_zoom: bool,
}
/// ⚠️ Experimental
/// Browser command ids used by executeBrowserCommand.
pub enum BrowserCommandId {
    OpenTabSearch,
    CloseTabSearch,
    OpenGlic,
}
/// ⚠️ Experimental
/// Chrome histogram bucket.
pub struct Bucket {
    pub low: i64,
    pub high: i64,
    pub count: i64,
}
/// ⚠️ Experimental
/// Chrome histogram.
pub struct Histogram {
    pub name: String,
    pub sum: i64,
    pub count: i64,
    pub buckets: Vec<Bucket>,
}
/// ⚠️ Experimental
pub enum PrivacySandboxApi {
    BiddingAndAuctionServices,
    TrustedKeyValue,
}
/// ⚠️ Experimental
/// Set permission settings for given origin.
pub struct BrowserSetPermissionParams {
    pub permission: (),
    pub setting: (),
    pub origin: (),
    pub browser_context_id: (),
}
/// ⚠️ Experimental
/// Set permission settings for given origin.
pub type BrowserSetPermissionReturns = ();
/// ⚠️ Experimental
/// Grant specific permissions to the given origin and reject all others.
pub struct BrowserGrantPermissionsParams {
    pub permissions: (),
    pub origin: (),
    pub browser_context_id: (),
}
/// ⚠️ Experimental
/// Grant specific permissions to the given origin and reject all others.
pub type BrowserGrantPermissionsReturns = ();
/// Reset all permission management for all origins.
pub struct BrowserResetPermissionsParams {
    pub browser_context_id: (),
}
/// Reset all permission management for all origins.
pub type BrowserResetPermissionsReturns = ();
/// ⚠️ Experimental
/// Set the behavior when downloading a file.
pub struct BrowserSetDownloadBehaviorParams {
    pub behavior: (),
    pub browser_context_id: (),
    pub download_path: (),
    pub events_enabled: (),
}
/// ⚠️ Experimental
/// Set the behavior when downloading a file.
pub type BrowserSetDownloadBehaviorReturns = ();
/// ⚠️ Experimental
/// Cancel a download if in progress
pub struct BrowserCancelDownloadParams {
    pub guid: (),
    pub browser_context_id: (),
}
/// ⚠️ Experimental
/// Cancel a download if in progress
pub type BrowserCancelDownloadReturns = ();
/// Close browser gracefully.
pub type BrowserCloseParams = ();
/// Close browser gracefully.
pub type BrowserCloseReturns = ();
/// ⚠️ Experimental
/// Crashes browser on the main thread.
pub type BrowserCrashParams = ();
/// ⚠️ Experimental
/// Crashes browser on the main thread.
pub type BrowserCrashReturns = ();
/// ⚠️ Experimental
/// Crashes GPU process.
pub type BrowserCrashGpuProcessParams = ();
/// ⚠️ Experimental
/// Crashes GPU process.
pub type BrowserCrashGpuProcessReturns = ();
/// Returns version information.
pub type BrowserGetVersionParams = ();
/// Returns version information.
pub type BrowserGetVersionReturns = ();
/// ⚠️ Experimental
/** Returns the command line switches for the browser process if, and only if
--enable-automation is on the commandline.*/
pub type BrowserGetBrowserCommandLineParams = ();
/// ⚠️ Experimental
/** Returns the command line switches for the browser process if, and only if
--enable-automation is on the commandline.*/
pub type BrowserGetBrowserCommandLineReturns = ();
/// ⚠️ Experimental
/// Get Chrome histograms.
pub struct BrowserGetHistogramsParams {
    pub query: (),
    pub delta: (),
}
/// ⚠️ Experimental
/// Get Chrome histograms.
pub type BrowserGetHistogramsReturns = ();
/// ⚠️ Experimental
/// Get a Chrome histogram by name.
pub struct BrowserGetHistogramParams {
    pub name: (),
    pub delta: (),
}
/// ⚠️ Experimental
/// Get a Chrome histogram by name.
pub type BrowserGetHistogramReturns = ();
/// ⚠️ Experimental
/// Get position and size of the browser window.
pub struct BrowserGetWindowBoundsParams {
    pub window_id: (),
}
/// ⚠️ Experimental
/// Get position and size of the browser window.
pub type BrowserGetWindowBoundsReturns = ();
/// ⚠️ Experimental
/// Get the browser window that contains the devtools target.
pub struct BrowserGetWindowForTargetParams {
    pub target_id: (),
}
/// ⚠️ Experimental
/// Get the browser window that contains the devtools target.
pub type BrowserGetWindowForTargetReturns = ();
/// ⚠️ Experimental
/// Set position and/or size of the browser window.
pub struct BrowserSetWindowBoundsParams {
    pub window_id: (),
    pub bounds: (),
}
/// ⚠️ Experimental
/// Set position and/or size of the browser window.
pub type BrowserSetWindowBoundsReturns = ();
/// ⚠️ Experimental
/// Set dock tile details, platform-specific.
pub struct BrowserSetDockTileParams {
    pub badge_label: (),
    pub image: (),
}
/// ⚠️ Experimental
/// Set dock tile details, platform-specific.
pub type BrowserSetDockTileReturns = ();
/// ⚠️ Experimental
/// Invoke custom browser commands used by telemetry.
pub struct BrowserExecuteBrowserCommandParams {
    pub command_id: (),
}
/// ⚠️ Experimental
/// Invoke custom browser commands used by telemetry.
pub type BrowserExecuteBrowserCommandReturns = ();
/** Allows a site to use privacy sandbox features that require enrollment
without the site actually being enrolled. Only supported on page targets.*/
pub struct BrowserAddPrivacySandboxEnrollmentOverrideParams {
    pub url: (),
}
/** Allows a site to use privacy sandbox features that require enrollment
without the site actually being enrolled. Only supported on page targets.*/
pub type BrowserAddPrivacySandboxEnrollmentOverrideReturns = ();
/** Configures encryption keys used with a given privacy sandbox API to talk
to a trusted coordinator.  Since this is intended for test automation only,
coordinatorOrigin must be a .test domain. No existing coordinator
configuration for the origin may exist.*/
pub struct BrowserAddPrivacySandboxCoordinatorKeyConfigParams {
    pub api: (),
    pub coordinator_origin: (),
    pub key_config: (),
    pub browser_context_id: (),
}
/** Configures encryption keys used with a given privacy sandbox API to talk
to a trusted coordinator.  Since this is intended for test automation only,
coordinatorOrigin must be a .test domain. No existing coordinator
configuration for the origin may exist.*/
pub type BrowserAddPrivacySandboxCoordinatorKeyConfigReturns = ();
