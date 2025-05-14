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
