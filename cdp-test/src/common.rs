/// ⚠️ Experimental
/// The state of the browser window.
pub enum BrowserWindowState {
    Normal,
    Minimized,
    Maximized,
    Fullscreen,
}
pub struct CsscssComputedStyleProperty {
    pub name: String,
    pub value: String,
}
/// Data entry.
pub struct CacheStorageDataEntry {
    pub request_url: String,
    pub request_method: String,
    pub request_headers: (),
    pub response_time: u64,
    pub response_status: i64,
    pub response_status_text: String,
    pub response_type: (),
    pub response_headers: (),
}
pub struct DomcssComputedStyleProperty {
    pub name: String,
    pub value: String,
}
pub struct DomStorageSerializedStorageKey(String);
/// Data entry.
pub struct IndexedDbDataEntry {
    pub key: (),
    pub primary_key: (),
    pub value: (),
}
/// UTC time in seconds, counted from January 1, 1970.
pub struct InputTimeSinceEpoch(u64);
/** Unique network request identifier.
Note that this does not identify individual HTTP requests that are part of
a network request.*/
pub struct NetworkRequestId(String);
/// UTC time in seconds, counted from January 1, 1970.
pub struct NetworkTimeSinceEpoch(u64);
/// ⚠️ Experimental
/// Authorization challenge for HTTP status code 401 or 407.
pub struct NetworkAuthChallenge {
    pub source: String,
    pub origin: String,
    pub scheme: String,
    pub realm: String,
}
/// ⚠️ Experimental
/// Response to an AuthChallenge.
pub struct NetworkAuthChallengeResponse {
    pub response: String,
    pub username: String,
    pub password: String,
}
/// ⚠️ Experimental
/// Request pattern for interception.
pub struct NetworkRequestPattern {
    pub url_pattern: String,
    pub resource_type: (),
    pub interception_stage: (),
}
/// Javascript dialog type.
pub enum PageDialogType {
    Alert,
    Confirm,
    Prompt,
    Beforeunload,
}
/// ⚠️ Experimental
pub struct PageFileHandler {
    pub action: String,
    pub name: String,
    pub icons: (),
    pub accepts: (),
    pub launch_type: String,
}
pub struct StorageSerializedStorageKey(String);
/// ⚠️ Experimental
/// The state of the target window.
pub enum TargetWindowState {
    Normal,
    Minimized,
    Maximized,
    Fullscreen,
}
/** Unique request identifier.
Note that this does not identify individual HTTP requests that are part of
a network request.*/
pub struct FetchRequestId(String);
pub struct FetchRequestPattern {
    pub url_pattern: String,
    pub resource_type: (),
    pub request_stage: (),
}
/// Authorization challenge for HTTP status code 401 or 407.
pub struct FetchAuthChallenge {
    pub source: String,
    pub origin: String,
    pub scheme: String,
    pub realm: String,
}
/// Response to an AuthChallenge.
pub struct FetchAuthChallengeResponse {
    pub response: String,
    pub username: String,
    pub password: String,
}
/// Device request id.
pub struct DeviceAccessRequestId(String);
/// The types of FedCM dialogs.
pub enum FedCmDialogType {
    AccountChooser,
    AutoReauthn,
    ConfirmIdpLogin,
    Error,
}
pub struct PwaFileHandler {
    pub action: String,
    pub accepts: (),
    pub display_name: String,
}
