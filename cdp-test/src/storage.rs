use crate::common::*;
use crate::browser::*;
use crate::network::*;
/// Enum of possible storage types.
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
pub struct UsageForType {
    pub storage_type: Box<StorageType>,
    pub usage: Box<u64>,
}
/// ⚠️ Experimental
/** Pair of issuer origin and number of available (signed, but not used) Trust
Tokens from that issuer.*/
pub struct TrustTokens {
    pub issuer_origin: Box<String>,
    pub count: Box<u64>,
}
/// Protected audience interest group auction identifier.
pub struct InterestGroupAuctionId(String);
/// Enum of interest group access types.
pub enum InterestGroupAccessType {
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
pub enum InterestGroupAuctionEventType {
    Started,
    ConfigResolved,
}
/// Enum of network fetches auctions can do.
pub enum InterestGroupAuctionFetchType {
    BidderJs,
    BidderWasm,
    SellerJs,
    BidderTrustedSignals,
    SellerTrustedSignals,
}
/// Enum of shared storage access scopes.
pub enum SharedStorageAccessScope {
    Window,
    SharedStorageWorklet,
    ProtectedAudienceWorklet,
    Header,
}
/// Enum of shared storage access methods.
pub enum SharedStorageAccessMethod {
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
pub struct SharedStorageEntry {
    pub key: Box<String>,
    pub value: Box<String>,
}
/// Details for an origin's shared storage.
pub struct SharedStorageMetadata {
    pub creation_time: Box<NetworkTimeSinceEpoch>,
    pub length: Box<i64>,
    pub remaining_budget: Box<u64>,
    pub bytes_used: Box<i64>,
}
/** Represents a dictionary object passed in as privateAggregationConfig to
run or selectURL.*/
pub struct SharedStoragePrivateAggregationConfig {
    pub aggregation_coordinator_origin: Box<String>,
    pub context_id: Box<String>,
    pub filtering_id_max_bytes: Box<i64>,
    pub max_contributions: Box<i64>,
}
/// Pair of reporting metadata details for a candidate URL for `selectURL()`.
pub struct SharedStorageReportingMetadata {
    pub event_type: Box<String>,
    pub reporting_url: Box<String>,
}
/// Bundles a candidate URL with its reporting metadata.
pub struct SharedStorageUrlWithMetadata {
    pub url: Box<String>,
    pub reporting_metadata: (),
}
/** Bundles the parameters for shared storage access events whose
presence/absence can vary according to SharedStorageAccessType.*/
pub struct SharedStorageAccessParams {
    pub script_source_url: Box<String>,
    pub data_origin: Box<String>,
    pub operation_name: Box<String>,
    pub keep_alive: (),
    pub private_aggregation_config: Box<SharedStoragePrivateAggregationConfig>,
    pub serialized_data: Box<String>,
    pub urls_with_metadata: (),
    pub urn_uuid: Box<String>,
    pub key: Box<String>,
    pub value: Box<String>,
    pub ignore_if_present: (),
    pub worklet_id: Box<String>,
    pub with_lock: Box<String>,
    pub batch_update_id: Box<String>,
    pub batch_size: Box<i64>,
}
pub enum StorageBucketsDurability {
    Relaxed,
    Strict,
}
pub struct StorageBucket {
    pub storage_key: Box<StorageSerializedStorageKey>,
    pub name: Box<String>,
}
pub struct StorageBucketInfo {
    pub bucket: Box<StorageBucket>,
    pub id: Box<String>,
    pub expiration: Box<NetworkTimeSinceEpoch>,
    pub quota: Box<u64>,
    pub persistent: (),
    pub durability: Box<StorageBucketsDurability>,
}
/// ⚠️ Experimental
pub enum AttributionReportingSourceType {
    Navigation,
    Event,
}
/// ⚠️ Experimental
pub struct UnsignedInt64AsBase10(String);
/// ⚠️ Experimental
pub struct UnsignedInt128AsBase16(String);
/// ⚠️ Experimental
pub struct SignedInt64AsBase10(String);
/// ⚠️ Experimental
pub struct AttributionReportingFilterDataEntry {
    pub key: Box<String>,
    pub values: (),
}
/// ⚠️ Experimental
pub struct AttributionReportingFilterConfig {
    pub filter_values: (),
    pub lookback_window: Box<i64>,
}
/// ⚠️ Experimental
pub struct AttributionReportingFilterPair {
    pub filters: (),
    pub not_filters: (),
}
/// ⚠️ Experimental
pub struct AttributionReportingAggregationKeysEntry {
    pub key: Box<String>,
    pub value: Box<UnsignedInt128AsBase16>,
}
/// ⚠️ Experimental
pub struct AttributionReportingEventReportWindows {
    pub start: Box<i64>,
    pub ends: (),
}
/// ⚠️ Experimental
pub struct AttributionReportingTriggerSpec {
    pub trigger_data: (),
    pub event_report_windows: Box<AttributionReportingEventReportWindows>,
}
/// ⚠️ Experimental
pub enum AttributionReportingTriggerDataMatching {
    Exact,
    Modulus,
}
/// ⚠️ Experimental
pub struct AttributionReportingAggregatableDebugReportingData {
    pub key_piece: Box<UnsignedInt128AsBase16>,
    pub value: Box<u64>,
    pub types: (),
}
/// ⚠️ Experimental
pub struct AttributionReportingAggregatableDebugReportingConfig {
    pub budget: Box<u64>,
    pub key_piece: Box<UnsignedInt128AsBase16>,
    pub debug_data: (),
    pub aggregation_coordinator_origin: Box<String>,
}
/// ⚠️ Experimental
pub struct AttributionScopesData {
    pub values: (),
    pub limit: Box<u64>,
    pub max_event_states: Box<u64>,
}
/// ⚠️ Experimental
pub struct AttributionReportingNamedBudgetDef {
    pub name: Box<String>,
    pub budget: Box<i64>,
}
/// ⚠️ Experimental
pub struct AttributionReportingSourceRegistration {
    pub time: Box<NetworkTimeSinceEpoch>,
    pub expiry: Box<i64>,
    pub trigger_specs: (),
    pub aggregatable_report_window: Box<i64>,
    pub _type: Box<AttributionReportingSourceType>,
    pub source_origin: Box<String>,
    pub reporting_origin: Box<String>,
    pub destination_sites: (),
    pub event_id: Box<UnsignedInt64AsBase10>,
    pub priority: Box<SignedInt64AsBase10>,
    pub filter_data: (),
    pub aggregation_keys: (),
    pub debug_key: Box<UnsignedInt64AsBase10>,
    pub trigger_data_matching: Box<AttributionReportingTriggerDataMatching>,
    pub destination_limit_priority: Box<SignedInt64AsBase10>,
    pub aggregatable_debug_reporting_config: Box<
        AttributionReportingAggregatableDebugReportingConfig,
    >,
    pub scopes_data: Box<AttributionScopesData>,
    pub max_event_level_reports: Box<i64>,
    pub named_budgets: (),
    pub debug_reporting: (),
    pub event_level_epsilon: Box<u64>,
}
/// ⚠️ Experimental
pub enum AttributionReportingSourceRegistrationResult {
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
pub enum AttributionReportingSourceRegistrationTimeConfig {
    Include,
    Exclude,
}
/// ⚠️ Experimental
pub struct AttributionReportingAggregatableValueDictEntry {
    pub key: Box<String>,
    pub value: Box<u64>,
    pub filtering_id: Box<UnsignedInt64AsBase10>,
}
/// ⚠️ Experimental
pub struct AttributionReportingAggregatableValueEntry {
    pub values: (),
    pub filters: Box<AttributionReportingFilterPair>,
}
/// ⚠️ Experimental
pub struct AttributionReportingEventTriggerData {
    pub data: Box<UnsignedInt64AsBase10>,
    pub priority: Box<SignedInt64AsBase10>,
    pub dedup_key: Box<UnsignedInt64AsBase10>,
    pub filters: Box<AttributionReportingFilterPair>,
}
/// ⚠️ Experimental
pub struct AttributionReportingAggregatableTriggerData {
    pub key_piece: Box<UnsignedInt128AsBase16>,
    pub source_keys: (),
    pub filters: Box<AttributionReportingFilterPair>,
}
/// ⚠️ Experimental
pub struct AttributionReportingAggregatableDedupKey {
    pub dedup_key: Box<UnsignedInt64AsBase10>,
    pub filters: Box<AttributionReportingFilterPair>,
}
/// ⚠️ Experimental
pub struct AttributionReportingNamedBudgetCandidate {
    pub name: Box<String>,
    pub filters: Box<AttributionReportingFilterPair>,
}
/// ⚠️ Experimental
pub struct AttributionReportingTriggerRegistration {
    pub filters: Box<AttributionReportingFilterPair>,
    pub debug_key: Box<UnsignedInt64AsBase10>,
    pub aggregatable_dedup_keys: (),
    pub event_trigger_data: (),
    pub aggregatable_trigger_data: (),
    pub aggregatable_values: (),
    pub aggregatable_filtering_id_max_bytes: Box<i64>,
    pub debug_reporting: (),
    pub aggregation_coordinator_origin: Box<String>,
    pub source_registration_time_config: Box<
        AttributionReportingSourceRegistrationTimeConfig,
    >,
    pub trigger_context_id: Box<String>,
    pub aggregatable_debug_reporting_config: Box<
        AttributionReportingAggregatableDebugReportingConfig,
    >,
    pub scopes: (),
    pub named_budgets: (),
}
/// ⚠️ Experimental
pub enum AttributionReportingEventLevelResult {
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
pub enum AttributionReportingAggregatableResult {
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
pub enum AttributionReportingReportResult {
    Sent,
    Prohibited,
    FailedToAssemble,
    Expired,
}
/// ⚠️ Experimental
/// A single Related Website Set object.
pub struct RelatedWebsiteSet {
    pub primary_sites: (),
    pub associated_sites: (),
    pub service_sites: (),
}
