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
/// Activates (focuses) the target.
pub type TargetActivateTarget = ();
/// Attaches to the target with given id.
pub type TargetAttachToTarget = ();
/// ⚠️ Experimental
/// Attaches to the browser target, only uses flat sessionId mode.
pub type TargetAttachToBrowserTarget = ();
/// Closes the target. If the target is a page that gets closed too.
pub type TargetCloseTarget = ();
/// ⚠️ Experimental
/** Inject object to the target's main frame that provides a communication
channel with browser target.

Injected object will be available as `window[bindingName]`.

The object has the following API:
- `binding.send(json)` - a method to send messages over the remote debugging protocol
- `binding.onmessage = json => handleMessage(json)` - a callback that will be called for the protocol notifications and command responses.*/
pub type TargetExposeDevToolsProtocol = ();
/** Creates a new empty BrowserContext. Similar to an incognito profile but you can have more than
one.*/
pub type TargetCreateBrowserContext = ();
/// Returns all browser contexts created with `Target.createBrowserContext` method.
pub type TargetGetBrowserContexts = ();
/// Creates a new page.
pub type TargetCreateTarget = ();
/// Detaches session with given id.
pub type TargetDetachFromTarget = ();
/** Deletes a BrowserContext. All the belonging pages will be closed without calling their
beforeunload hooks.*/
pub type TargetDisposeBrowserContext = ();
/// ⚠️ Experimental
/// Returns information about a target.
pub type TargetGetTargetInfo = ();
/// Retrieves a list of available targets.
pub type TargetGetTargets = ();
#[deprecated]
/** Sends protocol message over session with given id.
Consider using flat mode instead; see commands attachToTarget, setAutoAttach,
and crbug.com/991325.*/
pub type TargetSendMessageToTarget = ();
/** Controls whether to automatically attach to new targets which are considered
to be directly related to this one (for example, iframes or workers).
When turned on, attaches to all existing related targets as well. When turned off,
automatically detaches from all currently attached targets.
This also clears all targets added by `autoAttachRelated` from the list of targets to watch
for creation of related targets.
You might want to call this recursively for auto-attached targets to attach
to all available targets.*/
pub type TargetSetAutoAttach = ();
/// ⚠️ Experimental
/** Adds the specified target to the list of targets that will be monitored for any related target
creation (such as child frames, child workers and new versions of service worker) and reported
through `attachedToTarget`. The specified target is also auto-attached.
This cancels the effect of any previous `setAutoAttach` and is also cancelled by subsequent
`setAutoAttach`. Only available at the Browser target.*/
pub type TargetAutoAttachRelated = ();
/** Controls whether to discover available targets and notify via
`targetCreated/targetInfoChanged/targetDestroyed` events.*/
pub type TargetSetDiscoverTargets = ();
/// ⚠️ Experimental
/** Enables target discovery for the specified locations, when `setDiscoverTargets` was set to
`true`.*/
pub type TargetSetRemoteLocations = ();
