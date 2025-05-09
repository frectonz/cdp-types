use crate::network::*;
use crate::storage::*;
/// <https://chromedevtools.github.io/devtools-protocol/tot/FileSystem/#type-File>
pub struct FileSystemFile {
    pub name: String,
    pub last_modified: (),
    pub size: u64,
    pub _type: String,
}
/// <https://chromedevtools.github.io/devtools-protocol/tot/FileSystem/#type-Directory>
pub struct FileSystemDirectory {
    pub name: String,
    pub nested_directories: (),
    pub nested_files: (),
}
/// <https://chromedevtools.github.io/devtools-protocol/tot/FileSystem/#type-BucketFileSystemLocator>
pub struct FileSystemBucketFileSystemLocator {
    pub storage_key: (),
    pub bucket_name: String,
    pub path_components: (),
}
