pub use crate::common::*;
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
    pub storage_type: (),
    pub usage: u64,
}
/// ⚠️ Experimental
/** Pair of issuer origin and number of available (signed, but not used) Trust
Tokens from that issuer.*/
pub struct TrustTokens {
    pub issuer_origin: String,
    pub count: u64,
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
    pub key: String,
    pub value: String,
}
/// Details for an origin's shared storage.
pub struct SharedStorageMetadata {
    pub creation_time: (),
    pub length: i64,
    pub remaining_budget: u64,
    pub bytes_used: i64,
}
/** Represents a dictionary object passed in as privateAggregationConfig to
run or selectURL.*/
pub struct SharedStoragePrivateAggregationConfig {
    pub aggregation_coordinator_origin: String,
    pub context_id: String,
    pub filtering_id_max_bytes: i64,
    pub max_contributions: i64,
}
/// Pair of reporting metadata details for a candidate URL for `selectURL()`.
pub struct SharedStorageReportingMetadata {
    pub event_type: String,
    pub reporting_url: String,
}
/// Bundles a candidate URL with its reporting metadata.
pub struct SharedStorageUrlWithMetadata {
    pub url: String,
    pub reporting_metadata: (),
}
/** Bundles the parameters for shared storage access events whose
presence/absence can vary according to SharedStorageAccessType.*/
pub struct SharedStorageAccessParams {
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
pub enum StorageBucketsDurability {
    Relaxed,
    Strict,
}
pub struct StorageBucket {
    pub storage_key: (),
    pub name: String,
}
pub struct StorageBucketInfo {
    pub bucket: (),
    pub id: String,
    pub expiration: (),
    pub quota: u64,
    pub persistent: (),
    pub durability: (),
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
    pub key: String,
    pub values: (),
}
/// ⚠️ Experimental
pub struct AttributionReportingFilterConfig {
    pub filter_values: (),
    pub lookback_window: i64,
}
/// ⚠️ Experimental
pub struct AttributionReportingFilterPair {
    pub filters: (),
    pub not_filters: (),
}
/// ⚠️ Experimental
pub struct AttributionReportingAggregationKeysEntry {
    pub key: String,
    pub value: (),
}
/// ⚠️ Experimental
pub struct AttributionReportingEventReportWindows {
    pub start: i64,
    pub ends: (),
}
/// ⚠️ Experimental
pub struct AttributionReportingTriggerSpec {
    pub trigger_data: (),
    pub event_report_windows: (),
}
/// ⚠️ Experimental
pub enum AttributionReportingTriggerDataMatching {
    Exact,
    Modulus,
}
/// ⚠️ Experimental
pub struct AttributionReportingAggregatableDebugReportingData {
    pub key_piece: (),
    pub value: u64,
    pub types: (),
}
/// ⚠️ Experimental
pub struct AttributionReportingAggregatableDebugReportingConfig {
    pub budget: u64,
    pub key_piece: (),
    pub debug_data: (),
    pub aggregation_coordinator_origin: String,
}
/// ⚠️ Experimental
pub struct AttributionScopesData {
    pub values: (),
    pub limit: u64,
    pub max_event_states: u64,
}
/// ⚠️ Experimental
pub struct AttributionReportingNamedBudgetDef {
    pub name: String,
    pub budget: i64,
}
/// ⚠️ Experimental
pub struct AttributionReportingSourceRegistration {
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
    pub key: String,
    pub value: u64,
    pub filtering_id: (),
}
/// ⚠️ Experimental
pub struct AttributionReportingAggregatableValueEntry {
    pub values: (),
    pub filters: (),
}
/// ⚠️ Experimental
pub struct AttributionReportingEventTriggerData {
    pub data: (),
    pub priority: (),
    pub dedup_key: (),
    pub filters: (),
}
/// ⚠️ Experimental
pub struct AttributionReportingAggregatableTriggerData {
    pub key_piece: (),
    pub source_keys: (),
    pub filters: (),
}
/// ⚠️ Experimental
pub struct AttributionReportingAggregatableDedupKey {
    pub dedup_key: (),
    pub filters: (),
}
/// ⚠️ Experimental
pub struct AttributionReportingNamedBudgetCandidate {
    pub name: String,
    pub filters: (),
}
/// ⚠️ Experimental
pub struct AttributionReportingTriggerRegistration {
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
