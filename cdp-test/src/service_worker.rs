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
    pub controlled_clients: Vec<crate::target::TargetId>,
    pub target_id: Box<crate::target::TargetId>,
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
pub struct ServiceWorkerDeliverPushMessageParams {
    pub origin: String,
    pub registration_id: Box<RegistrationId>,
    pub data: String,
}
pub type ServiceWorkerDeliverPushMessageReturns = ();
pub type ServiceWorkerDisableParams = ();
pub type ServiceWorkerDisableReturns = ();
pub struct ServiceWorkerDispatchSyncEventParams {
    pub origin: String,
    pub registration_id: Box<RegistrationId>,
    pub tag: String,
    pub last_chance: bool,
}
pub type ServiceWorkerDispatchSyncEventReturns = ();
pub struct ServiceWorkerDispatchPeriodicSyncEventParams {
    pub origin: String,
    pub registration_id: Box<RegistrationId>,
    pub tag: String,
}
pub type ServiceWorkerDispatchPeriodicSyncEventReturns = ();
pub type ServiceWorkerEnableParams = ();
pub type ServiceWorkerEnableReturns = ();
pub struct ServiceWorkerInspectWorkerParams {
    pub version_id: String,
}
pub type ServiceWorkerInspectWorkerReturns = ();
pub struct ServiceWorkerSetForceUpdateOnPageLoadParams {
    pub force_update_on_page_load: bool,
}
pub type ServiceWorkerSetForceUpdateOnPageLoadReturns = ();
pub struct ServiceWorkerSkipWaitingParams {
    pub scope_url: String,
}
pub type ServiceWorkerSkipWaitingReturns = ();
pub struct ServiceWorkerStartWorkerParams {
    pub scope_url: String,
}
pub type ServiceWorkerStartWorkerReturns = ();
pub type ServiceWorkerStopAllWorkersParams = ();
pub type ServiceWorkerStopAllWorkersReturns = ();
pub struct ServiceWorkerStopWorkerParams {
    pub version_id: String,
}
pub type ServiceWorkerStopWorkerReturns = ();
pub struct ServiceWorkerUnregisterParams {
    pub scope_url: String,
}
pub type ServiceWorkerUnregisterReturns = ();
pub struct ServiceWorkerUpdateRegistrationParams {
    pub scope_url: String,
}
pub type ServiceWorkerUpdateRegistrationReturns = ();
