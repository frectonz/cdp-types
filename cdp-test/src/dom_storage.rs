use crate::common::*;
/// DOM Storage identifier.
pub struct StorageId {
    pub security_origin: String,
    pub storage_key: Box<StorageSerializedStorageKey>,
    pub is_local_storage: bool,
}
/// DOM Storage item.
pub struct Item(Vec<String>);
pub type DomStorageClearParams = ();
pub type DomStorageClearResults = ();
/// Disables storage tracking, prevents storage events from being sent to the client.
pub type DomStorageDisableParams = ();
/// Disables storage tracking, prevents storage events from being sent to the client.
pub type DomStorageDisableResults = ();
/// Enables storage tracking, storage events will now be delivered to the client.
pub type DomStorageEnableParams = ();
/// Enables storage tracking, storage events will now be delivered to the client.
pub type DomStorageEnableResults = ();
pub type DomStorageGetDomStorageItemsParams = ();
pub type DomStorageGetDomStorageItemsResults = ();
pub type DomStorageRemoveDomStorageItemParams = ();
pub type DomStorageRemoveDomStorageItemResults = ();
pub type DomStorageSetDomStorageItemParams = ();
pub type DomStorageSetDomStorageItemResults = ();
