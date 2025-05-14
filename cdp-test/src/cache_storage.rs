pub use crate::common::*;
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
    pub cache_id: (),
    pub security_origin: String,
    pub storage_key: String,
    pub storage_bucket: (),
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
