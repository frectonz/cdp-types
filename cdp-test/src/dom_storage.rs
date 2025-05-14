pub use crate::common::*;
/// DOM Storage identifier.
pub struct StorageId {
    pub security_origin: String,
    pub storage_key: (),
    pub is_local_storage: (),
}
/// DOM Storage item.
pub struct Item(Vec<String>);
