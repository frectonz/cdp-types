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
    test: (),
}
/// Deletes a cache.
pub type CacheStorageDeleteCacheReturns = ();
/// Deletes a cache entry.
pub struct CacheStorageDeleteEntryParams {
    test: (),
    test: (),
}
/// Deletes a cache entry.
pub type CacheStorageDeleteEntryReturns = ();
/// Requests cache names.
pub struct CacheStorageRequestCacheNamesParams {
    test: (),
    test: (),
    test: (),
}
/// Requests cache names.
pub type CacheStorageRequestCacheNamesReturns = ();
/// Fetches cache entry.
pub struct CacheStorageRequestCachedResponseParams {
    test: (),
    test: (),
    test: (),
}
/// Fetches cache entry.
pub type CacheStorageRequestCachedResponseReturns = ();
/// Requests data from cache.
pub struct CacheStorageRequestEntriesParams {
    test: (),
    test: (),
    test: (),
    test: (),
}
/// Requests data from cache.
pub type CacheStorageRequestEntriesReturns = ();
