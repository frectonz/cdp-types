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
    pub security_origin: Box<String>,
    pub storage_key: Box<String>,
    pub storage_bucket: Box<StorageBucket>,
    pub cache_name: Box<String>,
}
pub struct Header {
    pub name: Box<String>,
    pub value: Box<String>,
}
/// Cached response
pub struct CachedResponse {
    pub body: Box<String>,
}
