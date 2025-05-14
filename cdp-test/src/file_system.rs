use crate::common::*;
use crate::network::*;
use crate::storage::*;
pub struct File {
    pub name: Box<String>,
    pub last_modified: Box<NetworkTimeSinceEpoch>,
    pub size: Box<u64>,
    pub _type: Box<String>,
}
pub struct Directory {
    pub name: Box<String>,
    pub nested_directories: (),
    pub nested_files: (),
}
pub struct BucketFileSystemLocator {
    pub storage_key: Box<StorageSerializedStorageKey>,
    pub bucket_name: Box<String>,
    pub path_components: (),
}
