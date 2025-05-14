pub use crate::common::*;
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
    pub ad_frame_type: (),
    pub explanations: (),
}
/// ⚠️ Experimental
/** Identifies the bottom-most script which caused the frame to be labelled
as an ad.*/
pub struct AdScriptId {
    pub script_id: (),
    pub debugger_id: (),
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
    pub frame_id: (),
    pub block_reason: (),
}
/// ⚠️ Experimental
pub struct PermissionsPolicyFeatureState {
    pub feature: (),
    pub allowed: (),
    pub locator: (),
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
    pub match_sub_domains: (),
    pub trial_name: String,
    pub expiry_time: (),
    pub is_third_party: (),
    pub usage_restriction: (),
}
/// ⚠️ Experimental
pub struct OriginTrialTokenWithStatus {
    pub raw_token_text: String,
    pub parsed_token: (),
    pub status: (),
}
/// ⚠️ Experimental
pub struct OriginTrial {
    pub trial_name: String,
    pub status: (),
    pub tokens_with_status: (),
}
/// ⚠️ Experimental
/// Additional information about the frame document's security origin.
pub struct SecurityOriginDetails {
    pub is_localhost: (),
}
/// Information about the Frame on the page.
pub struct Frame {
    pub id: (),
    pub parent_id: (),
    pub loader_id: (),
    pub name: String,
    pub url: String,
    pub url_fragment: String,
    pub domain_and_registry: String,
    pub security_origin: String,
    pub security_origin_details: (),
    pub mime_type: String,
    pub unreachable_url: String,
    pub ad_frame_status: (),
    pub secure_context_type: (),
    pub cross_origin_isolated_context_type: (),
    pub gated_api_features: (),
}
/// ⚠️ Experimental
/// Information about the Resource on the page.
pub struct FrameResource {
    pub url: String,
    pub _type: (),
    pub mime_type: String,
    pub last_modified: (),
    pub content_size: u64,
    pub failed: (),
    pub canceled: (),
}
/// ⚠️ Experimental
/// Information about the Frame hierarchy along with their cached resources.
pub struct FrameResourceTree {
    pub frame: (),
    pub child_frames: (),
    pub resources: (),
}
/// Information about the Frame hierarchy.
pub struct FrameTree {
    pub frame: (),
    pub child_frames: (),
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
    pub transition_type: (),
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
    pub timestamp: (),
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
    pub font_families: (),
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
    pub error_arguments: (),
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
    pub eager: (),
}
/// ⚠️ Experimental
pub struct FileFilter {
    pub name: String,
    pub accepts: (),
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
    pub has_origin_wildcard: (),
}
/// ⚠️ Experimental
pub struct Screenshot {
    pub image: (),
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
    pub files: (),
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
    pub display_overrides: (),
    pub file_handlers: (),
    pub icons: (),
    pub id: String,
    pub lang: String,
    pub launch_handler: (),
    pub name: String,
    pub orientation: String,
    pub prefer_related_applications: (),
    pub protocol_handlers: (),
    pub related_applications: (),
    pub scope: String,
    pub scope_extensions: (),
    pub screenshots: (),
    pub share_target: (),
    pub short_name: String,
    pub shortcuts: (),
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
    pub _type: (),
    pub reason: (),
    pub context: String,
    pub details: (),
}
/// ⚠️ Experimental
pub struct BackForwardCacheNotRestoredExplanationTree {
    pub url: String,
    pub explanations: (),
    pub children: (),
}
