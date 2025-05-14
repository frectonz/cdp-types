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
    pub explanations: (),
}
/// ⚠️ Experimental
/** Identifies the bottom-most script which caused the frame to be labelled
as an ad.*/
pub struct AdScriptId {
    pub script_id: Box<RuntimeScriptId>,
    pub debugger_id: Box<RuntimeUniqueDebuggerId>,
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
    pub allowed: (),
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
    pub origin: Box<String>,
    pub match_sub_domains: (),
    pub trial_name: Box<String>,
    pub expiry_time: Box<NetworkTimeSinceEpoch>,
    pub is_third_party: (),
    pub usage_restriction: Box<OriginTrialUsageRestriction>,
}
/// ⚠️ Experimental
pub struct OriginTrialTokenWithStatus {
    pub raw_token_text: Box<String>,
    pub parsed_token: Box<OriginTrialToken>,
    pub status: Box<OriginTrialTokenStatus>,
}
/// ⚠️ Experimental
pub struct OriginTrial {
    pub trial_name: Box<String>,
    pub status: Box<OriginTrialStatus>,
    pub tokens_with_status: (),
}
/// ⚠️ Experimental
/// Additional information about the frame document's security origin.
pub struct SecurityOriginDetails {
    pub is_localhost: (),
}
/// Information about the Frame on the page.
pub struct Frame {
    pub id: Box<FrameId>,
    pub parent_id: Box<FrameId>,
    pub loader_id: Box<NetworkLoaderId>,
    pub name: Box<String>,
    pub url: Box<String>,
    pub url_fragment: Box<String>,
    pub domain_and_registry: Box<String>,
    pub security_origin: Box<String>,
    pub security_origin_details: Box<SecurityOriginDetails>,
    pub mime_type: Box<String>,
    pub unreachable_url: Box<String>,
    pub ad_frame_status: Box<AdFrameStatus>,
    pub secure_context_type: Box<SecureContextType>,
    pub cross_origin_isolated_context_type: Box<CrossOriginIsolatedContextType>,
    pub gated_api_features: (),
}
/// ⚠️ Experimental
/// Information about the Resource on the page.
pub struct FrameResource {
    pub url: Box<String>,
    pub _type: Box<NetworkResourceType>,
    pub mime_type: Box<String>,
    pub last_modified: Box<NetworkTimeSinceEpoch>,
    pub content_size: Box<u64>,
    pub failed: (),
    pub canceled: (),
}
/// ⚠️ Experimental
/// Information about the Frame hierarchy along with their cached resources.
pub struct FrameResourceTree {
    pub frame: Box<Frame>,
    pub child_frames: (),
    pub resources: (),
}
/// Information about the Frame hierarchy.
pub struct FrameTree {
    pub frame: Box<Frame>,
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
    pub id: Box<i64>,
    pub url: Box<String>,
    pub user_typed_url: Box<String>,
    pub title: Box<String>,
    pub transition_type: Box<TransitionType>,
}
/// ⚠️ Experimental
/// Screencast frame metadata.
pub struct ScreencastFrameMetadata {
    pub offset_top: Box<u64>,
    pub page_scale_factor: Box<u64>,
    pub device_width: Box<u64>,
    pub device_height: Box<u64>,
    pub scroll_offset_x: Box<u64>,
    pub scroll_offset_y: Box<u64>,
    pub timestamp: Box<NetworkTimeSinceEpoch>,
}
/// Error while paring app manifest.
pub struct AppManifestError {
    pub message: Box<String>,
    pub critical: Box<i64>,
    pub line: Box<i64>,
    pub column: Box<i64>,
}
/// ⚠️ Experimental
/// Parsed app manifest properties.
pub struct AppManifestParsedProperties {
    pub scope: Box<String>,
}
/// Layout viewport position and dimensions.
pub struct LayoutViewport {
    pub page_x: Box<i64>,
    pub page_y: Box<i64>,
    pub client_width: Box<i64>,
    pub client_height: Box<i64>,
}
/// Visual viewport position, dimensions, and scale.
pub struct VisualViewport {
    pub offset_x: Box<u64>,
    pub offset_y: Box<u64>,
    pub page_x: Box<u64>,
    pub page_y: Box<u64>,
    pub client_width: Box<u64>,
    pub client_height: Box<u64>,
    pub scale: Box<u64>,
    pub zoom: Box<u64>,
}
/// Viewport for capturing screenshot.
pub struct Viewport {
    pub x: Box<u64>,
    pub y: Box<u64>,
    pub width: Box<u64>,
    pub height: Box<u64>,
    pub scale: Box<u64>,
}
/// ⚠️ Experimental
/// Generic font families collection.
pub struct FontFamilies {
    pub standard: Box<String>,
    pub fixed: Box<String>,
    pub serif: Box<String>,
    pub sans_serif: Box<String>,
    pub cursive: Box<String>,
    pub fantasy: Box<String>,
    pub math: Box<String>,
}
/// ⚠️ Experimental
/// Font families collection for a script.
pub struct ScriptFontFamilies {
    pub script: Box<String>,
    pub font_families: Box<FontFamilies>,
}
/// ⚠️ Experimental
/// Default font sizes.
pub struct FontSizes {
    pub standard: Box<i64>,
    pub fixed: Box<i64>,
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
    pub name: Box<String>,
    pub value: Box<String>,
}
/// ⚠️ Experimental
/// The installability error
pub struct InstallabilityError {
    pub error_id: Box<String>,
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
    pub url: Box<String>,
    pub eager: (),
}
/// ⚠️ Experimental
pub struct FileFilter {
    pub name: Box<String>,
    pub accepts: (),
}
/// ⚠️ Experimental
/// The image definition used in both icon and screenshot.
pub struct ImageResource {
    pub url: Box<String>,
    pub sizes: Box<String>,
    pub _type: Box<String>,
}
/// ⚠️ Experimental
pub struct LaunchHandler {
    pub client_mode: Box<String>,
}
/// ⚠️ Experimental
pub struct ProtocolHandler {
    pub protocol: Box<String>,
    pub url: Box<String>,
}
/// ⚠️ Experimental
pub struct RelatedApplication {
    pub id: Box<String>,
    pub url: Box<String>,
}
/// ⚠️ Experimental
pub struct ScopeExtension {
    pub origin: Box<String>,
    pub has_origin_wildcard: (),
}
/// ⚠️ Experimental
pub struct Screenshot {
    pub image: Box<ImageResource>,
    pub form_factor: Box<String>,
    pub label: Box<String>,
}
/// ⚠️ Experimental
pub struct ShareTarget {
    pub action: Box<String>,
    pub method: Box<String>,
    pub enctype: Box<String>,
    pub title: Box<String>,
    pub text: Box<String>,
    pub url: Box<String>,
    pub files: (),
}
/// ⚠️ Experimental
pub struct Shortcut {
    pub name: Box<String>,
    pub url: Box<String>,
}
/// ⚠️ Experimental
pub struct WebAppManifest {
    pub background_color: Box<String>,
    pub description: Box<String>,
    pub dir: Box<String>,
    pub display: Box<String>,
    pub display_overrides: (),
    pub file_handlers: (),
    pub icons: (),
    pub id: Box<String>,
    pub lang: Box<String>,
    pub launch_handler: Box<LaunchHandler>,
    pub name: Box<String>,
    pub orientation: Box<String>,
    pub prefer_related_applications: (),
    pub protocol_handlers: (),
    pub related_applications: (),
    pub scope: Box<String>,
    pub scope_extensions: (),
    pub screenshots: (),
    pub share_target: Box<ShareTarget>,
    pub short_name: Box<String>,
    pub shortcuts: (),
    pub start_url: Box<String>,
    pub theme_color: Box<String>,
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
    pub url: Box<String>,
    pub function: Box<String>,
    pub line_number: Box<i64>,
    pub column_number: Box<i64>,
}
/// ⚠️ Experimental
pub struct BackForwardCacheNotRestoredExplanation {
    pub _type: Box<BackForwardCacheNotRestoredReasonType>,
    pub reason: Box<BackForwardCacheNotRestoredReason>,
    pub context: Box<String>,
    pub details: (),
}
/// ⚠️ Experimental
pub struct BackForwardCacheNotRestoredExplanationTree {
    pub url: Box<String>,
    pub explanations: (),
    pub children: (),
}
