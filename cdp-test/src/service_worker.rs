use crate::common::*;
use crate::target::*;
pub struct RegistrationId(String);
/// ServiceWorker registration.
pub struct ServiceWorkerRegistration {
    pub registration_id: Box<RegistrationId>,
    pub scope_url: String,
    pub is_deleted: bool,
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
    pub version_id: String,
    pub registration_id: Box<RegistrationId>,
    pub script_url: String,
    pub running_status: Box<ServiceWorkerVersionRunningStatus>,
    pub status: Box<ServiceWorkerVersionStatus>,
    pub script_last_modified: u64,
    pub script_response_time: u64,
    pub controlled_clients: Vec<TargetId>,
    pub target_id: Box<TargetId>,
    pub router_rules: String,
}
/// ServiceWorker error message.
pub struct ServiceWorkerErrorMessage {
    pub error_message: String,
    pub registration_id: Box<RegistrationId>,
    pub version_id: String,
    pub source_url: String,
    pub line_number: i64,
    pub column_number: i64,
}
