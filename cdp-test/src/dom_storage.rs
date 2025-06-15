use crate::common::*;
/// DOM Storage identifier.
pub struct StorageId {
    pub security_origin: String,
    pub storage_key: Box<StorageSerializedStorageKey>,
    pub is_local_storage: bool,
}
/// DOM Storage item.
pub struct Item(Vec<String>);
pub struct DomStorageClearParams {
    pub storage_id: Box<StorageId>,
}
pub type DomStorageClearReturns = ();
/// Disables storage tracking, prevents storage events from being sent to the client.
pub type DomStorageDisableParams = ();
/// Disables storage tracking, prevents storage events from being sent to the client.
pub type DomStorageDisableReturns = ();
/// Enables storage tracking, storage events will now be delivered to the client.
pub type DomStorageEnableParams = ();
/// Enables storage tracking, storage events will now be delivered to the client.
pub type DomStorageEnableReturns = ();
pub struct DomStorageGetDomStorageItemsParams {
    pub storage_id: Box<StorageId>,
}
pub struct DomStorageGetDomStorageItemsParams {
    pub entries: Vec<Item>,
}
pub struct DomStorageRemoveDomStorageItemParams {
    pub storage_id: Box<StorageId>,
    pub key: String,
}
pub type DomStorageRemoveDomStorageItemReturns = ();
pub struct DomStorageSetDomStorageItemParams {
    pub storage_id: Box<StorageId>,
    pub key: String,
    pub value: String,
}
pub type DomStorageSetDomStorageItemReturns = ();
