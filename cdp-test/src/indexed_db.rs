use crate::common::*;
use crate::storage::*;
/// Database with an array of object stores.
pub struct DatabaseWithObjectStores {
    pub name: Box<String>,
    pub version: Box<u64>,
    pub object_stores: (),
}
/// Object store.
pub struct ObjectStore {
    pub name: Box<String>,
    pub key_path: Box<KeyPath>,
    pub auto_increment: (),
    pub indexes: (),
}
/// Object store index.
pub struct ObjectStoreIndex {
    pub name: Box<String>,
    pub key_path: Box<KeyPath>,
    pub unique: (),
    pub multi_entry: (),
}
/// Key.
pub struct Key {
    pub _type: Box<String>,
    pub number: Box<u64>,
    pub string: Box<String>,
    pub date: Box<u64>,
    pub array: (),
}
/// Key range.
pub struct KeyRange {
    pub lower: Box<Key>,
    pub upper: Box<Key>,
    pub lower_open: (),
    pub upper_open: (),
}
/// Key path.
pub struct KeyPath {
    pub _type: Box<String>,
    pub string: Box<String>,
    pub array: (),
}
