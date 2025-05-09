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
    pub request_url: String,
    pub request_method: String,
    pub request_headers: (),
    pub response_time: u64,
    pub response_status: i64,
    pub response_status_text: String,
    pub response_type: (),
    pub response_headers: (),
}
/// Cache identifier.
/// <https://chromedevtools.github.io/devtools-protocol/tot/CacheStorage/#type-Cache>
pub struct CacheStorageCache {
    pub cache_id: (),
    pub security_origin: String,
    pub storage_key: String,
    pub storage_bucket: (),
    pub cache_name: String,
}
/// <https://chromedevtools.github.io/devtools-protocol/tot/CacheStorage/#type-Header>
pub struct CacheStorageHeader {
    pub name: String,
    pub value: String,
}
/// Cached response
/// <https://chromedevtools.github.io/devtools-protocol/tot/CacheStorage/#type-CachedResponse>
pub struct CacheStorageCachedResponse {
    pub body: String,
}
