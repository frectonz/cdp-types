use crate::browser::*;
/// ⚠️ Experimental
/// The state of the browser window.
pub enum BrowserWindowState {
    Normal,
    Minimized,
    Maximized,
    Fullscreen,
}
use crate::css::*;
pub struct CssComputedStyleProperty {
    pub name: String,
    pub value: String,
}
use crate::cache_storage::*;
/// Data entry.
pub struct CacheStorageDataEntry {
    pub request_url: String,
    pub request_method: String,
    pub request_headers: Vec<Header>,
    pub response_time: u64,
    pub response_status: i64,
    pub response_status_text: String,
    pub response_type: Box<CachedResponseType>,
    pub response_headers: Vec<Header>,
}
use crate::dom::*;
pub struct DomCssComputedStyleProperty {
    pub name: String,
    pub value: String,
}
use crate::dom_storage::*;
pub struct DomStorageSerializedStorageKey(String);
use crate::indexed_db::*;
/// Data entry.
pub struct IndexedDbDataEntry {
    pub key: Box<()>,
    pub primary_key: Box<()>,
    pub value: Box<()>,
}
use crate::input::*;
/// UTC time in seconds, counted from January 1, 1970.
pub struct InputTimeSinceEpoch(u64);
use crate::network::*;
/** Unique network request identifier.
Note that this does not identify individual HTTP requests that are part of
a network request.*/
pub struct NetworkRequestId(String);
use crate::network::*;
/// UTC time in seconds, counted from January 1, 1970.
pub struct NetworkTimeSinceEpoch(u64);
use crate::network::*;
/// ⚠️ Experimental
/// Authorization challenge for HTTP status code 401 or 407.
pub struct NetworkAuthChallenge {
    pub source: String,
    pub origin: String,
    pub scheme: String,
    pub realm: String,
}
use crate::network::*;
/// ⚠️ Experimental
/// Response to an AuthChallenge.
pub struct NetworkAuthChallengeResponse {
    pub response: String,
    pub username: String,
    pub password: String,
}
use crate::network::*;
/// ⚠️ Experimental
/// Request pattern for interception.
pub struct NetworkRequestPattern {
    pub url_pattern: String,
    pub resource_type: Box<ResourceType>,
    pub interception_stage: Box<InterceptionStage>,
}
use crate::page::*;
/// Javascript dialog type.
pub enum PageDialogType {
    Alert,
    Confirm,
    Prompt,
    Beforeunload,
}
use crate::page::*;
/// ⚠️ Experimental
pub struct PageFileHandler {
    pub action: String,
    pub name: String,
    pub icons: Vec<ImageResource>,
    pub accepts: Vec<FileFilter>,
    pub launch_type: String,
}
use crate::storage::*;
pub struct StorageSerializedStorageKey(String);
use crate::target::*;
/// ⚠️ Experimental
/// The state of the target window.
pub enum TargetWindowState {
    Normal,
    Minimized,
    Maximized,
    Fullscreen,
}
use crate::fetch::*;
/** Unique request identifier.
Note that this does not identify individual HTTP requests that are part of
a network request.*/
pub struct FetchRequestId(String);
use crate::fetch::*;
pub struct FetchRequestPattern {
    pub url_pattern: String,
    pub resource_type: Box<ResourceType>,
    pub request_stage: Box<RequestStage>,
}
use crate::fetch::*;
/// Authorization challenge for HTTP status code 401 or 407.
pub struct FetchAuthChallenge {
    pub source: String,
    pub origin: String,
    pub scheme: String,
    pub realm: String,
}
use crate::fetch::*;
/// Response to an AuthChallenge.
pub struct FetchAuthChallengeResponse {
    pub response: String,
    pub username: String,
    pub password: String,
}
use crate::device_access::*;
/// Device request id.
pub struct DeviceAccessRequestId(String);
use crate::fed_cm::*;
/// The types of FedCM dialogs.
pub enum FedCmDialogType {
    AccountChooser,
    AutoReauthn,
    ConfirmIdpLogin,
    Error,
}
use crate::pwa::*;
pub struct PwaFileHandler {
    pub action: String,
    pub accepts: Vec<FileHandlerAccept>,
    pub display_name: String,
}
