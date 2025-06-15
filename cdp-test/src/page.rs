use crate::common::*;
use crate::dom::*;
use crate::io::*;
use crate::network::*;
/// Unique frame identifier.
pub struct FrameId(String);
/// ⚠️ Experimental
/// Indicates whether a frame has been identified as an ad.
pub enum AdFrameType {
    None,
    Child,
    Root,
}
/// ⚠️ Experimental
pub enum AdFrameExplanation {
    ParentIsAd,
    CreatedByAdScript,
    MatchedBlockingRule,
}
/// ⚠️ Experimental
/// Indicates whether a frame has been identified as an ad and why.
pub struct AdFrameStatus {
    pub ad_frame_type: Box<AdFrameType>,
    pub explanations: Vec<AdFrameExplanation>,
}
/// ⚠️ Experimental
/** Identifies the bottom-most script which caused the frame to be labelled
as an ad.*/
pub struct AdScriptId {
    pub script_id: Box<()>,
    pub debugger_id: Box<()>,
}
/// ⚠️ Experimental
/// Indicates whether the frame is a secure context and why it is the case.
pub enum SecureContextType {
    Secure,
    SecureLocalhost,
    InsecureScheme,
    InsecureAncestor,
}
/// ⚠️ Experimental
/// Indicates whether the frame is cross-origin isolated and why it is the case.
pub enum CrossOriginIsolatedContextType {
    Isolated,
    NotIsolated,
    NotIsolatedFeatureDisabled,
}
/// ⚠️ Experimental
pub enum GatedApiFeatures {
    SharedArrayBuffers,
    SharedArrayBuffersTransferAllowed,
    PerformanceMeasureMemory,
    PerformanceProfile,
}
/// ⚠️ Experimental
/** All Permissions Policy features. This enum should match the one defined
in services/network/public/cpp/permissions_policy/permissions_policy_features.json5.
LINT.IfChange(PermissionsPolicyFeature)*/
pub enum PermissionsPolicyFeature {
    Accelerometer,
    AllScreensCapture,
    AmbientLightSensor,
    AttributionReporting,
    Autoplay,
    Bluetooth,
    BrowsingTopics,
    Camera,
    CapturedSurfaceControl,
    ChDpr,
    ChDeviceMemory,
    ChDownlink,
    ChEct,
    ChPrefersColorScheme,
    ChPrefersReducedMotion,
    ChPrefersReducedTransparency,
    ChRtt,
    ChSaveData,
    ChUa,
    ChUaArch,
    ChUaBitness,
    ChUaHighEntropyValues,
    ChUaPlatform,
    ChUaModel,
    ChUaMobile,
    ChUaFormFactors,
    ChUaFullVersion,
    ChUaFullVersionList,
    ChUaPlatformVersion,
    ChUaWow64,
    ChViewportHeight,
    ChViewportWidth,
    ChWidth,
    ClipboardRead,
    ClipboardWrite,
    ComputePressure,
    ControlledFrame,
    CrossOriginIsolated,
    DeferredFetch,
    DeferredFetchMinimal,
    DeviceAttributes,
    DigitalCredentialsGet,
    DirectSockets,
    DirectSocketsPrivate,
    DisplayCapture,
    DocumentDomain,
    EncryptedMedia,
    ExecutionWhileOutOfViewport,
    ExecutionWhileNotRendered,
    FencedUnpartitionedStorageRead,
    FocusWithoutUserActivation,
    Fullscreen,
    Frobulate,
    Gamepad,
    Geolocation,
    Gyroscope,
    Hid,
    IdentityCredentialsGet,
    IdleDetection,
    InterestCohort,
    JoinAdInterestGroup,
    KeyboardMap,
    LanguageDetector,
    LocalFonts,
    LocalNetworkAccess,
    Magnetometer,
    MediaPlaybackWhileNotVisible,
    Microphone,
    Midi,
    OtpCredentials,
    Payment,
    PictureInPicture,
    Popins,
    PrivateAggregation,
    PrivateStateTokenIssuance,
    PrivateStateTokenRedemption,
    PublickeyCredentialsCreate,
    PublickeyCredentialsGet,
    RecordAdAuctionEvents,
    Rewriter,
    RunAdAuction,
    ScreenWakeLock,
    Serial,
    SharedAutofill,
    SharedStorage,
    SharedStorageSelectUrl,
    SmartCard,
    SpeakerSelection,
    StorageAccess,
    SubApps,
    Summarizer,
    SyncXhr,
    Translator,
    Unload,
    Usb,
    UsbUnrestricted,
    VerticalScroll,
    WebAppInstallation,
    WebPrinting,
    WebShare,
    WindowManagement,
    Writer,
    XrSpatialTracking,
}
/// ⚠️ Experimental
/// Reason for a permissions policy feature to be disabled.
pub enum PermissionsPolicyBlockReason {
    Header,
    IframeAttribute,
    InFencedFrameTree,
    InIsolatedApp,
}
/// ⚠️ Experimental
pub struct PermissionsPolicyBlockLocator {
    pub frame_id: Box<FrameId>,
    pub block_reason: Box<PermissionsPolicyBlockReason>,
}
/// ⚠️ Experimental
pub struct PermissionsPolicyFeatureState {
    pub feature: Box<PermissionsPolicyFeature>,
    pub allowed: bool,
    pub locator: Box<PermissionsPolicyBlockLocator>,
}
/// ⚠️ Experimental
/** Origin Trial(https://www.chromium.org/blink/origin-trials) support.
Status for an Origin Trial token.*/
pub enum OriginTrialTokenStatus {
    Success,
    NotSupported,
    Insecure,
    Expired,
    WrongOrigin,
    InvalidSignature,
    Malformed,
    WrongVersion,
    FeatureDisabled,
    TokenDisabled,
    FeatureDisabledForUser,
    UnknownTrial,
}
/// ⚠️ Experimental
/// Status for an Origin Trial.
pub enum OriginTrialStatus {
    Enabled,
    ValidTokenNotProvided,
    OsNotSupported,
    TrialNotAllowed,
}
/// ⚠️ Experimental
pub enum OriginTrialUsageRestriction {
    None,
    Subset,
}
/// ⚠️ Experimental
pub struct OriginTrialToken {
    pub origin: String,
    pub match_sub_domains: bool,
    pub trial_name: String,
    pub expiry_time: Box<NetworkTimeSinceEpoch>,
    pub is_third_party: bool,
    pub usage_restriction: Box<OriginTrialUsageRestriction>,
}
/// ⚠️ Experimental
pub struct OriginTrialTokenWithStatus {
    pub raw_token_text: String,
    pub parsed_token: Box<OriginTrialToken>,
    pub status: Box<OriginTrialTokenStatus>,
}
/// ⚠️ Experimental
pub struct OriginTrial {
    pub trial_name: String,
    pub status: Box<OriginTrialStatus>,
    pub tokens_with_status: Vec<OriginTrialTokenWithStatus>,
}
/// ⚠️ Experimental
/// Additional information about the frame document's security origin.
pub struct SecurityOriginDetails {
    pub is_localhost: bool,
}
/// Information about the Frame on the page.
pub struct Frame {
    pub id: Box<FrameId>,
    pub parent_id: Box<FrameId>,
    pub loader_id: Box<LoaderId>,
    pub name: String,
    pub url: String,
    pub url_fragment: String,
    pub domain_and_registry: String,
    pub security_origin: String,
    pub security_origin_details: Box<SecurityOriginDetails>,
    pub mime_type: String,
    pub unreachable_url: String,
    pub ad_frame_status: Box<AdFrameStatus>,
    pub secure_context_type: Box<SecureContextType>,
    pub cross_origin_isolated_context_type: Box<CrossOriginIsolatedContextType>,
    pub gated_api_features: Vec<GatedApiFeatures>,
}
/// ⚠️ Experimental
/// Information about the Resource on the page.
pub struct FrameResource {
    pub url: String,
    pub _type: Box<ResourceType>,
    pub mime_type: String,
    pub last_modified: Box<NetworkTimeSinceEpoch>,
    pub content_size: u64,
    pub failed: bool,
    pub canceled: bool,
}
/// ⚠️ Experimental
/// Information about the Frame hierarchy along with their cached resources.
pub struct FrameResourceTree {
    pub frame: Box<Frame>,
    pub child_frames: Vec<FrameResourceTree>,
    pub resources: Vec<FrameResource>,
}
/// Information about the Frame hierarchy.
pub struct FrameTree {
    pub frame: Box<Frame>,
    pub child_frames: Vec<FrameTree>,
}
/// Unique script identifier.
pub struct ScriptIdentifier(String);
/// Transition type.
pub enum TransitionType {
    Link,
    Typed,
    AddressBar,
    AutoBookmark,
    AutoSubframe,
    ManualSubframe,
    Generated,
    AutoToplevel,
    FormSubmit,
    Reload,
    Keyword,
    KeywordGenerated,
    Other,
}
/// Navigation history entry.
pub struct NavigationEntry {
    pub id: i64,
    pub url: String,
    pub user_typed_url: String,
    pub title: String,
    pub transition_type: Box<TransitionType>,
}
/// ⚠️ Experimental
/// Screencast frame metadata.
pub struct ScreencastFrameMetadata {
    pub offset_top: u64,
    pub page_scale_factor: u64,
    pub device_width: u64,
    pub device_height: u64,
    pub scroll_offset_x: u64,
    pub scroll_offset_y: u64,
    pub timestamp: Box<NetworkTimeSinceEpoch>,
}
/// Error while paring app manifest.
pub struct AppManifestError {
    pub message: String,
    pub critical: i64,
    pub line: i64,
    pub column: i64,
}
/// ⚠️ Experimental
/// Parsed app manifest properties.
pub struct AppManifestParsedProperties {
    pub scope: String,
}
/// Layout viewport position and dimensions.
pub struct LayoutViewport {
    pub page_x: i64,
    pub page_y: i64,
    pub client_width: i64,
    pub client_height: i64,
}
/// Visual viewport position, dimensions, and scale.
pub struct VisualViewport {
    pub offset_x: u64,
    pub offset_y: u64,
    pub page_x: u64,
    pub page_y: u64,
    pub client_width: u64,
    pub client_height: u64,
    pub scale: u64,
    pub zoom: u64,
}
/// Viewport for capturing screenshot.
pub struct Viewport {
    pub x: u64,
    pub y: u64,
    pub width: u64,
    pub height: u64,
    pub scale: u64,
}
/// ⚠️ Experimental
/// Generic font families collection.
pub struct FontFamilies {
    pub standard: String,
    pub fixed: String,
    pub serif: String,
    pub sans_serif: String,
    pub cursive: String,
    pub fantasy: String,
    pub math: String,
}
/// ⚠️ Experimental
/// Font families collection for a script.
pub struct ScriptFontFamilies {
    pub script: String,
    pub font_families: Box<FontFamilies>,
}
/// ⚠️ Experimental
/// Default font sizes.
pub struct FontSizes {
    pub standard: i64,
    pub fixed: i64,
}
/// ⚠️ Experimental
pub enum ClientNavigationReason {
    AnchorClick,
    FormSubmissionGet,
    FormSubmissionPost,
    HttpHeaderRefresh,
    InitialFrameNavigation,
    MetaTagRefresh,
    Other,
    PageBlockInterstitial,
    Reload,
    ScriptInitiated,
}
/// ⚠️ Experimental
pub enum ClientNavigationDisposition {
    CurrentTab,
    NewTab,
    NewWindow,
    Download,
}
/// ⚠️ Experimental
pub struct InstallabilityErrorArgument {
    pub name: String,
    pub value: String,
}
/// ⚠️ Experimental
/// The installability error
pub struct InstallabilityError {
    pub error_id: String,
    pub error_arguments: Vec<InstallabilityErrorArgument>,
}
/// ⚠️ Experimental
/// The referring-policy used for the navigation.
pub enum ReferrerPolicy {
    NoReferrer,
    NoReferrerWhenDowngrade,
    Origin,
    OriginWhenCrossOrigin,
    SameOrigin,
    StrictOrigin,
    StrictOriginWhenCrossOrigin,
    UnsafeUrl,
}
/// ⚠️ Experimental
/// Per-script compilation cache parameters for `Page.produceCompilationCache`
pub struct CompilationCacheParams {
    pub url: String,
    pub eager: bool,
}
/// ⚠️ Experimental
pub struct FileFilter {
    pub name: String,
    pub accepts: Vec<String>,
}
/// ⚠️ Experimental
/// The image definition used in both icon and screenshot.
pub struct ImageResource {
    pub url: String,
    pub sizes: String,
    pub _type: String,
}
/// ⚠️ Experimental
pub struct LaunchHandler {
    pub client_mode: String,
}
/// ⚠️ Experimental
pub struct ProtocolHandler {
    pub protocol: String,
    pub url: String,
}
/// ⚠️ Experimental
pub struct RelatedApplication {
    pub id: String,
    pub url: String,
}
/// ⚠️ Experimental
pub struct ScopeExtension {
    pub origin: String,
    pub has_origin_wildcard: bool,
}
/// ⚠️ Experimental
pub struct Screenshot {
    pub image: Box<ImageResource>,
    pub form_factor: String,
    pub label: String,
}
/// ⚠️ Experimental
pub struct ShareTarget {
    pub action: String,
    pub method: String,
    pub enctype: String,
    pub title: String,
    pub text: String,
    pub url: String,
    pub files: Vec<FileFilter>,
}
/// ⚠️ Experimental
pub struct Shortcut {
    pub name: String,
    pub url: String,
}
/// ⚠️ Experimental
pub struct WebAppManifest {
    pub background_color: String,
    pub description: String,
    pub dir: String,
    pub display: String,
    pub display_overrides: Vec<String>,
    pub file_handlers: Vec<PageFileHandler>,
    pub icons: Vec<ImageResource>,
    pub id: String,
    pub lang: String,
    pub launch_handler: Box<LaunchHandler>,
    pub name: String,
    pub orientation: String,
    pub prefer_related_applications: bool,
    pub protocol_handlers: Vec<ProtocolHandler>,
    pub related_applications: Vec<RelatedApplication>,
    pub scope: String,
    pub scope_extensions: Vec<ScopeExtension>,
    pub screenshots: Vec<Screenshot>,
    pub share_target: Box<ShareTarget>,
    pub short_name: String,
    pub shortcuts: Vec<Shortcut>,
    pub start_url: String,
    pub theme_color: String,
}
/// ⚠️ Experimental
/// Enum of possible auto-response for permission / prompt dialogs.
pub enum AutoResponseMode {
    None,
    AutoAccept,
    AutoReject,
    AutoOptOut,
}
/// ⚠️ Experimental
/// The type of a frameNavigated event.
pub enum NavigationType {
    Navigation,
    BackForwardCacheRestore,
}
/// ⚠️ Experimental
/// List of not restored reasons for back-forward cache.
pub enum BackForwardCacheNotRestoredReason {
    NotPrimaryMainFrame,
    BackForwardCacheDisabled,
    RelatedActiveContentsExist,
    HttpStatusNotOk,
    SchemeNotHttpOrHttps,
    Loading,
    WasGrantedMediaAccess,
    DisableForRenderFrameHostCalled,
    DomainNotAllowed,
    HttpMethodNotGet,
    SubframeIsNavigating,
    Timeout,
    CacheLimit,
    JavaScriptExecution,
    RendererProcessKilled,
    RendererProcessCrashed,
    SchedulerTrackedFeatureUsed,
    ConflictingBrowsingInstance,
    CacheFlushed,
    ServiceWorkerVersionActivation,
    SessionRestored,
    ServiceWorkerPostMessage,
    EnteredBackForwardCacheBeforeServiceWorkerHostAdded,
    RenderFrameHostReusedSameSite,
    RenderFrameHostReusedCrossSite,
    ServiceWorkerClaim,
    IgnoreEventAndEvict,
    HaveInnerContents,
    TimeoutPuttingInCache,
    BackForwardCacheDisabledByLowMemory,
    BackForwardCacheDisabledByCommandLine,
    NetworkRequestDatapipeDrainedAsBytesConsumer,
    NetworkRequestRedirected,
    NetworkRequestTimeout,
    NetworkExceedsBufferLimit,
    NavigationCancelledWhileRestoring,
    NotMostRecentNavigationEntry,
    BackForwardCacheDisabledForPrerender,
    UserAgentOverrideDiffers,
    ForegroundCacheLimit,
    BrowsingInstanceNotSwapped,
    BackForwardCacheDisabledForDelegate,
    UnloadHandlerExistsInMainFrame,
    UnloadHandlerExistsInSubFrame,
    ServiceWorkerUnregistration,
    CacheControlNoStore,
    CacheControlNoStoreCookieModified,
    CacheControlNoStoreHttpOnlyCookieModified,
    NoResponseHead,
    Unknown,
    ActivationNavigationsDisallowedForBug1234857,
    ErrorDocument,
    FencedFramesEmbedder,
    CookieDisabled,
    HttpAuthRequired,
    CookieFlushed,
    BroadcastChannelOnMessage,
    WebViewSettingsChanged,
    WebViewJavaScriptObjectChanged,
    WebViewMessageListenerInjected,
    WebViewSafeBrowsingAllowlistChanged,
    WebViewDocumentStartJavascriptChanged,
    WebSocket,
    WebTransport,
    WebRtc,
    MainResourceHasCacheControlNoStore,
    MainResourceHasCacheControlNoCache,
    SubresourceHasCacheControlNoStore,
    SubresourceHasCacheControlNoCache,
    ContainsPlugins,
    DocumentLoaded,
    OutstandingNetworkRequestOthers,
    RequestedMidiPermission,
    RequestedAudioCapturePermission,
    RequestedVideoCapturePermission,
    RequestedBackForwardCacheBlockedSensors,
    RequestedBackgroundWorkPermission,
    BroadcastChannel,
    WebXr,
    SharedWorker,
    WebLocks,
    WebHid,
    WebShare,
    RequestedStorageAccessGrant,
    WebNfc,
    OutstandingNetworkRequestFetch,
    OutstandingNetworkRequestXhr,
    AppBanner,
    Printing,
    WebDatabase,
    PictureInPicture,
    SpeechRecognizer,
    IdleManager,
    PaymentManager,
    SpeechSynthesis,
    KeyboardLock,
    WebOtpService,
    OutstandingNetworkRequestDirectSocket,
    InjectedJavascript,
    InjectedStyleSheet,
    KeepaliveRequest,
    IndexedDbEvent,
    Dummy,
    JsNetworkRequestReceivedCacheControlNoStoreResource,
    WebRtcSticky,
    WebTransportSticky,
    WebSocketSticky,
    SmartCard,
    LiveMediaStreamTrack,
    UnloadHandler,
    ParserAborted,
    ContentSecurityHandler,
    ContentWebAuthenticationApi,
    ContentFileChooser,
    ContentSerial,
    ContentFileSystemAccess,
    ContentMediaDevicesDispatcherHost,
    ContentWebBluetooth,
    ContentWebUsb,
    ContentMediaSessionService,
    ContentScreenReader,
    ContentDiscarded,
    EmbedderPopupBlockerTabHelper,
    EmbedderSafeBrowsingTriggeredPopupBlocker,
    EmbedderSafeBrowsingThreatDetails,
    EmbedderAppBannerManager,
    EmbedderDomDistillerViewerSource,
    EmbedderDomDistillerSelfDeletingRequestDelegate,
    EmbedderOomInterventionTabHelper,
    EmbedderOfflinePage,
    EmbedderChromePasswordManagerClientBindCredentialManager,
    EmbedderPermissionRequestManager,
    EmbedderModalDialog,
    EmbedderExtensions,
    EmbedderExtensionMessaging,
    EmbedderExtensionMessagingForOpenPort,
    EmbedderExtensionSentMessageToCachedFrame,
    RequestedByWebViewClient,
    PostMessageByWebViewClient,
    CacheControlNoStoreDeviceBoundSessionTerminated,
    CacheLimitPrunedOnModerateMemoryPressure,
    CacheLimitPrunedOnCriticalMemoryPressure,
}
/// ⚠️ Experimental
/// Types of not restored reasons for back-forward cache.
pub enum BackForwardCacheNotRestoredReasonType {
    SupportPending,
    PageSupportNeeded,
    Circumstantial,
}
/// ⚠️ Experimental
pub struct BackForwardCacheBlockingDetails {
    pub url: String,
    pub function: String,
    pub line_number: i64,
    pub column_number: i64,
}
/// ⚠️ Experimental
pub struct BackForwardCacheNotRestoredExplanation {
    pub _type: Box<BackForwardCacheNotRestoredReasonType>,
    pub reason: Box<BackForwardCacheNotRestoredReason>,
    pub context: String,
    pub details: Vec<BackForwardCacheBlockingDetails>,
}
/// ⚠️ Experimental
pub struct BackForwardCacheNotRestoredExplanationTree {
    pub url: String,
    pub explanations: Vec<BackForwardCacheNotRestoredExplanation>,
    pub children: Vec<BackForwardCacheNotRestoredExplanationTree>,
}
#[deprecated]
/// ⚠️ Experimental
/// Deprecated, please use addScriptToEvaluateOnNewDocument instead.
pub struct PageAddScriptToEvaluateOnLoadParams {
    pub script_source: String,
}
#[deprecated]
/// ⚠️ Experimental
/// Deprecated, please use addScriptToEvaluateOnNewDocument instead.
pub type PageAddScriptToEvaluateOnLoadReturns = ();
/// Evaluates given script in every frame upon creation (before loading frame's scripts).
pub struct PageAddScriptToEvaluateOnNewDocumentParams {
    pub source: String,
    pub world_name: String,
    pub include_command_line_api: bool,
    pub run_immediately: bool,
}
/// Evaluates given script in every frame upon creation (before loading frame's scripts).
pub type PageAddScriptToEvaluateOnNewDocumentReturns = ();
/// Brings page to front (activates tab).
pub type PageBringToFrontParams = ();
/// Brings page to front (activates tab).
pub type PageBringToFrontReturns = ();
/// Capture page screenshot.
pub struct PageCaptureScreenshotParams {
    pub format: String,
    pub quality: i64,
    pub clip: Box<Viewport>,
    pub from_surface: bool,
    pub capture_beyond_viewport: bool,
    pub optimize_for_speed: bool,
}
/// Capture page screenshot.
pub type PageCaptureScreenshotReturns = ();
/// ⚠️ Experimental
/** Returns a snapshot of the page as a string. For MHTML format, the serialization includes
iframes, shadow DOM, external resources, and element-inline styles.*/
pub struct PageCaptureSnapshotParams {
    pub format: String,
}
/// ⚠️ Experimental
/** Returns a snapshot of the page as a string. For MHTML format, the serialization includes
iframes, shadow DOM, external resources, and element-inline styles.*/
pub type PageCaptureSnapshotReturns = ();
#[deprecated]
/// ⚠️ Experimental
/// Clears the overridden device metrics.
pub type PageClearDeviceMetricsOverrideParams = crate::emulation::EmulationClearDeviceMetricsOverrideParams;
#[deprecated]
/// ⚠️ Experimental
/// Clears the overridden device metrics.
pub type PageClearDeviceMetricsOverrideReturns = crate::emulation::EmulationClearDeviceMetricsOverrideReturns;
#[deprecated]
/// ⚠️ Experimental
/// Clears the overridden Device Orientation.
pub type PageClearDeviceOrientationOverrideParams = crate::device_orientation::DeviceOrientationClearDeviceOrientationOverrideParams;
#[deprecated]
/// ⚠️ Experimental
/// Clears the overridden Device Orientation.
pub type PageClearDeviceOrientationOverrideReturns = crate::device_orientation::DeviceOrientationClearDeviceOrientationOverrideReturns;
#[deprecated]
/// Clears the overridden Geolocation Position and Error.
pub type PageClearGeolocationOverrideParams = crate::emulation::EmulationClearGeolocationOverrideParams;
#[deprecated]
/// Clears the overridden Geolocation Position and Error.
pub type PageClearGeolocationOverrideReturns = crate::emulation::EmulationClearGeolocationOverrideReturns;
/// Creates an isolated world for the given frame.
pub struct PageCreateIsolatedWorldParams {
    pub frame_id: Box<FrameId>,
    pub world_name: String,
    pub grant_univeral_access: bool,
}
/// Creates an isolated world for the given frame.
pub type PageCreateIsolatedWorldReturns = ();
#[deprecated]
/// ⚠️ Experimental
/// Deletes browser cookie with given name, domain and path.
pub struct PageDeleteCookieParams {
    pub cookie_name: String,
    pub url: String,
}
#[deprecated]
/// ⚠️ Experimental
/// Deletes browser cookie with given name, domain and path.
pub type PageDeleteCookieReturns = ();
/// Disables page domain notifications.
pub type PageDisableParams = ();
/// Disables page domain notifications.
pub type PageDisableReturns = ();
/// Enables page domain notifications.
pub struct PageEnableParams {
    pub enable_file_chooser_opened_event: bool,
}
/// Enables page domain notifications.
pub type PageEnableReturns = ();
/** Gets the processed manifest for this current document.
  This API always waits for the manifest to be loaded.
  If manifestId is provided, and it does not match the manifest of the
    current document, this API errors out.
  If there is not a loaded page, this API errors out immediately.*/
pub struct PageGetAppManifestParams {
    pub manifest_id: String,
}
/** Gets the processed manifest for this current document.
  This API always waits for the manifest to be loaded.
  If manifestId is provided, and it does not match the manifest of the
    current document, this API errors out.
  If there is not a loaded page, this API errors out immediately.*/
pub type PageGetAppManifestReturns = ();
/// ⚠️ Experimental
pub type PageGetInstallabilityErrorsParams = ();
/// ⚠️ Experimental
pub type PageGetInstallabilityErrorsReturns = ();
#[deprecated]
/// ⚠️ Experimental
/// Deprecated because it's not guaranteed that the returned icon is in fact the one used for PWA installation.
pub type PageGetManifestIconsParams = ();
#[deprecated]
/// ⚠️ Experimental
/// Deprecated because it's not guaranteed that the returned icon is in fact the one used for PWA installation.
pub type PageGetManifestIconsReturns = ();
/// ⚠️ Experimental
/** Returns the unique (PWA) app id.
Only returns values if the feature flag 'WebAppEnableManifestId' is enabled*/
pub type PageGetAppIdParams = ();
/// ⚠️ Experimental
/** Returns the unique (PWA) app id.
Only returns values if the feature flag 'WebAppEnableManifestId' is enabled*/
pub type PageGetAppIdReturns = ();
/// ⚠️ Experimental
pub struct PageGetAdScriptAncestryIdsParams {
    pub frame_id: Box<FrameId>,
}
/// ⚠️ Experimental
pub type PageGetAdScriptAncestryIdsReturns = ();
/// Returns present frame tree structure.
pub type PageGetFrameTreeParams = ();
/// Returns present frame tree structure.
pub type PageGetFrameTreeReturns = ();
/// Returns metrics relating to the layouting of the page, such as viewport bounds/scale.
pub type PageGetLayoutMetricsParams = ();
/// Returns metrics relating to the layouting of the page, such as viewport bounds/scale.
pub type PageGetLayoutMetricsReturns = ();
/// Returns navigation history for the current page.
pub type PageGetNavigationHistoryParams = ();
/// Returns navigation history for the current page.
pub type PageGetNavigationHistoryReturns = ();
/// Resets navigation history for the current page.
pub type PageResetNavigationHistoryParams = ();
/// Resets navigation history for the current page.
pub type PageResetNavigationHistoryReturns = ();
/// ⚠️ Experimental
/// Returns content of the given resource.
pub struct PageGetResourceContentParams {
    pub frame_id: Box<FrameId>,
    pub url: String,
}
/// ⚠️ Experimental
/// Returns content of the given resource.
pub type PageGetResourceContentReturns = ();
/// ⚠️ Experimental
/// Returns present frame / resource tree structure.
pub type PageGetResourceTreeParams = ();
/// ⚠️ Experimental
/// Returns present frame / resource tree structure.
pub type PageGetResourceTreeReturns = ();
/// Accepts or dismisses a JavaScript initiated dialog (alert, confirm, prompt, or onbeforeunload).
pub struct PageHandleJavaScriptDialogParams {
    pub accept: bool,
    pub prompt_text: String,
}
/// Accepts or dismisses a JavaScript initiated dialog (alert, confirm, prompt, or onbeforeunload).
pub type PageHandleJavaScriptDialogReturns = ();
/// Navigates current page to the given URL.
pub struct PageNavigateParams {
    pub url: String,
    pub referrer: String,
    pub transition_type: Box<TransitionType>,
    pub frame_id: Box<FrameId>,
    pub referrer_policy: Box<ReferrerPolicy>,
}
/// Navigates current page to the given URL.
pub type PageNavigateReturns = ();
/// Navigates current page to the given history entry.
pub struct PageNavigateToHistoryEntryParams {
    pub entry_id: i64,
}
/// Navigates current page to the given history entry.
pub type PageNavigateToHistoryEntryReturns = ();
/// Print page as PDF.
pub struct PagePrintToPdfParams {
    pub landscape: bool,
    pub display_header_footer: bool,
    pub print_background: bool,
    pub scale: u64,
    pub paper_width: u64,
    pub paper_height: u64,
    pub margin_top: u64,
    pub margin_bottom: u64,
    pub margin_left: u64,
    pub margin_right: u64,
    pub page_ranges: String,
    pub header_template: String,
    pub footer_template: String,
    pub prefer_css_page_size: bool,
    pub transfer_mode: String,
    pub generate_tagged_pdf: bool,
    pub generate_document_outline: bool,
}
/// Print page as PDF.
pub type PagePrintToPdfReturns = ();
/// Reloads given page optionally ignoring the cache.
pub struct PageReloadParams {
    pub ignore_cache: bool,
    pub script_to_evaluate_on_load: String,
    pub loader_id: Box<LoaderId>,
}
/// Reloads given page optionally ignoring the cache.
pub type PageReloadReturns = ();
#[deprecated]
/// ⚠️ Experimental
/// Deprecated, please use removeScriptToEvaluateOnNewDocument instead.
pub struct PageRemoveScriptToEvaluateOnLoadParams {
    pub identifier: Box<ScriptIdentifier>,
}
#[deprecated]
/// ⚠️ Experimental
/// Deprecated, please use removeScriptToEvaluateOnNewDocument instead.
pub type PageRemoveScriptToEvaluateOnLoadReturns = ();
/// Removes given script from the list.
pub struct PageRemoveScriptToEvaluateOnNewDocumentParams {
    pub identifier: Box<ScriptIdentifier>,
}
/// Removes given script from the list.
pub type PageRemoveScriptToEvaluateOnNewDocumentReturns = ();
/// ⚠️ Experimental
/// Acknowledges that a screencast frame has been received by the frontend.
pub struct PageScreencastFrameAckParams {
    pub session_id: i64,
}
/// ⚠️ Experimental
/// Acknowledges that a screencast frame has been received by the frontend.
pub type PageScreencastFrameAckReturns = ();
/// ⚠️ Experimental
/// Searches for given string in resource content.
pub struct PageSearchInResourceParams {
    pub frame_id: Box<FrameId>,
    pub url: String,
    pub query: String,
    pub case_sensitive: bool,
    pub is_regex: bool,
}
/// ⚠️ Experimental
/// Searches for given string in resource content.
pub type PageSearchInResourceReturns = ();
/// ⚠️ Experimental
/// Enable Chrome's experimental ad filter on all sites.
pub struct PageSetAdBlockingEnabledParams {
    pub enabled: bool,
}
/// ⚠️ Experimental
/// Enable Chrome's experimental ad filter on all sites.
pub type PageSetAdBlockingEnabledReturns = ();
/// Enable page Content Security Policy by-passing.
pub struct PageSetBypassCspParams {
    pub enabled: bool,
}
/// Enable page Content Security Policy by-passing.
pub type PageSetBypassCspReturns = ();
/// ⚠️ Experimental
/// Get Permissions Policy state on given frame.
pub struct PageGetPermissionsPolicyStateParams {
    pub frame_id: Box<FrameId>,
}
/// ⚠️ Experimental
/// Get Permissions Policy state on given frame.
pub type PageGetPermissionsPolicyStateReturns = ();
/// ⚠️ Experimental
/// Get Origin Trials on given frame.
pub struct PageGetOriginTrialsParams {
    pub frame_id: Box<FrameId>,
}
/// ⚠️ Experimental
/// Get Origin Trials on given frame.
pub type PageGetOriginTrialsReturns = ();
#[deprecated]
/// ⚠️ Experimental
/** Overrides the values of device screen dimensions (window.screen.width, window.screen.height,
window.innerWidth, window.innerHeight, and "device-width"/"device-height"-related CSS media
query results).*/
pub struct PageSetDeviceMetricsOverrideParams {
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
    pub screen_orientation: Box<crate::emulation::ScreenOrientation>,
    pub viewport: Box<Viewport>,
}
#[deprecated]
/// ⚠️ Experimental
/** Overrides the values of device screen dimensions (window.screen.width, window.screen.height,
window.innerWidth, window.innerHeight, and "device-width"/"device-height"-related CSS media
query results).*/
pub type PageSetDeviceMetricsOverrideReturns = crate::emulation::EmulationSetDeviceMetricsOverrideReturns;
#[deprecated]
/// ⚠️ Experimental
/// Overrides the Device Orientation.
pub struct PageSetDeviceOrientationOverrideParams {
    pub alpha: u64,
    pub beta: u64,
    pub gamma: u64,
}
#[deprecated]
/// ⚠️ Experimental
/// Overrides the Device Orientation.
pub type PageSetDeviceOrientationOverrideReturns = crate::device_orientation::DeviceOrientationSetDeviceOrientationOverrideReturns;
/// ⚠️ Experimental
/// Set generic font families.
pub struct PageSetFontFamiliesParams {
    pub font_families: Box<FontFamilies>,
    pub for_scripts: Vec<ScriptFontFamilies>,
}
/// ⚠️ Experimental
/// Set generic font families.
pub type PageSetFontFamiliesReturns = ();
/// ⚠️ Experimental
/// Set default font sizes.
pub struct PageSetFontSizesParams {
    pub font_sizes: Box<FontSizes>,
}
/// ⚠️ Experimental
/// Set default font sizes.
pub type PageSetFontSizesReturns = ();
/// Sets given markup as the document's HTML.
pub struct PageSetDocumentContentParams {
    pub frame_id: Box<FrameId>,
    pub html: String,
}
/// Sets given markup as the document's HTML.
pub type PageSetDocumentContentReturns = ();
#[deprecated]
/// ⚠️ Experimental
/// Set the behavior when downloading a file.
pub struct PageSetDownloadBehaviorParams {
    pub behavior: String,
    pub download_path: String,
}
#[deprecated]
/// ⚠️ Experimental
/// Set the behavior when downloading a file.
pub type PageSetDownloadBehaviorReturns = ();
#[deprecated]
/** Overrides the Geolocation Position or Error. Omitting any of the parameters emulates position
unavailable.*/
pub struct PageSetGeolocationOverrideParams {
    pub latitude: u64,
    pub longitude: u64,
    pub accuracy: u64,
}
#[deprecated]
/** Overrides the Geolocation Position or Error. Omitting any of the parameters emulates position
unavailable.*/
pub type PageSetGeolocationOverrideReturns = crate::emulation::EmulationSetGeolocationOverrideReturns;
/// Controls whether page will emit lifecycle events.
pub struct PageSetLifecycleEventsEnabledParams {
    pub enabled: bool,
}
/// Controls whether page will emit lifecycle events.
pub type PageSetLifecycleEventsEnabledReturns = ();
#[deprecated]
/// ⚠️ Experimental
/// Toggles mouse event-based touch event emulation.
pub struct PageSetTouchEmulationEnabledParams {
    pub enabled: bool,
    pub configuration: String,
}
#[deprecated]
/// ⚠️ Experimental
/// Toggles mouse event-based touch event emulation.
pub type PageSetTouchEmulationEnabledReturns = crate::emulation::EmulationSetTouchEmulationEnabledReturns;
/// ⚠️ Experimental
/// Starts sending each frame using the `screencastFrame` event.
pub struct PageStartScreencastParams {
    pub format: String,
    pub quality: i64,
    pub max_width: i64,
    pub max_height: i64,
    pub every_nth_frame: i64,
}
/// ⚠️ Experimental
/// Starts sending each frame using the `screencastFrame` event.
pub type PageStartScreencastReturns = ();
/// Force the page stop all navigations and pending resource fetches.
pub type PageStopLoadingParams = ();
/// Force the page stop all navigations and pending resource fetches.
pub type PageStopLoadingReturns = ();
/// ⚠️ Experimental
/// Crashes renderer on the IO thread, generates minidumps.
pub type PageCrashParams = ();
/// ⚠️ Experimental
/// Crashes renderer on the IO thread, generates minidumps.
pub type PageCrashReturns = ();
/// Tries to close page, running its beforeunload hooks, if any.
pub type PageCloseParams = ();
/// Tries to close page, running its beforeunload hooks, if any.
pub type PageCloseReturns = ();
/// ⚠️ Experimental
#[doc = " Tries to update the web lifecycle state of the page.\nIt will transition the page to the given state according to:\nhttps://github.com/WICG/web-lifecycle/"]
pub struct PageSetWebLifecycleStateParams {
    pub state: String,
}
/// ⚠️ Experimental
#[doc = " Tries to update the web lifecycle state of the page.\nIt will transition the page to the given state according to:\nhttps://github.com/WICG/web-lifecycle/"]
pub type PageSetWebLifecycleStateReturns = ();
/// ⚠️ Experimental
/// Stops sending each frame in the `screencastFrame`.
pub type PageStopScreencastParams = ();
/// ⚠️ Experimental
/// Stops sending each frame in the `screencastFrame`.
pub type PageStopScreencastReturns = ();
/// ⚠️ Experimental
/** Requests backend to produce compilation cache for the specified scripts.
`scripts` are appended to the list of scripts for which the cache
would be produced. The list may be reset during page navigation.
When script with a matching URL is encountered, the cache is optionally
produced upon backend discretion, based on internal heuristics.
See also: `Page.compilationCacheProduced`.*/
pub struct PageProduceCompilationCacheParams {
    pub scripts: Vec<CompilationCacheParams>,
}
/// ⚠️ Experimental
/** Requests backend to produce compilation cache for the specified scripts.
`scripts` are appended to the list of scripts for which the cache
would be produced. The list may be reset during page navigation.
When script with a matching URL is encountered, the cache is optionally
produced upon backend discretion, based on internal heuristics.
See also: `Page.compilationCacheProduced`.*/
pub type PageProduceCompilationCacheReturns = ();
/// ⚠️ Experimental
/** Seeds compilation cache for given url. Compilation cache does not survive
cross-process navigation.*/
pub struct PageAddCompilationCacheParams {
    pub url: String,
    pub data: String,
}
/// ⚠️ Experimental
/** Seeds compilation cache for given url. Compilation cache does not survive
cross-process navigation.*/
pub type PageAddCompilationCacheReturns = ();
/// ⚠️ Experimental
/// Clears seeded compilation cache.
pub type PageClearCompilationCacheParams = ();
/// ⚠️ Experimental
/// Clears seeded compilation cache.
pub type PageClearCompilationCacheReturns = ();
/// ⚠️ Experimental
/** Sets the Secure Payment Confirmation transaction mode.
https://w3c.github.io/secure-payment-confirmation/#sctn-automation-set-spc-transaction-mode*/
pub struct PageSetSpcTransactionModeParams {
    pub mode: Box<AutoResponseMode>,
}
/// ⚠️ Experimental
/** Sets the Secure Payment Confirmation transaction mode.
https://w3c.github.io/secure-payment-confirmation/#sctn-automation-set-spc-transaction-mode*/
pub type PageSetSpcTransactionModeReturns = ();
/// ⚠️ Experimental
/** Extensions for Custom Handlers API:
https://html.spec.whatwg.org/multipage/system-state.html#rph-automation*/
pub struct PageSetRphRegistrationModeParams {
    pub mode: Box<AutoResponseMode>,
}
/// ⚠️ Experimental
/** Extensions for Custom Handlers API:
https://html.spec.whatwg.org/multipage/system-state.html#rph-automation*/
pub type PageSetRphRegistrationModeReturns = ();
/// ⚠️ Experimental
/// Generates a report for testing.
pub struct PageGenerateTestReportParams {
    pub message: String,
    pub group: String,
}
/// ⚠️ Experimental
/// Generates a report for testing.
pub type PageGenerateTestReportReturns = ();
/// ⚠️ Experimental
/// Pauses page execution. Can be resumed using generic Runtime.runIfWaitingForDebugger.
pub type PageWaitForDebuggerParams = ();
/// ⚠️ Experimental
/// Pauses page execution. Can be resumed using generic Runtime.runIfWaitingForDebugger.
pub type PageWaitForDebuggerReturns = ();
/** Intercept file chooser requests and transfer control to protocol clients.
When file chooser interception is enabled, native file chooser dialog is not shown.
Instead, a protocol event `Page.fileChooserOpened` is emitted.*/
pub struct PageSetInterceptFileChooserDialogParams {
    pub enabled: bool,
    pub cancel: bool,
}
/** Intercept file chooser requests and transfer control to protocol clients.
When file chooser interception is enabled, native file chooser dialog is not shown.
Instead, a protocol event `Page.fileChooserOpened` is emitted.*/
pub type PageSetInterceptFileChooserDialogReturns = ();
/// ⚠️ Experimental
/** Enable/disable prerendering manually.

This command is a short-term solution for https://crbug.com/1440085.
See https://docs.google.com/document/d/12HVmFxYj5Jc-eJr5OmWsa2bqTJsbgGLKI6ZIyx0_wpA
for more details.

TODO(https://crbug.com/1440085): Remove this once Puppeteer supports tab targets.*/
pub struct PageSetPrerenderingAllowedParams {
    pub is_allowed: bool,
}
/// ⚠️ Experimental
/** Enable/disable prerendering manually.

This command is a short-term solution for https://crbug.com/1440085.
See https://docs.google.com/document/d/12HVmFxYj5Jc-eJr5OmWsa2bqTJsbgGLKI6ZIyx0_wpA
for more details.

TODO(https://crbug.com/1440085): Remove this once Puppeteer supports tab targets.*/
pub type PageSetPrerenderingAllowedReturns = ();
