use crate::common::*;
use crate::network::*;
use crate::dom::*;
/// Unique id
pub struct RuleSetId(String);
/// Corresponds to SpeculationRuleSet
pub struct RuleSet {
    pub id: Box<RuleSetId>,
    pub loader_id: Box<LoaderId>,
    pub source_text: String,
    pub backend_node_id: Box<BackendNodeId>,
    pub url: String,
    pub request_id: Box<NetworkRequestId>,
    pub error_type: Box<RuleSetErrorType>,
    pub error_message: String,
}
pub enum RuleSetErrorType {
    SourceIsNotJsonObject,
    InvalidRulesSkipped,
}
/** The type of preloading attempted. It corresponds to
mojom::SpeculationAction (although PrefetchWithSubresources is omitted as it
isn't being used by clients).*/
pub enum SpeculationAction {
    Prefetch,
    Prerender,
}
/** Corresponds to mojom::SpeculationTargetHint.
See https://github.com/WICG/nav-speculation/blob/main/triggers.md#window-name-targeting-hints*/
pub enum SpeculationTargetHint {
    Blank,
    SELF,
}
/** A key that identifies a preloading attempt.

The url used is the url specified by the trigger (i.e. the initial URL), and
not the final url that is navigated to. For example, prerendering allows
same-origin main frame navigations during the attempt, but the attempt is
still keyed with the initial URL.*/
pub struct PreloadingAttemptKey {
    pub loader_id: Box<LoaderId>,
    pub action: Box<SpeculationAction>,
    pub url: String,
    pub target_hint: Box<SpeculationTargetHint>,
}
/** Lists sources for a preloading attempt, specifically the ids of rule sets
that had a speculation rule that triggered the attempt, and the
BackendNodeIds of <a href> or <area href> elements that triggered the
attempt (in the case of attempts triggered by a document rule). It is
possible for multiple rule sets and links to trigger a single attempt.*/
pub struct PreloadingAttemptSource {
    pub key: Box<PreloadingAttemptKey>,
    pub rule_set_ids: Vec<RuleSetId>,
    pub node_ids: Vec<BackendNodeId>,
}
/** Chrome manages different types of preloads together using a
concept of preloading pipeline. For example, if a site uses a
SpeculationRules for prerender, Chrome first starts a prefetch and
then upgrades it to prerender.

CDP events for them are emitted separately but they share
`PreloadPipelineId`.*/
pub struct PreloadPipelineId(String);
/// List of FinalStatus reasons for Prerender2.
pub enum PrerenderFinalStatus {
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
pub enum PrefetchStatus {
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
pub struct PrerenderMismatchedHeaders {
    pub header_name: String,
    pub initial_value: String,
    pub activation_value: String,
}
pub type PreloadEnableParams = ();
pub type PreloadEnableReturns = ();
pub type PreloadDisableParams = ();
pub type PreloadDisableReturns = ();
/// Upsert. Currently, it is only emitted when a rule set added.
pub struct PreloadRuleSetUpdatedEvent {
    pub rule_set: Box<RuleSet>,
}
pub struct PreloadRuleSetRemovedEvent {
    pub id: Box<RuleSetId>,
}
/// Fired when a preload enabled state is updated.
pub struct PreloadPreloadEnabledStateUpdatedEvent {
    pub disabled_by_preference: bool,
    pub disabled_by_data_saver: bool,
    pub disabled_by_battery_saver: bool,
    pub disabled_by_holdback_prefetch_speculation_rules: bool,
    pub disabled_by_holdback_prerender_speculation_rules: bool,
}
/// Fired when a prefetch attempt is updated.
pub struct PreloadPrefetchStatusUpdatedEvent {
    pub key: Box<PreloadingAttemptKey>,
    pub pipeline_id: Box<PreloadPipelineId>,
    pub initiating_frame_id: Box<crate::page::FrameId>,
    pub prefetch_url: String,
    pub status: Box<PreloadingStatus>,
    pub prefetch_status: Box<PrefetchStatus>,
    pub request_id: Box<NetworkRequestId>,
}
/// Fired when a prerender attempt is updated.
pub struct PreloadPrerenderStatusUpdatedEvent {
    pub key: Box<PreloadingAttemptKey>,
    pub pipeline_id: Box<PreloadPipelineId>,
    pub status: Box<PreloadingStatus>,
    pub prerender_status: Box<PrerenderFinalStatus>,
    pub disallowed_mojo_interface: String,
    pub mismatched_headers: Vec<PrerenderMismatchedHeaders>,
}
/// Send a list of sources for all preloading attempts in a document.
pub struct PreloadPreloadingAttemptSourcesUpdatedEvent {
    pub loader_id: Box<LoaderId>,
    pub preloading_attempt_sources: Vec<PreloadingAttemptSource>,
}
