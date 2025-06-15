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
    pub security_origin: String,
    pub storage_key: String,
    pub storage_bucket: Box<StorageBucket>,
    pub database_name: String,
    pub object_store_name: String,
}
/// Clears all entries from an object store.
pub type IndexedDbClearObjectStoreReturns = ();
/// Deletes a database.
pub struct IndexedDbDeleteDatabaseParams {
    pub security_origin: String,
    pub storage_key: String,
    pub storage_bucket: Box<StorageBucket>,
    pub database_name: String,
}
/// Deletes a database.
pub type IndexedDbDeleteDatabaseReturns = ();
/// Delete a range of entries from an object store
pub struct IndexedDbDeleteObjectStoreEntriesParams {
    pub security_origin: String,
    pub storage_key: String,
    pub storage_bucket: Box<StorageBucket>,
    pub database_name: String,
    pub object_store_name: String,
    pub key_range: Box<KeyRange>,
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
    pub security_origin: String,
    pub storage_key: String,
    pub storage_bucket: Box<StorageBucket>,
    pub database_name: String,
    pub object_store_name: String,
    pub index_name: String,
    pub skip_count: i64,
    pub page_size: i64,
    pub key_range: Box<KeyRange>,
}
/// Requests data from object store or index.
pub struct IndexedDbRequestDataParams {
    pub object_store_data_entries: Vec<DataEntry>,
    pub has_more: bool,
}
/// Gets metadata of an object store.
pub struct IndexedDbGetMetadataParams {
    pub security_origin: String,
    pub storage_key: String,
    pub storage_bucket: Box<StorageBucket>,
    pub database_name: String,
    pub object_store_name: String,
}
/// Gets metadata of an object store.
pub struct IndexedDbGetMetadataParams {
    pub entries_count: u64,
    pub key_generator_value: u64,
}
/// Requests database with given name in given frame.
pub struct IndexedDbRequestDatabaseParams {
    pub security_origin: String,
    pub storage_key: String,
    pub storage_bucket: Box<StorageBucket>,
    pub database_name: String,
}
/// Requests database with given name in given frame.
pub struct IndexedDbRequestDatabaseParams {
    pub database_with_object_stores: Box<DatabaseWithObjectStores>,
}
/// Requests database names for given security origin.
pub struct IndexedDbRequestDatabaseNamesParams {
    pub security_origin: String,
    pub storage_key: String,
    pub storage_bucket: Box<StorageBucket>,
}
/// Requests database names for given security origin.
pub struct IndexedDbRequestDatabaseNamesParams {
    pub database_names: Vec<String>,
}
