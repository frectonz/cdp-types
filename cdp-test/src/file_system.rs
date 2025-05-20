use crate::common::*;
use crate::network::*;
use crate::storage::*;
pub struct File {
    pub name: String,
    pub last_modified: Box<NetworkTimeSinceEpoch>,
    pub size: u64,
    pub _type: String,
}
pub struct Directory {
    pub name: String,
    pub nested_directories: Vec<String>,
    pub nested_files: Vec<File>,
}
pub struct BucketFileSystemLocator {
    pub storage_key: Box<StorageSerializedStorageKey>,
    pub bucket_name: String,
    pub path_components: Vec<String>,
}
pub type FileSystemGetDirectory = ();
