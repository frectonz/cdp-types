use crate::dom::*;
use crate::io::*;
use crate::network::*;
/// Unique frame identifier.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-FrameId>
pub struct PageFrameId(String);
/// ⚠️ Experimental
/// Indicates whether a frame has been identified as an ad.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-AdFrameType>
pub enum PageAdFrameType {
    None,
    Child,
    Root,
}
/// ⚠️ Experimental
/// <https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-AdFrameExplanation>
pub enum PageAdFrameExplanation {
    ParentIsAd,
    CreatedByAdScript,
    MatchedBlockingRule,
}
/// ⚠️ Experimental
/// Indicates whether a frame has been identified as an ad and why.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-AdFrameStatus>
pub struct PageAdFrameStatus {
    pub ad_frame_type: (),
    pub explanations: (),
}
/// ⚠️ Experimental
/** Identifies the bottom-most script which caused the frame to be labelled
as an ad.*/
/// <https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-AdScriptId>
pub struct PageAdScriptId {
    pub script_id: (),
    pub debugger_id: (),
}
/// ⚠️ Experimental
/// Indicates whether the frame is a secure context and why it is the case.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-SecureContextType>
pub enum PageSecureContextType {
    Secure,
    SecureLocalhost,
    InsecureScheme,
    InsecureAncestor,
}
/// ⚠️ Experimental
/// Indicates whether the frame is cross-origin isolated and why it is the case.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-CrossOriginIsolatedContextType>
pub enum PageCrossOriginIsolatedContextType {
    Isolated,
    NotIsolated,
    NotIsolatedFeatureDisabled,
}
/// ⚠️ Experimental
/// <https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-GatedAPIFeatures>
pub enum PageGatedApiFeatures {
    SharedArrayBuffers,
    SharedArrayBuffersTransferAllowed,
    PerformanceMeasureMemory,
    PerformanceProfile,
}
/// ⚠️ Experimental
/** All Permissions Policy features. This enum should match the one defined
in services/network/public/cpp/permissions_policy/permissions_policy_features.json5.
LINT.IfChange(PermissionsPolicyFeature)*/
/// <https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-PermissionsPolicyFeature>
pub enum PagePermissionsPolicyFeature {
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
/// <https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-PermissionsPolicyBlockReason>
pub enum PagePermissionsPolicyBlockReason {
    Header,
    IframeAttribute,
    InFencedFrameTree,
    InIsolatedApp,
}
/// ⚠️ Experimental
/// <https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-PermissionsPolicyBlockLocator>
pub struct PagePermissionsPolicyBlockLocator {
    pub frame_id: (),
    pub block_reason: (),
}
/// ⚠️ Experimental
/// <https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-PermissionsPolicyFeatureState>
pub struct PagePermissionsPolicyFeatureState {
    pub feature: (),
    pub allowed: (),
    pub locator: (),
}
/// ⚠️ Experimental
/** Origin Trial(https://www.chromium.org/blink/origin-trials) support.
Status for an Origin Trial token.*/
/// <https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-OriginTrialTokenStatus>
pub enum PageOriginTrialTokenStatus {
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
/// <https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-OriginTrialStatus>
pub enum PageOriginTrialStatus {
    Enabled,
    ValidTokenNotProvided,
    OsNotSupported,
    TrialNotAllowed,
}
/// ⚠️ Experimental
/// <https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-OriginTrialUsageRestriction>
pub enum PageOriginTrialUsageRestriction {
    None,
    Subset,
}
/// ⚠️ Experimental
/// <https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-OriginTrialToken>
pub struct PageOriginTrialToken {
    pub origin: (),
    pub match_sub_domains: (),
    pub trial_name: (),
    pub expiry_time: (),
    pub is_third_party: (),
    pub usage_restriction: (),
}
/// ⚠️ Experimental
/// <https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-OriginTrialTokenWithStatus>
pub struct PageOriginTrialTokenWithStatus {
    pub raw_token_text: (),
    pub parsed_token: (),
    pub status: (),
}
/// ⚠️ Experimental
/// <https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-OriginTrial>
pub struct PageOriginTrial {
    pub trial_name: (),
    pub status: (),
    pub tokens_with_status: (),
}
/// ⚠️ Experimental
/// Additional information about the frame document's security origin.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-SecurityOriginDetails>
pub struct PageSecurityOriginDetails {
    pub is_localhost: (),
}
/// Information about the Frame on the page.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-Frame>
pub struct PageFrame {
    pub id: (),
    pub parent_id: (),
    pub loader_id: (),
    pub name: (),
    pub url: (),
    pub url_fragment: (),
    pub domain_and_registry: (),
    pub security_origin: (),
    pub security_origin_details: (),
    pub mime_type: (),
    pub unreachable_url: (),
    pub ad_frame_status: (),
    pub secure_context_type: (),
    pub cross_origin_isolated_context_type: (),
    pub gated_api_features: (),
}
/// ⚠️ Experimental
/// Information about the Resource on the page.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-FrameResource>
pub struct PageFrameResource {
    pub url: (),
    pub _type: (),
    pub mime_type: (),
    pub last_modified: (),
    pub content_size: (),
    pub failed: (),
    pub canceled: (),
}
/// ⚠️ Experimental
/// Information about the Frame hierarchy along with their cached resources.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-FrameResourceTree>
pub struct PageFrameResourceTree {
    pub frame: (),
    pub child_frames: (),
    pub resources: (),
}
/// Information about the Frame hierarchy.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-FrameTree>
pub struct PageFrameTree {
    pub frame: (),
    pub child_frames: (),
}
/// Unique script identifier.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-ScriptIdentifier>
pub struct PageScriptIdentifier(String);
/// Transition type.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-TransitionType>
pub enum PageTransitionType {
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
/// <https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-NavigationEntry>
pub struct PageNavigationEntry {
    pub id: (),
    pub url: (),
    pub user_typed_url: (),
    pub title: (),
    pub transition_type: (),
}
/// ⚠️ Experimental
/// Screencast frame metadata.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-ScreencastFrameMetadata>
pub struct PageScreencastFrameMetadata {
    pub offset_top: (),
    pub page_scale_factor: (),
    pub device_width: (),
    pub device_height: (),
    pub scroll_offset_x: (),
    pub scroll_offset_y: (),
    pub timestamp: (),
}
/// Javascript dialog type.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-DialogType>
pub enum PageDialogType {
    Alert,
    Confirm,
    Prompt,
    Beforeunload,
}
/// Error while paring app manifest.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-AppManifestError>
pub struct PageAppManifestError {
    pub message: (),
    pub critical: (),
    pub line: (),
    pub column: (),
}
/// ⚠️ Experimental
/// Parsed app manifest properties.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-AppManifestParsedProperties>
pub struct PageAppManifestParsedProperties {
    pub scope: (),
}
/// Layout viewport position and dimensions.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-LayoutViewport>
pub struct PageLayoutViewport {
    pub page_x: (),
    pub page_y: (),
    pub client_width: (),
    pub client_height: (),
}
/// Visual viewport position, dimensions, and scale.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-VisualViewport>
pub struct PageVisualViewport {
    pub offset_x: (),
    pub offset_y: (),
    pub page_x: (),
    pub page_y: (),
    pub client_width: (),
    pub client_height: (),
    pub scale: (),
    pub zoom: (),
}
/// Viewport for capturing screenshot.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-Viewport>
pub struct PageViewport {
    pub x: (),
    pub y: (),
    pub width: (),
    pub height: (),
    pub scale: (),
}
/// ⚠️ Experimental
/// Generic font families collection.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-FontFamilies>
pub struct PageFontFamilies {
    pub standard: (),
    pub fixed: (),
    pub serif: (),
    pub sans_serif: (),
    pub cursive: (),
    pub fantasy: (),
    pub math: (),
}
/// ⚠️ Experimental
/// Font families collection for a script.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-ScriptFontFamilies>
pub struct PageScriptFontFamilies {
    pub script: (),
    pub font_families: (),
}
/// ⚠️ Experimental
/// Default font sizes.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-FontSizes>
pub struct PageFontSizes {
    pub standard: (),
    pub fixed: (),
}
/// ⚠️ Experimental
/// <https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-ClientNavigationReason>
pub enum PageClientNavigationReason {
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
/// <https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-ClientNavigationDisposition>
pub enum PageClientNavigationDisposition {
    CurrentTab,
    NewTab,
    NewWindow,
    Download,
}
/// ⚠️ Experimental
/// <https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-InstallabilityErrorArgument>
pub struct PageInstallabilityErrorArgument {
    pub name: (),
    pub value: (),
}
/// ⚠️ Experimental
/// The installability error
/// <https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-InstallabilityError>
pub struct PageInstallabilityError {
    pub error_id: (),
    pub error_arguments: (),
}
/// ⚠️ Experimental
/// The referring-policy used for the navigation.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-ReferrerPolicy>
pub enum PageReferrerPolicy {
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
/// <https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-CompilationCacheParams>
pub struct PageCompilationCacheParams {
    pub url: (),
    pub eager: (),
}
/// ⚠️ Experimental
/// <https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-FileFilter>
pub struct PageFileFilter {
    pub name: (),
    pub accepts: (),
}
/// ⚠️ Experimental
/// <https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-FileHandler>
pub struct PageFileHandler {
    pub action: (),
    pub name: (),
    pub icons: (),
    pub accepts: (),
    pub launch_type: (),
}
/// ⚠️ Experimental
/// The image definition used in both icon and screenshot.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-ImageResource>
pub struct PageImageResource {
    pub url: (),
    pub sizes: (),
    pub _type: (),
}
/// ⚠️ Experimental
/// <https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-LaunchHandler>
pub struct PageLaunchHandler {
    pub client_mode: (),
}
/// ⚠️ Experimental
/// <https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-ProtocolHandler>
pub struct PageProtocolHandler {
    pub protocol: (),
    pub url: (),
}
/// ⚠️ Experimental
/// <https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-RelatedApplication>
pub struct PageRelatedApplication {
    pub id: (),
    pub url: (),
}
/// ⚠️ Experimental
/// <https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-ScopeExtension>
pub struct PageScopeExtension {
    pub origin: (),
    pub has_origin_wildcard: (),
}
/// ⚠️ Experimental
/// <https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-Screenshot>
pub struct PageScreenshot {
    pub image: (),
    pub form_factor: (),
    pub label: (),
}
/// ⚠️ Experimental
/// <https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-ShareTarget>
pub struct PageShareTarget {
    pub action: (),
    pub method: (),
    pub enctype: (),
    pub title: (),
    pub text: (),
    pub url: (),
    pub files: (),
}
/// ⚠️ Experimental
/// <https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-Shortcut>
pub struct PageShortcut {
    pub name: (),
    pub url: (),
}
/// ⚠️ Experimental
/// <https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-WebAppManifest>
pub struct PageWebAppManifest {
    pub background_color: (),
    pub description: (),
    pub dir: (),
    pub display: (),
    pub display_overrides: (),
    pub file_handlers: (),
    pub icons: (),
    pub id: (),
    pub lang: (),
    pub launch_handler: (),
    pub name: (),
    pub orientation: (),
    pub prefer_related_applications: (),
    pub protocol_handlers: (),
    pub related_applications: (),
    pub scope: (),
    pub scope_extensions: (),
    pub screenshots: (),
    pub share_target: (),
    pub short_name: (),
    pub shortcuts: (),
    pub start_url: (),
    pub theme_color: (),
}
/// ⚠️ Experimental
/// Enum of possible auto-response for permission / prompt dialogs.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-AutoResponseMode>
pub enum PageAutoResponseMode {
    None,
    AutoAccept,
    AutoReject,
    AutoOptOut,
}
/// ⚠️ Experimental
/// The type of a frameNavigated event.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-NavigationType>
pub enum PageNavigationType {
    Navigation,
    BackForwardCacheRestore,
}
/// ⚠️ Experimental
/// List of not restored reasons for back-forward cache.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-BackForwardCacheNotRestoredReason>
pub enum PageBackForwardCacheNotRestoredReason {
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
/// <https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-BackForwardCacheNotRestoredReasonType>
pub enum PageBackForwardCacheNotRestoredReasonType {
    SupportPending,
    PageSupportNeeded,
    Circumstantial,
}
/// ⚠️ Experimental
/// <https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-BackForwardCacheBlockingDetails>
pub struct PageBackForwardCacheBlockingDetails {
    pub url: (),
    pub function: (),
    pub line_number: (),
    pub column_number: (),
}
/// ⚠️ Experimental
/// <https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-BackForwardCacheNotRestoredExplanation>
pub struct PageBackForwardCacheNotRestoredExplanation {
    pub _type: (),
    pub reason: (),
    pub context: (),
    pub details: (),
}
/// ⚠️ Experimental
/// <https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-BackForwardCacheNotRestoredExplanationTree>
pub struct PageBackForwardCacheNotRestoredExplanationTree {
    pub url: (),
    pub explanations: (),
    pub children: (),
}
