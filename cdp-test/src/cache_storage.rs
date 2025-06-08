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
pub type CacheStorageDeleteCacheParams = ();
/// Deletes a cache.
pub type CacheStorageDeleteCacheResults = ();
/// Deletes a cache entry.
pub type CacheStorageDeleteEntryParams = ();
/// Deletes a cache entry.
pub type CacheStorageDeleteEntryResults = ();
/// Requests cache names.
pub type CacheStorageRequestCacheNamesParams = ();
/// Requests cache names.
pub type CacheStorageRequestCacheNamesResults = ();
/// Fetches cache entry.
pub type CacheStorageRequestCachedResponseParams = ();
/// Fetches cache entry.
pub type CacheStorageRequestCachedResponseResults = ();
/// Requests data from cache.
pub type CacheStorageRequestEntriesParams = ();
/// Requests data from cache.
pub type CacheStorageRequestEntriesResults = ();
