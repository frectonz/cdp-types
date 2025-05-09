use crate::storage::*;
/// Database with an array of object stores.
/// <https://chromedevtools.github.io/devtools-protocol/tot/IndexedDB/#type-DatabaseWithObjectStores>
pub struct IndexedDbDatabaseWithObjectStores {
    pub name: (),
    pub version: (),
    pub object_stores: (),
}
/// Object store.
/// <https://chromedevtools.github.io/devtools-protocol/tot/IndexedDB/#type-ObjectStore>
pub struct IndexedDbObjectStore {
    pub name: (),
    pub key_path: (),
    pub auto_increment: (),
    pub indexes: (),
}
/// Object store index.
/// <https://chromedevtools.github.io/devtools-protocol/tot/IndexedDB/#type-ObjectStoreIndex>
pub struct IndexedDbObjectStoreIndex {
    pub name: (),
    pub key_path: (),
    pub unique: (),
    pub multi_entry: (),
}
/// Key.
/// <https://chromedevtools.github.io/devtools-protocol/tot/IndexedDB/#type-Key>
pub struct IndexedDbKey {
    pub _type: (),
    pub number: (),
    pub string: (),
    pub date: (),
    pub array: (),
}
/// Key range.
/// <https://chromedevtools.github.io/devtools-protocol/tot/IndexedDB/#type-KeyRange>
pub struct IndexedDbKeyRange {
    pub lower: (),
    pub upper: (),
    pub lower_open: (),
    pub upper_open: (),
}
/// Data entry.
/// <https://chromedevtools.github.io/devtools-protocol/tot/IndexedDB/#type-DataEntry>
pub struct IndexedDbDataEntry {
    pub key: (),
    pub primary_key: (),
    pub value: (),
}
/// Key path.
/// <https://chromedevtools.github.io/devtools-protocol/tot/IndexedDB/#type-KeyPath>
pub struct IndexedDbKeyPath {
    pub _type: (),
    pub string: (),
    pub array: (),
}
