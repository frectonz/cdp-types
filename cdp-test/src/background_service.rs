use crate::common::*;
use crate::network::*;
use crate::service_worker::*;
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
    pub key: String,
    pub value: String,
}
pub struct BackgroundServiceEvent {
    pub timestamp: Box<NetworkTimeSinceEpoch>,
    pub origin: String,
    pub service_worker_registration_id: Box<RegistrationId>,
    pub service: Box<ServiceName>,
    pub event_name: String,
    pub instance_id: String,
    pub event_metadata: Vec<EventMetadata>,
    pub storage_key: String,
}
/// Enables event updates for the service.
pub type BackgroundServiceStartObservingParams = ();
/// Enables event updates for the service.
pub type BackgroundServiceStartObservingResults = ();
/// Disables event updates for the service.
pub type BackgroundServiceStopObservingParams = ();
/// Disables event updates for the service.
pub type BackgroundServiceStopObservingResults = ();
/// Set the recording state for the service.
pub type BackgroundServiceSetRecordingParams = ();
/// Set the recording state for the service.
pub type BackgroundServiceSetRecordingResults = ();
/// Clears all stored data for the service.
pub type BackgroundServiceClearEventsParams = ();
/// Clears all stored data for the service.
pub type BackgroundServiceClearEventsResults = ();
