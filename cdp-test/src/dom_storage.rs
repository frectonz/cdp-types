/// <https://chromedevtools.github.io/devtools-protocol/tot/DOMStorage/#type-SerializedStorageKey>
pub struct DomStorageSerializedStorageKey(String);
/// DOM Storage identifier.
/// <https://chromedevtools.github.io/devtools-protocol/tot/DOMStorage/#type-StorageId>
pub struct DomStorageStorageId {
    pub security_origin: (),
    pub storage_key: (),
    pub is_local_storage: (),
}
/// DOM Storage item.
/// <https://chromedevtools.github.io/devtools-protocol/tot/DOMStorage/#type-Item>
pub struct DomStorageItem(Vec<String>);
