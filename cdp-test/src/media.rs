use crate::common::*;
/// Players will get an ID that is unique within the agent context.
pub struct PlayerId(String);
pub struct Timestamp(u64);
/** Have one type per entry in MediaLogRecord::Type
Corresponds to kMessage*/
pub struct PlayerMessage {
    pub level: String,
    pub message: String,
}
/// Corresponds to kMediaPropertyChange
pub struct PlayerProperty {
    pub name: String,
    pub value: String,
}
/// Corresponds to kMediaEventTriggered
pub struct PlayerEvent {
    pub timestamp: Box<Timestamp>,
    pub value: String,
}
/** Represents logged source line numbers reported in an error.
NOTE: file and line are from chromium c++ implementation code, not js.*/
pub struct PlayerErrorSourceLocation {
    pub file: String,
    pub line: i64,
}
/// Corresponds to kMediaError
pub struct PlayerError {
    pub error_type: String,
    pub code: i64,
    pub stack: Vec<PlayerErrorSourceLocation>,
    pub cause: Vec<PlayerError>,
    pub data: serde_json::Map<String, serde_json::Value>,
}
/// Enables the Media domain
pub type MediaEnableParams = ();
/// Enables the Media domain
pub type MediaEnableReturns = ();
/// Disables the Media domain.
pub type MediaDisableParams = ();
/// Disables the Media domain.
pub type MediaDisableReturns = ();
/** This can be called multiple times, and can be used to set / override /
remove player properties. A null propValue indicates removal.*/
pub struct MediaPlayerPropertiesChangedEvent {
    pub player_id: Box<PlayerId>,
    pub properties: Vec<PlayerProperty>,
}
/** Send events as a list, allowing them to be batched on the browser for less
congestion. If batched, events must ALWAYS be in chronological order.*/
pub struct MediaPlayerEventsAddedEvent {
    pub player_id: Box<PlayerId>,
    pub events: Vec<PlayerEvent>,
}
/// Send a list of any messages that need to be delivered.
pub struct MediaPlayerMessagesLoggedEvent {
    pub player_id: Box<PlayerId>,
    pub messages: Vec<PlayerMessage>,
}
/// Send a list of any errors that need to be delivered.
pub struct MediaPlayerErrorsRaisedEvent {
    pub player_id: Box<PlayerId>,
    pub errors: Vec<PlayerError>,
}
/** Called whenever a player is created, or when a new agent joins and receives
a list of active players. If an agent is restored, it will receive the full
list of player ids and all events again.*/
pub struct MediaPlayersCreatedEvent {
    pub players: Vec<PlayerId>,
}
