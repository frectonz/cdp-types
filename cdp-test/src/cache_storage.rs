use crate::storage::*;
/// Unique identifier of the Cache object.
/// <https://chromedevtools.github.io/devtools-protocol/tot/CacheStorage/#type-CacheId>
pub struct CacheStorageCacheId(String);
/// type of HTTP response cached
/// <https://chromedevtools.github.io/devtools-protocol/tot/CacheStorage/#type-CachedResponseType>
pub enum CacheStorageCachedResponseType {
    Basic,
    Cors,
    Default,
    Error,
    OpaqueResponse,
    OpaqueRedirect,
}
/// Data entry.
/// <https://chromedevtools.github.io/devtools-protocol/tot/CacheStorage/#type-DataEntry>
pub struct CacheStorageDataEntry {
    pub request_url: (),
    pub request_method: (),
    pub request_headers: (),
    pub response_time: (),
    pub response_status: (),
    pub response_status_text: (),
    pub response_type: (),
    pub response_headers: (),
}
/// Cache identifier.
/// <https://chromedevtools.github.io/devtools-protocol/tot/CacheStorage/#type-Cache>
pub struct CacheStorageCache {
    pub cache_id: (),
    pub security_origin: (),
    pub storage_key: (),
    pub storage_bucket: (),
    pub cache_name: (),
}
/// <https://chromedevtools.github.io/devtools-protocol/tot/CacheStorage/#type-Header>
pub struct CacheStorageHeader {
    pub name: (),
    pub value: (),
}
/// Cached response
/// <https://chromedevtools.github.io/devtools-protocol/tot/CacheStorage/#type-CachedResponse>
pub struct CacheStorageCachedResponse {
    pub body: (),
}
