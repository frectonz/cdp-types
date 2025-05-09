/** The Background Service that will be associated with the commands/events.
Every Background Service operates independently, but they share the same
API.*/
/// <https://chromedevtools.github.io/devtools-protocol/tot/BackgroundService/#type-ServiceName>
pub enum BackgroundServiceServiceName {
    BackgroundFetch,
    BackgroundSync,
    PushMessaging,
    Notifications,
    PaymentHandler,
    PeriodicBackgroundSync,
}
/// A key-value pair for additional event information to pass along.
/// <https://chromedevtools.github.io/devtools-protocol/tot/BackgroundService/#type-EventMetadata>
pub struct BackgroundServiceEventMetadata {
    pub key: (),
    pub value: (),
}
/// <https://chromedevtools.github.io/devtools-protocol/tot/BackgroundService/#type-BackgroundServiceEvent>
pub struct BackgroundServiceEvent {
    pub timestamp: (),
    pub origin: (),
    pub service_worker_registration_id: (),
    pub service: (),
    pub event_name: (),
    pub instance_id: (),
    pub event_metadata: (),
    pub storage_key: (),
}
