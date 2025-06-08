use crate::common::*;
pub struct Sink {
    pub name: String,
    pub id: String,
    pub session: String,
}
/** Starts observing for sinks that can be used for tab mirroring, and if set,
sinks compatible with |presentationUrl| as well. When sinks are found, a
|sinksUpdated| event is fired.
Also starts observing for issue messages. When an issue is added or removed,
an |issueUpdated| event is fired.*/
pub type CastEnableParams = ();
/** Starts observing for sinks that can be used for tab mirroring, and if set,
sinks compatible with |presentationUrl| as well. When sinks are found, a
|sinksUpdated| event is fired.
Also starts observing for issue messages. When an issue is added or removed,
an |issueUpdated| event is fired.*/
pub type CastEnableResults = ();
/// Stops observing for sinks and issues.
pub type CastDisableParams = ();
/// Stops observing for sinks and issues.
pub type CastDisableResults = ();
/** Sets a sink to be used when the web page requests the browser to choose a
sink via Presentation API, Remote Playback API, or Cast SDK.*/
pub type CastSetSinkToUseParams = ();
/** Sets a sink to be used when the web page requests the browser to choose a
sink via Presentation API, Remote Playback API, or Cast SDK.*/
pub type CastSetSinkToUseResults = ();
/// Starts mirroring the desktop to the sink.
pub type CastStartDesktopMirroringParams = ();
/// Starts mirroring the desktop to the sink.
pub type CastStartDesktopMirroringResults = ();
/// Starts mirroring the tab to the sink.
pub type CastStartTabMirroringParams = ();
/// Starts mirroring the tab to the sink.
pub type CastStartTabMirroringResults = ();
/// Stops the active Cast session on the sink.
pub type CastStopCastingParams = ();
/// Stops the active Cast session on the sink.
pub type CastStopCastingResults = ();
