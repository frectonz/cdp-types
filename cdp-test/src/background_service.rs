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
pub struct BackgroundServiceStartObservingParams {
    test: (),
}
/// Enables event updates for the service.
pub type BackgroundServiceStartObservingReturns = ();
/// Disables event updates for the service.
pub struct BackgroundServiceStopObservingParams {
    test: (),
}
/// Disables event updates for the service.
pub type BackgroundServiceStopObservingReturns = ();
/// Set the recording state for the service.
pub struct BackgroundServiceSetRecordingParams {
    test: (),
    test: (),
}
/// Set the recording state for the service.
pub type BackgroundServiceSetRecordingReturns = ();
/// Clears all stored data for the service.
pub struct BackgroundServiceClearEventsParams {
    test: (),
}
/// Clears all stored data for the service.
pub type BackgroundServiceClearEventsReturns = ();
