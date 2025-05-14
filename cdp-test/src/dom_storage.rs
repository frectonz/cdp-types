use crate::common::*;
/// DOM Storage identifier.
pub struct StorageId {
    pub security_origin: Box<String>,
    pub storage_key: Box<StorageSerializedStorageKey>,
    pub is_local_storage: (),
}
/// DOM Storage item.
pub struct Item(Vec<String>);
