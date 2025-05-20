use crate::common::*;
use crate::page::*;
use crate::browser::*;
pub struct TargetId(String);
/// Unique identifier of attached debugging session.
pub struct SessionId(String);
pub struct TargetInfo {
    pub target_id: Box<TargetId>,
    pub _type: String,
    pub title: String,
    pub url: String,
    pub attached: bool,
    pub opener_id: Box<TargetId>,
    pub can_access_opener: bool,
    pub opener_frame_id: Box<crate::page::FrameId>,
    pub browser_context_id: Box<BrowserContextId>,
    pub subtype: String,
}
/// ⚠️ Experimental
/// A filter used by target query/discovery/auto-attach operations.
pub struct FilterEntry {
    pub exclude: bool,
    pub _type: String,
}
/// ⚠️ Experimental
/** The entries in TargetFilter are matched sequentially against targets and
the first entry that matches determines if the target is included or not,
depending on the value of `exclude` field in the entry.
If filter is not specified, the one assumed is
[{type: "browser", exclude: true}, {type: "tab", exclude: true}, {}]
(i.e. include everything but `browser` and `tab`).*/
pub struct TargetFilter(Vec<FilterEntry>);
/// ⚠️ Experimental
pub struct RemoteLocation {
    pub host: String,
    pub port: i64,
}
pub type TargetActivateTarget = ();
pub type TargetAttachToTarget = ();
pub type TargetAttachToBrowserTarget = ();
pub type TargetCloseTarget = ();
pub type TargetExposeDevToolsProtocol = ();
pub type TargetCreateBrowserContext = ();
pub type TargetGetBrowserContexts = ();
pub type TargetCreateTarget = ();
pub type TargetDetachFromTarget = ();
pub type TargetDisposeBrowserContext = ();
pub type TargetGetTargetInfo = ();
pub type TargetGetTargets = ();
pub type TargetSendMessageToTarget = ();
pub type TargetSetAutoAttach = ();
pub type TargetAutoAttachRelated = ();
pub type TargetSetDiscoverTargets = ();
pub type TargetSetRemoteLocations = ();
