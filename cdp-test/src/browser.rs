/// ⚠️ Experimental
/// <https://chromedevtools.github.io/devtools-protocol/tot/Browser/#type-BrowserContextID>
pub struct BrowserContextId(String);
/// ⚠️ Experimental
/// <https://chromedevtools.github.io/devtools-protocol/tot/Browser/#type-WindowID>
pub struct BrowserWindowId(i64);
/// ⚠️ Experimental
/// The state of the browser window.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Browser/#type-WindowState>
pub enum BrowserWindowState {
    Normal,
    Minimized,
    Maximized,
    Fullscreen,
}
/// ⚠️ Experimental
/// Browser window bounds information
/// <https://chromedevtools.github.io/devtools-protocol/tot/Browser/#type-Bounds>
pub struct BrowserBounds {
    pub left: (),
    pub top: (),
    pub width: (),
    pub height: (),
    pub window_state: (),
}
/// ⚠️ Experimental
/// <https://chromedevtools.github.io/devtools-protocol/tot/Browser/#type-PermissionType>
pub enum BrowserPermissionType {
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
/// <https://chromedevtools.github.io/devtools-protocol/tot/Browser/#type-PermissionSetting>
pub enum BrowserPermissionSetting {
    Granted,
    Denied,
    Prompt,
}
/// ⚠️ Experimental
/** Definition of PermissionDescriptor defined in the Permissions API:
https://w3c.github.io/permissions/#dom-permissiondescriptor.*/
/// <https://chromedevtools.github.io/devtools-protocol/tot/Browser/#type-PermissionDescriptor>
pub struct BrowserPermissionDescriptor {
    pub name: (),
    pub sysex: (),
    pub user_visible_only: (),
    pub allow_without_sanitization: (),
    pub allow_without_gesture: (),
    pub pan_tilt_zoom: (),
}
/// ⚠️ Experimental
/// Browser command ids used by executeBrowserCommand.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Browser/#type-BrowserCommandId>
pub enum BrowserCommandId {
    OpenTabSearch,
    CloseTabSearch,
    OpenGlic,
}
/// ⚠️ Experimental
/// Chrome histogram bucket.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Browser/#type-Bucket>
pub struct BrowserBucket {
    pub low: (),
    pub high: (),
    pub count: (),
}
/// ⚠️ Experimental
/// Chrome histogram.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Browser/#type-Histogram>
pub struct BrowserHistogram {
    pub name: (),
    pub sum: (),
    pub count: (),
    pub buckets: (),
}
/// ⚠️ Experimental
/// <https://chromedevtools.github.io/devtools-protocol/tot/Browser/#type-PrivacySandboxAPI>
pub enum BrowserPrivacySandboxApi {
    BiddingAndAuctionServices,
    TrustedKeyValue,
}
