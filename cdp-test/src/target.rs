pub use crate::common::*;
pub struct TargetId(String);
/// Unique identifier of attached debugging session.
pub struct SessionId(String);
pub struct TargetInfo {
    pub target_id: (),
    pub _type: String,
    pub title: String,
    pub url: String,
    pub attached: (),
    pub opener_id: (),
    pub can_access_opener: (),
    pub opener_frame_id: (),
    pub browser_context_id: (),
    pub subtype: String,
}
/// ⚠️ Experimental
/// A filter used by target query/discovery/auto-attach operations.
pub struct FilterEntry {
    pub exclude: (),
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
