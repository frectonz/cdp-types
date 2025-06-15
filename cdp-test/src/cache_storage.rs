use crate::common::*;
use crate::storage::*;
/// Unique identifier of the Cache object.
pub struct CacheId(String);
/// type of HTTP response cached
pub enum CachedResponseType {
    Basic,
    Cors,
    Default,
    Error,
    OpaqueResponse,
    OpaqueRedirect,
}
/// Cache identifier.
pub struct Cache {
    pub cache_id: Box<CacheId>,
    pub security_origin: String,
    pub storage_key: String,
    pub storage_bucket: Box<StorageBucket>,
    pub cache_name: String,
}
pub struct Header {
    pub name: String,
    pub value: String,
}
/// Cached response
pub struct CachedResponse {
    pub body: String,
}
/// Deletes a cache.
pub struct CacheStorageDeleteCacheParams {
    pub cache_id: Box<CacheId>,
}
/// Deletes a cache.
pub type CacheStorageDeleteCacheReturns = ();
/// Deletes a cache entry.
pub struct CacheStorageDeleteEntryParams {
    pub cache_id: Box<CacheId>,
    pub request: String,
}
/// Deletes a cache entry.
pub type CacheStorageDeleteEntryReturns = ();
/// Requests cache names.
pub struct CacheStorageRequestCacheNamesParams {
    pub security_origin: String,
    pub storage_key: String,
    pub storage_bucket: Box<StorageBucket>,
}
/// Requests cache names.
pub type CacheStorageRequestCacheNamesReturns = ();
/// Fetches cache entry.
pub struct CacheStorageRequestCachedResponseParams {
    pub cache_id: Box<CacheId>,
    pub request_url: String,
    pub request_headers: Vec<Header>,
}
/// Fetches cache entry.
pub type CacheStorageRequestCachedResponseReturns = ();
/// Requests data from cache.
pub struct CacheStorageRequestEntriesParams {
    pub cache_id: Box<CacheId>,
    pub skip_count: i64,
    pub page_size: i64,
    pub path_filter: String,
}
/// Requests data from cache.
pub type CacheStorageRequestEntriesReturns = ();
