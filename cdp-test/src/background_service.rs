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
    pub service: Box<ServiceName>,
}
/// Enables event updates for the service.
pub type BackgroundServiceStartObservingReturns = ();
/// Disables event updates for the service.
pub struct BackgroundServiceStopObservingParams {
    pub service: Box<ServiceName>,
}
/// Disables event updates for the service.
pub type BackgroundServiceStopObservingReturns = ();
/// Set the recording state for the service.
pub struct BackgroundServiceSetRecordingParams {
    pub should_record: bool,
    pub service: Box<ServiceName>,
}
/// Set the recording state for the service.
pub type BackgroundServiceSetRecordingReturns = ();
/// Clears all stored data for the service.
pub struct BackgroundServiceClearEventsParams {
    pub service: Box<ServiceName>,
}
/// Clears all stored data for the service.
pub type BackgroundServiceClearEventsReturns = ();
/// Called when the recording state for the service has been updated.
pub struct BackgroundServiceRecordingStateChangedEvent {
    pub is_recording: bool,
    pub service: Box<ServiceName>,
}
/** Called with all existing backgroundServiceEvents when enabled, and all new
events afterwards if enabled and recording.*/
pub struct BackgroundServiceBackgroundServiceEventReceivedEvent {
    pub background_service_event: Box<BackgroundServiceEvent>,
}
