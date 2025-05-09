/// Unique id
/// <https://chromedevtools.github.io/devtools-protocol/tot/Preload/#type-RuleSetId>
pub struct PreloadRuleSetId(String);
/// Corresponds to SpeculationRuleSet
/// <https://chromedevtools.github.io/devtools-protocol/tot/Preload/#type-RuleSet>
pub struct PreloadRuleSet {
    pub id: (),
    pub loader_id: (),
    pub source_text: String,
    pub backend_node_id: (),
    pub url: String,
    pub request_id: (),
    pub error_type: (),
    pub error_message: String,
}
/// <https://chromedevtools.github.io/devtools-protocol/tot/Preload/#type-RuleSetErrorType>
pub enum PreloadRuleSetErrorType {
    SourceIsNotJsonObject,
    InvalidRulesSkipped,
}
/** The type of preloading attempted. It corresponds to
mojom::SpeculationAction (although PrefetchWithSubresources is omitted as it
isn't being used by clients).*/
/// <https://chromedevtools.github.io/devtools-protocol/tot/Preload/#type-SpeculationAction>
pub enum PreloadSpeculationAction {
    Prefetch,
    Prerender,
}
/** Corresponds to mojom::SpeculationTargetHint.
See https://github.com/WICG/nav-speculation/blob/main/triggers.md#window-name-targeting-hints*/
/// <https://chromedevtools.github.io/devtools-protocol/tot/Preload/#type-SpeculationTargetHint>
pub enum PreloadSpeculationTargetHint {
    Blank,
    SELF,
}
/** A key that identifies a preloading attempt.

The url used is the url specified by the trigger (i.e. the initial URL), and
not the final url that is navigated to. For example, prerendering allows
same-origin main frame navigations during the attempt, but the attempt is
still keyed with the initial URL.*/
/// <https://chromedevtools.github.io/devtools-protocol/tot/Preload/#type-PreloadingAttemptKey>
pub struct PreloadingAttemptKey {
    pub loader_id: (),
    pub action: (),
    pub url: String,
    pub target_hint: (),
}
/** Lists sources for a preloading attempt, specifically the ids of rule sets
that had a speculation rule that triggered the attempt, and the
BackendNodeIds of <a href> or <area href> elements that triggered the
attempt (in the case of attempts triggered by a document rule). It is
possible for multiple rule sets and links to trigger a single attempt.*/
/// <https://chromedevtools.github.io/devtools-protocol/tot/Preload/#type-PreloadingAttemptSource>
pub struct PreloadingAttemptSource {
    pub key: (),
    pub rule_set_ids: (),
    pub node_ids: (),
}
/** Chrome manages different types of preloads together using a
concept of preloading pipeline. For example, if a site uses a
SpeculationRules for prerender, Chrome first starts a prefetch and
then upgrades it to prerender.

CDP events for them are emitted separately but they share
`PreloadPipelineId`.*/
/// <https://chromedevtools.github.io/devtools-protocol/tot/Preload/#type-PreloadPipelineId>
pub struct PreloadPipelineId(String);
/// List of FinalStatus reasons for Prerender2.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Preload/#type-PrerenderFinalStatus>
pub enum PreloadPrerenderFinalStatus {
    Activated,
    Destroyed,
    LowEndDevice,
    InvalidSchemeRedirect,
    InvalidSchemeNavigation,
    NavigationRequestBlockedByCsp,
    MainFrameNavigation,
    MojoBinderPolicy,
    RendererProcessCrashed,
    RendererProcessKilled,
    Download,
    TriggerDestroyed,
    NavigationNotCommitted,
    NavigationBadHttpStatus,
    ClientCertRequested,
    NavigationRequestNetworkError,
    CancelAllHostsForTesting,
    DidFailLoad,
    Stop,
    SslCertificateError,
    LoginAuthRequested,
    UaChangeRequiresReload,
    BlockedByClient,
    AudioOutputDeviceRequested,
    MixedContent,
    TriggerBackgrounded,
    MemoryLimitExceeded,
    DataSaverEnabled,
    TriggerUrlHasEffectiveUrl,
    ActivatedBeforeStarted,
    InactivePageRestriction,
    StartFailed,
    TimeoutBackgrounded,
    CrossSiteRedirectInInitialNavigation,
    CrossSiteNavigationInInitialNavigation,
    SameSiteCrossOriginRedirectNotOptInInInitialNavigation,
    SameSiteCrossOriginNavigationNotOptInInInitialNavigation,
    ActivationNavigationParameterMismatch,
    ActivatedInBackground,
    EmbedderHostDisallowed,
    ActivationNavigationDestroyedBeforeSuccess,
    TabClosedByUserGesture,
    TabClosedWithoutUserGesture,
    PrimaryMainFrameRendererProcessCrashed,
    PrimaryMainFrameRendererProcessKilled,
    ActivationFramePolicyNotCompatible,
    PreloadingDisabled,
    BatterySaverEnabled,
    ActivatedDuringMainFrameNavigation,
    PreloadingUnsupportedByWebContents,
    CrossSiteRedirectInMainFrameNavigation,
    CrossSiteNavigationInMainFrameNavigation,
    SameSiteCrossOriginRedirectNotOptInInMainFrameNavigation,
    SameSiteCrossOriginNavigationNotOptInInMainFrameNavigation,
    MemoryPressureOnTrigger,
    MemoryPressureAfterTriggered,
    PrerenderingDisabledByDevTools,
    SpeculationRuleRemoved,
    ActivatedWithAuxiliaryBrowsingContexts,
    MaxNumOfRunningEagerPrerendersExceeded,
    MaxNumOfRunningNonEagerPrerendersExceeded,
    MaxNumOfRunningEmbedderPrerendersExceeded,
    PrerenderingUrlHasEffectiveUrl,
    RedirectedPrerenderingUrlHasEffectiveUrl,
    ActivationUrlHasEffectiveUrl,
    JavaScriptInterfaceAdded,
    JavaScriptInterfaceRemoved,
    AllPrerenderingCanceled,
    WindowClosed,
    SlowNetwork,
    OtherPrerenderedPageActivated,
    V8OptimizerDisabled,
    PrerenderFailedDuringPrefetch,
    BrowsingDataRemoved,
}
/** Preloading status values, see also PreloadingTriggeringOutcome. This
status is shared by prefetchStatusUpdated and prerenderStatusUpdated.*/
/// <https://chromedevtools.github.io/devtools-protocol/tot/Preload/#type-PreloadingStatus>
pub enum PreloadingStatus {
    Pending,
    Running,
    Ready,
    Success,
    Failure,
    NotSupported,
}
/** TODO(https://crbug.com/1384419): revisit the list of PrefetchStatus and
filter out the ones that aren't necessary to the developers.*/
/// <https://chromedevtools.github.io/devtools-protocol/tot/Preload/#type-PrefetchStatus>
pub enum PreloadPrefetchStatus {
    PrefetchAllowed,
    PrefetchFailedIneligibleRedirect,
    PrefetchFailedInvalidRedirect,
    PrefetchFailedMimeNotSupported,
    PrefetchFailedNetError,
    PrefetchFailedNon2Xx,
    PrefetchEvictedAfterBrowsingDataRemoved,
    PrefetchEvictedAfterCandidateRemoved,
    PrefetchEvictedForNewerPrefetch,
    PrefetchHeldback,
    PrefetchIneligibleRetryAfter,
    PrefetchIsPrivacyDecoy,
    PrefetchIsStale,
    PrefetchNotEligibleBrowserContextOffTheRecord,
    PrefetchNotEligibleDataSaverEnabled,
    PrefetchNotEligibleExistingProxy,
    PrefetchNotEligibleHostIsNonUnique,
    PrefetchNotEligibleNonDefaultStoragePartition,
    PrefetchNotEligibleSameSiteCrossOriginPrefetchRequiredProxy,
    PrefetchNotEligibleSchemeIsNotHttps,
    PrefetchNotEligibleUserHasCookies,
    PrefetchNotEligibleUserHasServiceWorker,
    PrefetchNotEligibleUserHasServiceWorkerNoFetchHandler,
    PrefetchNotEligibleRedirectFromServiceWorker,
    PrefetchNotEligibleRedirectToServiceWorker,
    PrefetchNotEligibleBatterySaverEnabled,
    PrefetchNotEligiblePreloadingDisabled,
    PrefetchNotFinishedInTime,
    PrefetchNotStarted,
    PrefetchNotUsedCookiesChanged,
    PrefetchProxyNotAvailable,
    PrefetchResponseUsed,
    PrefetchSuccessfulButNotUsed,
    PrefetchNotUsedProbeFailed,
}
/// Information of headers to be displayed when the header mismatch occurred.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Preload/#type-PrerenderMismatchedHeaders>
pub struct PreloadPrerenderMismatchedHeaders {
    pub header_name: String,
    pub initial_value: String,
    pub activation_value: String,
}
