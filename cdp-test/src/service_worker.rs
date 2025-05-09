use crate::target::*;
/// <https://chromedevtools.github.io/devtools-protocol/tot/ServiceWorker/#type-RegistrationID>
pub struct ServiceWorkerRegistrationId(String);
/// ServiceWorker registration.
/// <https://chromedevtools.github.io/devtools-protocol/tot/ServiceWorker/#type-ServiceWorkerRegistration>
pub struct ServiceWorkerRegistration {
    pub registration_id: (),
    pub scope_url: (),
    pub is_deleted: (),
}
/// <https://chromedevtools.github.io/devtools-protocol/tot/ServiceWorker/#type-ServiceWorkerVersionRunningStatus>
pub enum ServiceWorkerVersionRunningStatus {
    Stopped,
    Starting,
    Running,
    Stopping,
}
/// <https://chromedevtools.github.io/devtools-protocol/tot/ServiceWorker/#type-ServiceWorkerVersionStatus>
pub enum ServiceWorkerVersionStatus {
    New,
    Installing,
    Installed,
    Activating,
    Activated,
    Redundant,
}
/// ServiceWorker version.
/// <https://chromedevtools.github.io/devtools-protocol/tot/ServiceWorker/#type-ServiceWorkerVersion>
pub struct ServiceWorkerVersion {
    pub version_id: (),
    pub registration_id: (),
    pub script_url: (),
    pub running_status: (),
    pub status: (),
    pub script_last_modified: (),
    pub script_response_time: (),
    pub controlled_clients: (),
    pub target_id: (),
    pub router_rules: (),
}
/// ServiceWorker error message.
/// <https://chromedevtools.github.io/devtools-protocol/tot/ServiceWorker/#type-ServiceWorkerErrorMessage>
pub struct ServiceWorkerErrorMessage {
    pub error_message: (),
    pub registration_id: (),
    pub version_id: (),
    pub source_url: (),
    pub line_number: (),
    pub column_number: (),
}
