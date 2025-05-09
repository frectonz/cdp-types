use crate::network::*;
use crate::storage::*;
/// <https://chromedevtools.github.io/devtools-protocol/tot/FileSystem/#type-File>
pub struct FileSystemFile {
    pub name: (),
    pub last_modified: (),
    pub size: (),
    pub _type: (),
}
/// <https://chromedevtools.github.io/devtools-protocol/tot/FileSystem/#type-Directory>
pub struct FileSystemDirectory {
    pub name: (),
    pub nested_directories: (),
    pub nested_files: (),
}
/// <https://chromedevtools.github.io/devtools-protocol/tot/FileSystem/#type-BucketFileSystemLocator>
pub struct FileSystemBucketFileSystemLocator {
    pub storage_key: (),
    pub bucket_name: (),
    pub path_components: (),
}
