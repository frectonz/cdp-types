use crate::browser::*;
use crate::network::*;
/// <https://chromedevtools.github.io/devtools-protocol/tot/Storage/#type-SerializedStorageKey>
pub struct StorageSerializedStorageKey(String);
/// Enum of possible storage types.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Storage/#type-StorageType>
pub enum StorageType {
    Cookies,
    FileSystems,
    Indexeddb,
    LocalStorage,
    ShaderCache,
    Websql,
    ServiceWorkers,
    CacheStorage,
    InterestGroups,
    SharedStorage,
    StorageBuckets,
    All,
    Other,
}
/// Usage for a storage type.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Storage/#type-UsageForType>
pub struct StorageUsageForType {
    pub storage_type: (),
    pub usage: u64,
}
/// ⚠️ Experimental
/** Pair of issuer origin and number of available (signed, but not used) Trust
Tokens from that issuer.*/
/// <https://chromedevtools.github.io/devtools-protocol/tot/Storage/#type-TrustTokens>
pub struct StorageTrustTokens {
    pub issuer_origin: String,
    pub count: u64,
}
/// Protected audience interest group auction identifier.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Storage/#type-InterestGroupAuctionId>
pub struct StorageInterestGroupAuctionId(String);
/// Enum of interest group access types.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Storage/#type-InterestGroupAccessType>
pub enum StorageInterestGroupAccessType {
    Join,
    Leave,
    Update,
    Loaded,
    Bid,
    Win,
    AdditionalBid,
    AdditionalBidWin,
    TopLevelBid,
    TopLevelAdditionalBid,
    Clear,
}
/// Enum of auction events.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Storage/#type-InterestGroupAuctionEventType>
pub enum StorageInterestGroupAuctionEventType {
    Started,
    ConfigResolved,
}
/// Enum of network fetches auctions can do.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Storage/#type-InterestGroupAuctionFetchType>
pub enum StorageInterestGroupAuctionFetchType {
    BidderJs,
    BidderWasm,
    SellerJs,
    BidderTrustedSignals,
    SellerTrustedSignals,
}
/// Enum of shared storage access scopes.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Storage/#type-SharedStorageAccessScope>
pub enum StorageSharedStorageAccessScope {
    Window,
    SharedStorageWorklet,
    ProtectedAudienceWorklet,
    Header,
}
/// Enum of shared storage access methods.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Storage/#type-SharedStorageAccessMethod>
pub enum StorageSharedStorageAccessMethod {
    AddModule,
    CreateWorklet,
    SelectUrl,
    Run,
    BatchUpdate,
    Set,
    Append,
    Delete,
    Clear,
    Get,
    Keys,
    Values,
    Entries,
    Length,
    RemainingBudget,
}
/// Struct for a single key-value pair in an origin's shared storage.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Storage/#type-SharedStorageEntry>
pub struct StorageSharedStorageEntry {
    pub key: String,
    pub value: String,
}
/// Details for an origin's shared storage.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Storage/#type-SharedStorageMetadata>
pub struct StorageSharedStorageMetadata {
    pub creation_time: (),
    pub length: i64,
    pub remaining_budget: u64,
    pub bytes_used: i64,
}
/** Represents a dictionary object passed in as privateAggregationConfig to
run or selectURL.*/
/// <https://chromedevtools.github.io/devtools-protocol/tot/Storage/#type-SharedStoragePrivateAggregationConfig>
pub struct StorageSharedStoragePrivateAggregationConfig {
    pub aggregation_coordinator_origin: String,
    pub context_id: String,
    pub filtering_id_max_bytes: i64,
    pub max_contributions: i64,
}
/// Pair of reporting metadata details for a candidate URL for `selectURL()`.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Storage/#type-SharedStorageReportingMetadata>
pub struct StorageSharedStorageReportingMetadata {
    pub event_type: String,
    pub reporting_url: String,
}
/// Bundles a candidate URL with its reporting metadata.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Storage/#type-SharedStorageUrlWithMetadata>
pub struct StorageSharedStorageUrlWithMetadata {
    pub url: String,
    pub reporting_metadata: (),
}
/** Bundles the parameters for shared storage access events whose
presence/absence can vary according to SharedStorageAccessType.*/
/// <https://chromedevtools.github.io/devtools-protocol/tot/Storage/#type-SharedStorageAccessParams>
pub struct StorageSharedStorageAccessParams {
    pub script_source_url: String,
    pub data_origin: String,
    pub operation_name: String,
    pub keep_alive: (),
    pub private_aggregation_config: (),
    pub serialized_data: String,
    pub urls_with_metadata: (),
    pub urn_uuid: String,
    pub key: String,
    pub value: String,
    pub ignore_if_present: (),
    pub worklet_id: String,
    pub with_lock: String,
    pub batch_update_id: String,
    pub batch_size: i64,
}
/// <https://chromedevtools.github.io/devtools-protocol/tot/Storage/#type-StorageBucketsDurability>
pub enum StorageBucketsDurability {
    Relaxed,
    Strict,
}
/// <https://chromedevtools.github.io/devtools-protocol/tot/Storage/#type-StorageBucket>
pub struct StorageBucket {
    pub storage_key: (),
    pub name: String,
}
/// <https://chromedevtools.github.io/devtools-protocol/tot/Storage/#type-StorageBucketInfo>
pub struct StorageBucketInfo {
    pub bucket: (),
    pub id: String,
    pub expiration: (),
    pub quota: u64,
    pub persistent: (),
    pub durability: (),
}
/// ⚠️ Experimental
/// <https://chromedevtools.github.io/devtools-protocol/tot/Storage/#type-AttributionReportingSourceType>
pub enum StorageAttributionReportingSourceType {
    Navigation,
    Event,
}
/// ⚠️ Experimental
/// <https://chromedevtools.github.io/devtools-protocol/tot/Storage/#type-UnsignedInt64AsBase10>
pub struct StorageUnsignedInt64AsBase10(String);
/// ⚠️ Experimental
/// <https://chromedevtools.github.io/devtools-protocol/tot/Storage/#type-UnsignedInt128AsBase16>
pub struct StorageUnsignedInt128AsBase16(String);
/// ⚠️ Experimental
/// <https://chromedevtools.github.io/devtools-protocol/tot/Storage/#type-SignedInt64AsBase10>
pub struct StorageSignedInt64AsBase10(String);
/// ⚠️ Experimental
/// <https://chromedevtools.github.io/devtools-protocol/tot/Storage/#type-AttributionReportingFilterDataEntry>
pub struct StorageAttributionReportingFilterDataEntry {
    pub key: String,
    pub values: (),
}
/// ⚠️ Experimental
/// <https://chromedevtools.github.io/devtools-protocol/tot/Storage/#type-AttributionReportingFilterConfig>
pub struct StorageAttributionReportingFilterConfig {
    pub filter_values: (),
    pub lookback_window: i64,
}
/// ⚠️ Experimental
/// <https://chromedevtools.github.io/devtools-protocol/tot/Storage/#type-AttributionReportingFilterPair>
pub struct StorageAttributionReportingFilterPair {
    pub filters: (),
    pub not_filters: (),
}
/// ⚠️ Experimental
/// <https://chromedevtools.github.io/devtools-protocol/tot/Storage/#type-AttributionReportingAggregationKeysEntry>
pub struct StorageAttributionReportingAggregationKeysEntry {
    pub key: String,
    pub value: (),
}
/// ⚠️ Experimental
/// <https://chromedevtools.github.io/devtools-protocol/tot/Storage/#type-AttributionReportingEventReportWindows>
pub struct StorageAttributionReportingEventReportWindows {
    pub start: i64,
    pub ends: (),
}
/// ⚠️ Experimental
/// <https://chromedevtools.github.io/devtools-protocol/tot/Storage/#type-AttributionReportingTriggerSpec>
pub struct StorageAttributionReportingTriggerSpec {
    pub trigger_data: (),
    pub event_report_windows: (),
}
/// ⚠️ Experimental
/// <https://chromedevtools.github.io/devtools-protocol/tot/Storage/#type-AttributionReportingTriggerDataMatching>
pub enum StorageAttributionReportingTriggerDataMatching {
    Exact,
    Modulus,
}
/// ⚠️ Experimental
/// <https://chromedevtools.github.io/devtools-protocol/tot/Storage/#type-AttributionReportingAggregatableDebugReportingData>
pub struct StorageAttributionReportingAggregatableDebugReportingData {
    pub key_piece: (),
    pub value: u64,
    pub types: (),
}
/// ⚠️ Experimental
/// <https://chromedevtools.github.io/devtools-protocol/tot/Storage/#type-AttributionReportingAggregatableDebugReportingConfig>
pub struct StorageAttributionReportingAggregatableDebugReportingConfig {
    pub budget: u64,
    pub key_piece: (),
    pub debug_data: (),
    pub aggregation_coordinator_origin: String,
}
/// ⚠️ Experimental
/// <https://chromedevtools.github.io/devtools-protocol/tot/Storage/#type-AttributionScopesData>
pub struct StorageAttributionScopesData {
    pub values: (),
    pub limit: u64,
    pub max_event_states: u64,
}
/// ⚠️ Experimental
/// <https://chromedevtools.github.io/devtools-protocol/tot/Storage/#type-AttributionReportingNamedBudgetDef>
pub struct StorageAttributionReportingNamedBudgetDef {
    pub name: String,
    pub budget: i64,
}
/// ⚠️ Experimental
/// <https://chromedevtools.github.io/devtools-protocol/tot/Storage/#type-AttributionReportingSourceRegistration>
pub struct StorageAttributionReportingSourceRegistration {
    pub time: (),
    pub expiry: i64,
    pub trigger_specs: (),
    pub aggregatable_report_window: i64,
    pub _type: (),
    pub source_origin: String,
    pub reporting_origin: String,
    pub destination_sites: (),
    pub event_id: (),
    pub priority: (),
    pub filter_data: (),
    pub aggregation_keys: (),
    pub debug_key: (),
    pub trigger_data_matching: (),
    pub destination_limit_priority: (),
    pub aggregatable_debug_reporting_config: (),
    pub scopes_data: (),
    pub max_event_level_reports: i64,
    pub named_budgets: (),
    pub debug_reporting: (),
    pub event_level_epsilon: u64,
}
/// ⚠️ Experimental
/// <https://chromedevtools.github.io/devtools-protocol/tot/Storage/#type-AttributionReportingSourceRegistrationResult>
pub enum StorageAttributionReportingSourceRegistrationResult {
    Success,
    InternalError,
    InsufficientSourceCapacity,
    InsufficientUniqueDestinationCapacity,
    ExcessiveReportingOrigins,
    ProhibitedByBrowserPolicy,
    SuccessNoised,
    DestinationReportingLimitReached,
    DestinationGlobalLimitReached,
    DestinationBothLimitsReached,
    ReportingOriginsPerSiteLimitReached,
    ExceedsMaxChannelCapacity,
    ExceedsMaxScopesChannelCapacity,
    ExceedsMaxTriggerStateCardinality,
    ExceedsMaxEventStatesLimit,
    DestinationPerDayReportingLimitReached,
}
/// ⚠️ Experimental
/// <https://chromedevtools.github.io/devtools-protocol/tot/Storage/#type-AttributionReportingSourceRegistrationTimeConfig>
pub enum StorageAttributionReportingSourceRegistrationTimeConfig {
    Include,
    Exclude,
}
/// ⚠️ Experimental
/// <https://chromedevtools.github.io/devtools-protocol/tot/Storage/#type-AttributionReportingAggregatableValueDictEntry>
pub struct StorageAttributionReportingAggregatableValueDictEntry {
    pub key: String,
    pub value: u64,
    pub filtering_id: (),
}
/// ⚠️ Experimental
/// <https://chromedevtools.github.io/devtools-protocol/tot/Storage/#type-AttributionReportingAggregatableValueEntry>
pub struct StorageAttributionReportingAggregatableValueEntry {
    pub values: (),
    pub filters: (),
}
/// ⚠️ Experimental
/// <https://chromedevtools.github.io/devtools-protocol/tot/Storage/#type-AttributionReportingEventTriggerData>
pub struct StorageAttributionReportingEventTriggerData {
    pub data: (),
    pub priority: (),
    pub dedup_key: (),
    pub filters: (),
}
/// ⚠️ Experimental
/// <https://chromedevtools.github.io/devtools-protocol/tot/Storage/#type-AttributionReportingAggregatableTriggerData>
pub struct StorageAttributionReportingAggregatableTriggerData {
    pub key_piece: (),
    pub source_keys: (),
    pub filters: (),
}
/// ⚠️ Experimental
/// <https://chromedevtools.github.io/devtools-protocol/tot/Storage/#type-AttributionReportingAggregatableDedupKey>
pub struct StorageAttributionReportingAggregatableDedupKey {
    pub dedup_key: (),
    pub filters: (),
}
/// ⚠️ Experimental
/// <https://chromedevtools.github.io/devtools-protocol/tot/Storage/#type-AttributionReportingNamedBudgetCandidate>
pub struct StorageAttributionReportingNamedBudgetCandidate {
    pub name: String,
    pub filters: (),
}
/// ⚠️ Experimental
/// <https://chromedevtools.github.io/devtools-protocol/tot/Storage/#type-AttributionReportingTriggerRegistration>
pub struct StorageAttributionReportingTriggerRegistration {
    pub filters: (),
    pub debug_key: (),
    pub aggregatable_dedup_keys: (),
    pub event_trigger_data: (),
    pub aggregatable_trigger_data: (),
    pub aggregatable_values: (),
    pub aggregatable_filtering_id_max_bytes: i64,
    pub debug_reporting: (),
    pub aggregation_coordinator_origin: String,
    pub source_registration_time_config: (),
    pub trigger_context_id: String,
    pub aggregatable_debug_reporting_config: (),
    pub scopes: (),
    pub named_budgets: (),
}
/// ⚠️ Experimental
/// <https://chromedevtools.github.io/devtools-protocol/tot/Storage/#type-AttributionReportingEventLevelResult>
pub enum StorageAttributionReportingEventLevelResult {
    Success,
    SuccessDroppedLowerPriority,
    InternalError,
    NoCapacityForAttributionDestination,
    NoMatchingSources,
    Deduplicated,
    ExcessiveAttributions,
    PriorityTooLow,
    NeverAttributedSource,
    ExcessiveReportingOrigins,
    NoMatchingSourceFilterData,
    ProhibitedByBrowserPolicy,
    NoMatchingConfigurations,
    ExcessiveReports,
    FalselyAttributedSource,
    ReportWindowPassed,
    NotRegistered,
    ReportWindowNotStarted,
    NoMatchingTriggerData,
}
/// ⚠️ Experimental
/// <https://chromedevtools.github.io/devtools-protocol/tot/Storage/#type-AttributionReportingAggregatableResult>
pub enum StorageAttributionReportingAggregatableResult {
    Success,
    InternalError,
    NoCapacityForAttributionDestination,
    NoMatchingSources,
    ExcessiveAttributions,
    ExcessiveReportingOrigins,
    NoHistograms,
    InsufficientBudget,
    InsufficientNamedBudget,
    NoMatchingSourceFilterData,
    NotRegistered,
    ProhibitedByBrowserPolicy,
    Deduplicated,
    ReportWindowPassed,
    ExcessiveReports,
}
/// ⚠️ Experimental
/// <https://chromedevtools.github.io/devtools-protocol/tot/Storage/#type-AttributionReportingReportResult>
pub enum StorageAttributionReportingReportResult {
    Sent,
    Prohibited,
    FailedToAssemble,
    Expired,
}
/// ⚠️ Experimental
/// A single Related Website Set object.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Storage/#type-RelatedWebsiteSet>
pub struct StorageRelatedWebsiteSet {
    pub primary_sites: (),
    pub associated_sites: (),
    pub service_sites: (),
}
