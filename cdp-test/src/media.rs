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
