pub use crate::common::*;
use crate::network::*;
use crate::storage::*;
pub struct File {
    pub name: String,
    pub last_modified: (),
    pub size: u64,
    pub _type: String,
}
pub struct Directory {
    pub name: String,
    pub nested_directories: (),
    pub nested_files: (),
}
pub struct BucketFileSystemLocator {
    pub storage_key: (),
    pub bucket_name: String,
    pub path_components: (),
}
