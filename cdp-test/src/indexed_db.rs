pub use crate::common::*;
use crate::storage::*;
/// Database with an array of object stores.
pub struct DatabaseWithObjectStores {
    pub name: String,
    pub version: u64,
    pub object_stores: (),
}
/// Object store.
pub struct ObjectStore {
    pub name: String,
    pub key_path: (),
    pub auto_increment: (),
    pub indexes: (),
}
/// Object store index.
pub struct ObjectStoreIndex {
    pub name: String,
    pub key_path: (),
    pub unique: (),
    pub multi_entry: (),
}
/// Key.
pub struct Key {
    pub _type: String,
    pub number: u64,
    pub string: String,
    pub date: u64,
    pub array: (),
}
/// Key range.
pub struct KeyRange {
    pub lower: (),
    pub upper: (),
    pub lower_open: (),
    pub upper_open: (),
}
/// Key path.
pub struct KeyPath {
    pub _type: String,
    pub string: String,
    pub array: (),
}
