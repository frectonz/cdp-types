use crate::common::*;
/** The Background Service that will be associated with the commands/events.
Every Background Service operates independently, but they share the same
API.*/
pub enum ServiceName {
    BackgroundFetch,
    BackgroundSync,
    PushMessaging,
    Notifications,
    PaymentHandler,
    PeriodicBackgroundSync,
}
/// A key-value pair for additional event information to pass along.
pub struct EventMetadata {
    pub key: Box<String>,
    pub value: Box<String>,
}
pub struct BackgroundServiceEvent {
    pub timestamp: Box<NetworkTimeSinceEpoch>,
    pub origin: Box<String>,
    pub service_worker_registration_id: Box<ServiceWorkerRegistrationId>,
    pub service: Box<ServiceName>,
    pub event_name: Box<String>,
    pub instance_id: Box<String>,
    pub event_metadata: (),
    pub storage_key: Box<String>,
}
