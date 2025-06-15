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
    pub creation_time: Box<NetworkTimeSinceEpoch>,
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
    pub reporting_metadata: Vec<SharedStorageReportingMetadata>,
}
/** Bundles the parameters for shared storage access events whose
presence/absence can vary according to SharedStorageAccessType.*/
pub struct SharedStorageAccessParams {
    pub script_source_url: String,
    pub data_origin: String,
    pub operation_name: String,
    pub keep_alive: bool,
    pub private_aggregation_config: Box<SharedStoragePrivateAggregationConfig>,
    pub serialized_data: String,
    pub urls_with_metadata: Vec<SharedStorageUrlWithMetadata>,
    pub urn_uuid: String,
    pub key: String,
    pub value: String,
    pub ignore_if_present: bool,
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
    pub storage_key: Box<StorageSerializedStorageKey>,
    pub name: String,
}
pub struct StorageBucketInfo {
    pub bucket: Box<StorageBucket>,
    pub id: String,
    pub expiration: Box<NetworkTimeSinceEpoch>,
    pub quota: u64,
    pub persistent: bool,
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
    pub key: String,
    pub values: Vec<String>,
}
/// ⚠️ Experimental
pub struct AttributionReportingFilterConfig {
    pub filter_values: Vec<AttributionReportingFilterDataEntry>,
    pub lookback_window: i64,
}
/// ⚠️ Experimental
pub struct AttributionReportingFilterPair {
    pub filters: Vec<AttributionReportingFilterConfig>,
    pub not_filters: Vec<AttributionReportingFilterConfig>,
}
/// ⚠️ Experimental
pub struct AttributionReportingAggregationKeysEntry {
    pub key: String,
    pub value: Box<UnsignedInt128AsBase16>,
}
/// ⚠️ Experimental
pub struct AttributionReportingEventReportWindows {
    pub start: i64,
    pub ends: Vec<i64>,
}
/// ⚠️ Experimental
pub struct AttributionReportingTriggerSpec {
    pub trigger_data: Vec<u64>,
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
    pub value: u64,
    pub types: Vec<String>,
}
/// ⚠️ Experimental
pub struct AttributionReportingAggregatableDebugReportingConfig {
    pub budget: u64,
    pub key_piece: Box<UnsignedInt128AsBase16>,
    pub debug_data: Vec<AttributionReportingAggregatableDebugReportingData>,
    pub aggregation_coordinator_origin: String,
}
/// ⚠️ Experimental
pub struct AttributionScopesData {
    pub values: Vec<String>,
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
    pub time: Box<NetworkTimeSinceEpoch>,
    pub expiry: i64,
    pub trigger_specs: Vec<AttributionReportingTriggerSpec>,
    pub aggregatable_report_window: i64,
    pub _type: Box<AttributionReportingSourceType>,
    pub source_origin: String,
    pub reporting_origin: String,
    pub destination_sites: Vec<String>,
    pub event_id: Box<UnsignedInt64AsBase10>,
    pub priority: Box<SignedInt64AsBase10>,
    pub filter_data: Vec<AttributionReportingFilterDataEntry>,
    pub aggregation_keys: Vec<AttributionReportingAggregationKeysEntry>,
    pub debug_key: Box<UnsignedInt64AsBase10>,
    pub trigger_data_matching: Box<AttributionReportingTriggerDataMatching>,
    pub destination_limit_priority: Box<SignedInt64AsBase10>,
    pub aggregatable_debug_reporting_config: Box<
        AttributionReportingAggregatableDebugReportingConfig,
    >,
    pub scopes_data: Box<AttributionScopesData>,
    pub max_event_level_reports: i64,
    pub named_budgets: Vec<AttributionReportingNamedBudgetDef>,
    pub debug_reporting: bool,
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
    pub filtering_id: Box<UnsignedInt64AsBase10>,
}
/// ⚠️ Experimental
pub struct AttributionReportingAggregatableValueEntry {
    pub values: Vec<AttributionReportingAggregatableValueDictEntry>,
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
    pub source_keys: Vec<String>,
    pub filters: Box<AttributionReportingFilterPair>,
}
/// ⚠️ Experimental
pub struct AttributionReportingAggregatableDedupKey {
    pub dedup_key: Box<UnsignedInt64AsBase10>,
    pub filters: Box<AttributionReportingFilterPair>,
}
/// ⚠️ Experimental
pub struct AttributionReportingNamedBudgetCandidate {
    pub name: String,
    pub filters: Box<AttributionReportingFilterPair>,
}
/// ⚠️ Experimental
pub struct AttributionReportingTriggerRegistration {
    pub filters: Box<AttributionReportingFilterPair>,
    pub debug_key: Box<UnsignedInt64AsBase10>,
    pub aggregatable_dedup_keys: Vec<AttributionReportingAggregatableDedupKey>,
    pub event_trigger_data: Vec<AttributionReportingEventTriggerData>,
    pub aggregatable_trigger_data: Vec<AttributionReportingAggregatableTriggerData>,
    pub aggregatable_values: Vec<AttributionReportingAggregatableValueEntry>,
    pub aggregatable_filtering_id_max_bytes: i64,
    pub debug_reporting: bool,
    pub aggregation_coordinator_origin: String,
    pub source_registration_time_config: Box<
        AttributionReportingSourceRegistrationTimeConfig,
    >,
    pub trigger_context_id: String,
    pub aggregatable_debug_reporting_config: Box<
        AttributionReportingAggregatableDebugReportingConfig,
    >,
    pub scopes: Vec<String>,
    pub named_budgets: Vec<AttributionReportingNamedBudgetCandidate>,
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
    pub primary_sites: Vec<String>,
    pub associated_sites: Vec<String>,
    pub service_sites: Vec<String>,
}
/// Returns a storage key given a frame id.
pub struct StorageGetStorageKeyForFrameParams {
    pub frame_id: (),
}
/// Returns a storage key given a frame id.
pub type StorageGetStorageKeyForFrameReturns = ();
/// Clears storage for origin.
pub struct StorageClearDataForOriginParams {
    pub origin: (),
    pub storage_types: (),
}
/// Clears storage for origin.
pub type StorageClearDataForOriginReturns = ();
/// Clears storage for storage key.
pub struct StorageClearDataForStorageKeyParams {
    pub storage_key: (),
    pub storage_types: (),
}
/// Clears storage for storage key.
pub type StorageClearDataForStorageKeyReturns = ();
/// Returns all browser cookies.
pub struct StorageGetCookiesParams {
    pub browser_context_id: (),
}
/// Returns all browser cookies.
pub type StorageGetCookiesReturns = ();
/// Sets given cookies.
pub struct StorageSetCookiesParams {
    pub cookies: (),
    pub browser_context_id: (),
}
/// Sets given cookies.
pub type StorageSetCookiesReturns = ();
/// Clears cookies.
pub struct StorageClearCookiesParams {
    pub browser_context_id: (),
}
/// Clears cookies.
pub type StorageClearCookiesReturns = ();
/// Returns usage and quota in bytes.
pub struct StorageGetUsageAndQuotaParams {
    pub origin: (),
}
/// Returns usage and quota in bytes.
pub type StorageGetUsageAndQuotaReturns = ();
/// ⚠️ Experimental
/// Override quota for the specified origin
pub struct StorageOverrideQuotaForOriginParams {
    pub origin: (),
    pub quota_size: (),
}
/// ⚠️ Experimental
/// Override quota for the specified origin
pub type StorageOverrideQuotaForOriginReturns = ();
/// Registers origin to be notified when an update occurs to its cache storage list.
pub struct StorageTrackCacheStorageForOriginParams {
    pub origin: (),
}
/// Registers origin to be notified when an update occurs to its cache storage list.
pub type StorageTrackCacheStorageForOriginReturns = ();
/// Registers storage key to be notified when an update occurs to its cache storage list.
pub struct StorageTrackCacheStorageForStorageKeyParams {
    pub storage_key: (),
}
/// Registers storage key to be notified when an update occurs to its cache storage list.
pub type StorageTrackCacheStorageForStorageKeyReturns = ();
/// Registers origin to be notified when an update occurs to its IndexedDB.
pub struct StorageTrackIndexedDbForOriginParams {
    pub origin: (),
}
/// Registers origin to be notified when an update occurs to its IndexedDB.
pub type StorageTrackIndexedDbForOriginReturns = ();
/// Registers storage key to be notified when an update occurs to its IndexedDB.
pub struct StorageTrackIndexedDbForStorageKeyParams {
    pub storage_key: (),
}
/// Registers storage key to be notified when an update occurs to its IndexedDB.
pub type StorageTrackIndexedDbForStorageKeyReturns = ();
/// Unregisters origin from receiving notifications for cache storage.
pub struct StorageUntrackCacheStorageForOriginParams {
    pub origin: (),
}
/// Unregisters origin from receiving notifications for cache storage.
pub type StorageUntrackCacheStorageForOriginReturns = ();
/// Unregisters storage key from receiving notifications for cache storage.
pub struct StorageUntrackCacheStorageForStorageKeyParams {
    pub storage_key: (),
}
/// Unregisters storage key from receiving notifications for cache storage.
pub type StorageUntrackCacheStorageForStorageKeyReturns = ();
/// Unregisters origin from receiving notifications for IndexedDB.
pub struct StorageUntrackIndexedDbForOriginParams {
    pub origin: (),
}
/// Unregisters origin from receiving notifications for IndexedDB.
pub type StorageUntrackIndexedDbForOriginReturns = ();
/// Unregisters storage key from receiving notifications for IndexedDB.
pub struct StorageUntrackIndexedDbForStorageKeyParams {
    pub storage_key: (),
}
/// Unregisters storage key from receiving notifications for IndexedDB.
pub type StorageUntrackIndexedDbForStorageKeyReturns = ();
/// ⚠️ Experimental
/** Returns the number of stored Trust Tokens per issuer for the
current browsing context.*/
pub type StorageGetTrustTokensParams = ();
/// ⚠️ Experimental
/** Returns the number of stored Trust Tokens per issuer for the
current browsing context.*/
pub type StorageGetTrustTokensReturns = ();
/// ⚠️ Experimental
/** Removes all Trust Tokens issued by the provided issuerOrigin.
Leaves other stored data, including the issuer's Redemption Records, intact.*/
pub struct StorageClearTrustTokensParams {
    pub issuer_origin: (),
}
/// ⚠️ Experimental
/** Removes all Trust Tokens issued by the provided issuerOrigin.
Leaves other stored data, including the issuer's Redemption Records, intact.*/
pub type StorageClearTrustTokensReturns = ();
/// ⚠️ Experimental
/// Gets details for a named interest group.
pub struct StorageGetInterestGroupDetailsParams {
    pub owner_origin: (),
    pub name: (),
}
/// ⚠️ Experimental
/// Gets details for a named interest group.
pub type StorageGetInterestGroupDetailsReturns = ();
/// ⚠️ Experimental
/// Enables/Disables issuing of interestGroupAccessed events.
pub struct StorageSetInterestGroupTrackingParams {
    pub enable: (),
}
/// ⚠️ Experimental
/// Enables/Disables issuing of interestGroupAccessed events.
pub type StorageSetInterestGroupTrackingReturns = ();
/// ⚠️ Experimental
/** Enables/Disables issuing of interestGroupAuctionEventOccurred and
interestGroupAuctionNetworkRequestCreated.*/
pub struct StorageSetInterestGroupAuctionTrackingParams {
    pub enable: (),
}
/// ⚠️ Experimental
/** Enables/Disables issuing of interestGroupAuctionEventOccurred and
interestGroupAuctionNetworkRequestCreated.*/
pub type StorageSetInterestGroupAuctionTrackingReturns = ();
/// ⚠️ Experimental
/// Gets metadata for an origin's shared storage.
pub struct StorageGetSharedStorageMetadataParams {
    pub owner_origin: (),
}
/// ⚠️ Experimental
/// Gets metadata for an origin's shared storage.
pub type StorageGetSharedStorageMetadataReturns = ();
/// ⚠️ Experimental
/// Gets the entries in an given origin's shared storage.
pub struct StorageGetSharedStorageEntriesParams {
    pub owner_origin: (),
}
/// ⚠️ Experimental
/// Gets the entries in an given origin's shared storage.
pub type StorageGetSharedStorageEntriesReturns = ();
/// ⚠️ Experimental
/// Sets entry with `key` and `value` for a given origin's shared storage.
pub struct StorageSetSharedStorageEntryParams {
    pub owner_origin: (),
    pub key: (),
    pub value: (),
    pub ignore_if_present: (),
}
/// ⚠️ Experimental
/// Sets entry with `key` and `value` for a given origin's shared storage.
pub type StorageSetSharedStorageEntryReturns = ();
/// ⚠️ Experimental
/// Deletes entry for `key` (if it exists) for a given origin's shared storage.
pub struct StorageDeleteSharedStorageEntryParams {
    pub owner_origin: (),
    pub key: (),
}
/// ⚠️ Experimental
/// Deletes entry for `key` (if it exists) for a given origin's shared storage.
pub type StorageDeleteSharedStorageEntryReturns = ();
/// ⚠️ Experimental
/// Clears all entries for a given origin's shared storage.
pub struct StorageClearSharedStorageEntriesParams {
    pub owner_origin: (),
}
/// ⚠️ Experimental
/// Clears all entries for a given origin's shared storage.
pub type StorageClearSharedStorageEntriesReturns = ();
/// ⚠️ Experimental
/// Resets the budget for `ownerOrigin` by clearing all budget withdrawals.
pub struct StorageResetSharedStorageBudgetParams {
    pub owner_origin: (),
}
/// ⚠️ Experimental
/// Resets the budget for `ownerOrigin` by clearing all budget withdrawals.
pub type StorageResetSharedStorageBudgetReturns = ();
/// ⚠️ Experimental
/// Enables/disables issuing of sharedStorageAccessed events.
pub struct StorageSetSharedStorageTrackingParams {
    pub enable: (),
}
/// ⚠️ Experimental
/// Enables/disables issuing of sharedStorageAccessed events.
pub type StorageSetSharedStorageTrackingReturns = ();
/// ⚠️ Experimental
/// Set tracking for a storage key's buckets.
pub struct StorageSetStorageBucketTrackingParams {
    pub storage_key: (),
    pub enable: (),
}
/// ⚠️ Experimental
/// Set tracking for a storage key's buckets.
pub type StorageSetStorageBucketTrackingReturns = ();
/// ⚠️ Experimental
/// Deletes the Storage Bucket with the given storage key and bucket name.
pub struct StorageDeleteStorageBucketParams {
    pub bucket: (),
}
/// ⚠️ Experimental
/// Deletes the Storage Bucket with the given storage key and bucket name.
pub type StorageDeleteStorageBucketReturns = ();
/// ⚠️ Experimental
/// Deletes state for sites identified as potential bounce trackers, immediately.
pub type StorageRunBounceTrackingMitigationsParams = ();
/// ⚠️ Experimental
/// Deletes state for sites identified as potential bounce trackers, immediately.
pub type StorageRunBounceTrackingMitigationsReturns = ();
/// ⚠️ Experimental
/// https://wicg.github.io/attribution-reporting-api/
pub struct StorageSetAttributionReportingLocalTestingModeParams {
    pub enabled: (),
}
/// ⚠️ Experimental
/// https://wicg.github.io/attribution-reporting-api/
pub type StorageSetAttributionReportingLocalTestingModeReturns = ();
/// ⚠️ Experimental
/// Enables/disables issuing of Attribution Reporting events.
pub struct StorageSetAttributionReportingTrackingParams {
    pub enable: (),
}
/// ⚠️ Experimental
/// Enables/disables issuing of Attribution Reporting events.
pub type StorageSetAttributionReportingTrackingReturns = ();
/// ⚠️ Experimental
/** Sends all pending Attribution Reports immediately, regardless of their
scheduled report time.*/
pub type StorageSendPendingAttributionReportsParams = ();
/// ⚠️ Experimental
/** Sends all pending Attribution Reports immediately, regardless of their
scheduled report time.*/
pub type StorageSendPendingAttributionReportsReturns = ();
/// ⚠️ Experimental
/** Returns the effective Related Website Sets in use by this profile for the browser
session. The effective Related Website Sets will not change during a browser session.*/
pub type StorageGetRelatedWebsiteSetsParams = ();
/// ⚠️ Experimental
/** Returns the effective Related Website Sets in use by this profile for the browser
session. The effective Related Website Sets will not change during a browser session.*/
pub type StorageGetRelatedWebsiteSetsReturns = ();
/// ⚠️ Experimental
/** Returns the list of URLs from a page and its embedded resources that match
existing grace period URL pattern rules.
https://developers.google.com/privacy-sandbox/cookies/temporary-exceptions/grace-period*/
pub struct StorageGetAffectedUrlsForThirdPartyCookieMetadataParams {
    pub first_party_url: (),
    pub third_party_urls: (),
}
/// ⚠️ Experimental
/** Returns the list of URLs from a page and its embedded resources that match
existing grace period URL pattern rules.
https://developers.google.com/privacy-sandbox/cookies/temporary-exceptions/grace-period*/
pub type StorageGetAffectedUrlsForThirdPartyCookieMetadataReturns = ();
pub struct StorageSetProtectedAudienceKAnonymityParams {
    pub owner: (),
    pub name: (),
    pub hashes: (),
}
pub type StorageSetProtectedAudienceKAnonymityReturns = ();
