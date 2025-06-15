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
pub struct CastEnableParams {
    pub presentation_url: String,
}
/** Starts observing for sinks that can be used for tab mirroring, and if set,
sinks compatible with |presentationUrl| as well. When sinks are found, a
|sinksUpdated| event is fired.
Also starts observing for issue messages. When an issue is added or removed,
an |issueUpdated| event is fired.*/
pub type CastEnableReturns = ();
/// Stops observing for sinks and issues.
pub type CastDisableParams = ();
/// Stops observing for sinks and issues.
pub type CastDisableReturns = ();
/** Sets a sink to be used when the web page requests the browser to choose a
sink via Presentation API, Remote Playback API, or Cast SDK.*/
pub struct CastSetSinkToUseParams {
    pub sink_name: String,
}
/** Sets a sink to be used when the web page requests the browser to choose a
sink via Presentation API, Remote Playback API, or Cast SDK.*/
pub type CastSetSinkToUseReturns = ();
/// Starts mirroring the desktop to the sink.
pub struct CastStartDesktopMirroringParams {
    pub sink_name: String,
}
/// Starts mirroring the desktop to the sink.
pub type CastStartDesktopMirroringReturns = ();
/// Starts mirroring the tab to the sink.
pub struct CastStartTabMirroringParams {
    pub sink_name: String,
}
/// Starts mirroring the tab to the sink.
pub type CastStartTabMirroringReturns = ();
/// Stops the active Cast session on the sink.
pub struct CastStopCastingParams {
    pub sink_name: String,
}
/// Stops the active Cast session on the sink.
pub type CastStopCastingReturns = ();
/** This is fired whenever the list of available sinks changes. A sink is a
device or a software surface that you can cast to.*/
pub struct CastSinksUpdatedEvent {
    pub sinks: Vec<Sink>,
}
/** This is fired whenever the outstanding issue/error message changes.
|issueMessage| is empty if there is no issue.*/
pub struct CastIssueUpdatedEvent {
    pub issue_message: String,
}
