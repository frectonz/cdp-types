use crate::common::*;
use crate::target::*;
pub struct RegistrationId(String);
/// ServiceWorker registration.
pub struct ServiceWorkerRegistration {
    pub registration_id: Box<RegistrationId>,
    pub scope_url: Box<String>,
    pub is_deleted: (),
}
pub enum ServiceWorkerVersionRunningStatus {
    Stopped,
    Starting,
    Running,
    Stopping,
}
pub enum ServiceWorkerVersionStatus {
    New,
    Installing,
    Installed,
    Activating,
    Activated,
    Redundant,
}
/// ServiceWorker version.
pub struct ServiceWorkerVersion {
    pub version_id: Box<String>,
    pub registration_id: Box<RegistrationId>,
    pub script_url: Box<String>,
    pub running_status: Box<ServiceWorkerVersionRunningStatus>,
    pub status: Box<ServiceWorkerVersionStatus>,
    pub script_last_modified: Box<u64>,
    pub script_response_time: Box<u64>,
    pub controlled_clients: (),
    pub target_id: Box<TargetId>,
    pub router_rules: Box<String>,
}
/// ServiceWorker error message.
pub struct ServiceWorkerErrorMessage {
    pub error_message: Box<String>,
    pub registration_id: Box<RegistrationId>,
    pub version_id: Box<String>,
    pub source_url: Box<String>,
    pub line_number: Box<i64>,
    pub column_number: Box<i64>,
}
