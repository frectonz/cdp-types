/// Players will get an ID that is unique within the agent context.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Media/#type-PlayerId>
pub struct MediaPlayerId(String);
/// <https://chromedevtools.github.io/devtools-protocol/tot/Media/#type-Timestamp>
pub struct MediaTimestamp(u64);
/** Have one type per entry in MediaLogRecord::Type
Corresponds to kMessage*/
/// <https://chromedevtools.github.io/devtools-protocol/tot/Media/#type-PlayerMessage>
pub struct MediaPlayerMessage {
    pub level: (),
    pub message: (),
}
/// Corresponds to kMediaPropertyChange
/// <https://chromedevtools.github.io/devtools-protocol/tot/Media/#type-PlayerProperty>
pub struct MediaPlayerProperty {
    pub name: (),
    pub value: (),
}
/// Corresponds to kMediaEventTriggered
/// <https://chromedevtools.github.io/devtools-protocol/tot/Media/#type-PlayerEvent>
pub struct MediaPlayerEvent {
    pub timestamp: (),
    pub value: (),
}
/** Represents logged source line numbers reported in an error.
NOTE: file and line are from chromium c++ implementation code, not js.*/
/// <https://chromedevtools.github.io/devtools-protocol/tot/Media/#type-PlayerErrorSourceLocation>
pub struct MediaPlayerErrorSourceLocation {
    pub file: (),
    pub line: (),
}
/// Corresponds to kMediaError
/// <https://chromedevtools.github.io/devtools-protocol/tot/Media/#type-PlayerError>
pub struct MediaPlayerError {
    pub error_type: (),
    pub code: (),
    pub stack: (),
    pub cause: (),
    pub data: (),
}
