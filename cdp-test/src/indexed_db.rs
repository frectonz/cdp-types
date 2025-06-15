use crate::common::*;
use crate::storage::*;
/// Database with an array of object stores.
pub struct DatabaseWithObjectStores {
    pub name: String,
    pub version: u64,
    pub object_stores: Vec<ObjectStore>,
}
/// Object store.
pub struct ObjectStore {
    pub name: String,
    pub key_path: Box<KeyPath>,
    pub auto_increment: bool,
    pub indexes: Vec<ObjectStoreIndex>,
}
/// Object store index.
pub struct ObjectStoreIndex {
    pub name: String,
    pub key_path: Box<KeyPath>,
    pub unique: bool,
    pub multi_entry: bool,
}
/// Key.
pub struct Key {
    pub _type: String,
    pub number: u64,
    pub string: String,
    pub date: u64,
    pub array: Vec<Key>,
}
/// Key range.
pub struct KeyRange {
    pub lower: Box<Key>,
    pub upper: Box<Key>,
    pub lower_open: bool,
    pub upper_open: bool,
}
/// Key path.
pub struct KeyPath {
    pub _type: String,
    pub string: String,
    pub array: Vec<String>,
}
/// Clears all entries from an object store.
pub struct IndexedDbClearObjectStoreParams {
    pub security_origin: (),
    pub storage_key: (),
    pub storage_bucket: (),
    pub database_name: (),
    pub object_store_name: (),
}
/// Clears all entries from an object store.
pub type IndexedDbClearObjectStoreReturns = ();
/// Deletes a database.
pub struct IndexedDbDeleteDatabaseParams {
    pub security_origin: (),
    pub storage_key: (),
    pub storage_bucket: (),
    pub database_name: (),
}
/// Deletes a database.
pub type IndexedDbDeleteDatabaseReturns = ();
/// Delete a range of entries from an object store
pub struct IndexedDbDeleteObjectStoreEntriesParams {
    pub security_origin: (),
    pub storage_key: (),
    pub storage_bucket: (),
    pub database_name: (),
    pub object_store_name: (),
    pub key_range: (),
}
/// Delete a range of entries from an object store
pub type IndexedDbDeleteObjectStoreEntriesReturns = ();
/// Disables events from backend.
pub type IndexedDbDisableParams = ();
/// Disables events from backend.
pub type IndexedDbDisableReturns = ();
/// Enables events from backend.
pub type IndexedDbEnableParams = ();
/// Enables events from backend.
pub type IndexedDbEnableReturns = ();
/// Requests data from object store or index.
pub struct IndexedDbRequestDataParams {
    pub security_origin: (),
    pub storage_key: (),
    pub storage_bucket: (),
    pub database_name: (),
    pub object_store_name: (),
    pub index_name: (),
    pub skip_count: (),
    pub page_size: (),
    pub key_range: (),
}
/// Requests data from object store or index.
pub type IndexedDbRequestDataReturns = ();
/// Gets metadata of an object store.
pub struct IndexedDbGetMetadataParams {
    pub security_origin: (),
    pub storage_key: (),
    pub storage_bucket: (),
    pub database_name: (),
    pub object_store_name: (),
}
/// Gets metadata of an object store.
pub type IndexedDbGetMetadataReturns = ();
/// Requests database with given name in given frame.
pub struct IndexedDbRequestDatabaseParams {
    pub security_origin: (),
    pub storage_key: (),
    pub storage_bucket: (),
    pub database_name: (),
}
/// Requests database with given name in given frame.
pub type IndexedDbRequestDatabaseReturns = ();
/// Requests database names for given security origin.
pub struct IndexedDbRequestDatabaseNamesParams {
    pub security_origin: (),
    pub storage_key: (),
    pub storage_bucket: (),
}
/// Requests database names for given security origin.
pub type IndexedDbRequestDatabaseNamesReturns = ();
