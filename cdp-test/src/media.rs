use crate::common::*;
/// Players will get an ID that is unique within the agent context.
pub struct PlayerId(String);
pub struct Timestamp(u64);
/** Have one type per entry in MediaLogRecord::Type
Corresponds to kMessage*/
pub struct PlayerMessage {
    pub level: Box<String>,
    pub message: Box<String>,
}
/// Corresponds to kMediaPropertyChange
pub struct PlayerProperty {
    pub name: Box<String>,
    pub value: Box<String>,
}
/// Corresponds to kMediaEventTriggered
pub struct PlayerEvent {
    pub timestamp: Box<Timestamp>,
    pub value: Box<String>,
}
/** Represents logged source line numbers reported in an error.
NOTE: file and line are from chromium c++ implementation code, not js.*/
pub struct PlayerErrorSourceLocation {
    pub file: Box<String>,
    pub line: Box<i64>,
}
/// Corresponds to kMediaError
pub struct PlayerError {
    pub error_type: Box<String>,
    pub code: Box<i64>,
    pub stack: (),
    pub cause: (),
    pub data: Box<serde_json::Map<String, serde_json::Value>>,
}
