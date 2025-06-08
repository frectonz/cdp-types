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
/// Set permission settings for given origin.
pub type BrowserSetPermission = ();
/// Grant specific permissions to the given origin and reject all others.
pub type BrowserGrantPermissions = ();
/// Reset all permission management for all origins.
pub type BrowserResetPermissions = ();
/// Set the behavior when downloading a file.
pub type BrowserSetDownloadBehavior = ();
/// Cancel a download if in progress
pub type BrowserCancelDownload = ();
/// Close browser gracefully.
pub type BrowserClose = ();
/// Crashes browser on the main thread.
pub type BrowserCrash = ();
/// Crashes GPU process.
pub type BrowserCrashGpuProcess = ();
/// Returns version information.
pub type BrowserGetVersion = ();
/** Returns the command line switches for the browser process if, and only if
--enable-automation is on the commandline.*/
pub type BrowserGetBrowserCommandLine = ();
/// Get Chrome histograms.
pub type BrowserGetHistograms = ();
/// Get a Chrome histogram by name.
pub type BrowserGetHistogram = ();
/// Get position and size of the browser window.
pub type BrowserGetWindowBounds = ();
/// Get the browser window that contains the devtools target.
pub type BrowserGetWindowForTarget = ();
/// Set position and/or size of the browser window.
pub type BrowserSetWindowBounds = ();
/// Set dock tile details, platform-specific.
pub type BrowserSetDockTile = ();
/// Invoke custom browser commands used by telemetry.
pub type BrowserExecuteBrowserCommand = ();
/** Allows a site to use privacy sandbox features that require enrollment
without the site actually being enrolled. Only supported on page targets.*/
pub type BrowserAddPrivacySandboxEnrollmentOverride = ();
/** Configures encryption keys used with a given privacy sandbox API to talk
to a trusted coordinator.  Since this is intended for test automation only,
coordinatorOrigin must be a .test domain. No existing coordinator
configuration for the origin may exist.*/
pub type BrowserAddPrivacySandboxCoordinatorKeyConfig = ();
