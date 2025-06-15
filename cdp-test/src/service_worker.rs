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
pub struct ServiceWorkerDeliverPushMessageParams {
    test: (),
    test: (),
    test: (),
}
pub type ServiceWorkerDeliverPushMessageReturns = ();
pub type ServiceWorkerDisableParams = ();
pub type ServiceWorkerDisableReturns = ();
pub struct ServiceWorkerDispatchSyncEventParams {
    test: (),
    test: (),
    test: (),
    test: (),
}
pub type ServiceWorkerDispatchSyncEventReturns = ();
pub struct ServiceWorkerDispatchPeriodicSyncEventParams {
    test: (),
    test: (),
    test: (),
}
pub type ServiceWorkerDispatchPeriodicSyncEventReturns = ();
pub type ServiceWorkerEnableParams = ();
pub type ServiceWorkerEnableReturns = ();
pub struct ServiceWorkerInspectWorkerParams {
    test: (),
}
pub type ServiceWorkerInspectWorkerReturns = ();
pub struct ServiceWorkerSetForceUpdateOnPageLoadParams {
    test: (),
}
pub type ServiceWorkerSetForceUpdateOnPageLoadReturns = ();
pub struct ServiceWorkerSkipWaitingParams {
    test: (),
}
pub type ServiceWorkerSkipWaitingReturns = ();
pub struct ServiceWorkerStartWorkerParams {
    test: (),
}
pub type ServiceWorkerStartWorkerReturns = ();
pub type ServiceWorkerStopAllWorkersParams = ();
pub type ServiceWorkerStopAllWorkersReturns = ();
pub struct ServiceWorkerStopWorkerParams {
    test: (),
}
pub type ServiceWorkerStopWorkerReturns = ();
pub struct ServiceWorkerUnregisterParams {
    test: (),
}
pub type ServiceWorkerUnregisterReturns = ();
pub struct ServiceWorkerUpdateRegistrationParams {
    test: (),
}
pub type ServiceWorkerUpdateRegistrationReturns = ();
