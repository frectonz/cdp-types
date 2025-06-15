use crate::common::*;
use crate::page::*;
/// Unique DOM node identifier.
pub struct NodeId(i64);
/** Unique DOM node identifier used to reference a node that may not have been pushed to the
front-end.*/
pub struct BackendNodeId(i64);
/// Backend node with a friendly name.
pub struct BackendNode {
    pub node_type: i64,
    pub node_name: String,
    pub backend_node_id: Box<BackendNodeId>,
}
/// Pseudo element type.
pub enum PseudoType {
    FirstLine,
    FirstLetter,
    Checkmark,
    Before,
    After,
    PickerIcon,
    Marker,
    Backdrop,
    Column,
    Selection,
    SearchText,
    TargetText,
    SpellingError,
    GrammarError,
    Highlight,
    FirstLineInherited,
    ScrollMarker,
    ScrollMarkerGroup,
    ScrollButton,
    Scrollbar,
    ScrollbarThumb,
    ScrollbarButton,
    ScrollbarTrack,
    ScrollbarTrackPiece,
    ScrollbarCorner,
    Resizer,
    InputListButton,
    ViewTransition,
    ViewTransitionGroup,
    ViewTransitionImagePair,
    ViewTransitionOld,
    ViewTransitionNew,
    Placeholder,
    FileSelectorButton,
    DetailsContent,
    Picker,
}
/// Shadow root type.
pub enum ShadowRootType {
    UserAgent,
    Open,
    Closed,
}
/// Document compatibility mode.
pub enum CompatibilityMode {
    QuirksMode,
    LimitedQuirksMode,
    NoQuirksMode,
}
/// ContainerSelector physical axes
pub enum PhysicalAxes {
    Horizontal,
    Vertical,
    Both,
}
/// ContainerSelector logical axes
pub enum LogicalAxes {
    Inline,
    Block,
    Both,
}
/// Physical scroll orientation
pub enum ScrollOrientation {
    Horizontal,
    Vertical,
}
/** DOM interaction is implemented in terms of mirror objects that represent the actual DOM nodes.
DOMNode is a base node mirror type.*/
pub struct Node {
    pub node_id: Box<NodeId>,
    pub parent_id: Box<NodeId>,
    pub backend_node_id: Box<BackendNodeId>,
    pub node_type: i64,
    pub node_name: String,
    pub local_name: String,
    pub node_value: String,
    pub child_node_count: i64,
    pub children: Vec<Node>,
    pub attributes: Vec<String>,
    pub document_url: String,
    pub base_url: String,
    pub public_id: String,
    pub system_id: String,
    pub internal_subset: String,
    pub xml_version: String,
    pub name: String,
    pub value: String,
    pub pseudo_type: Box<PseudoType>,
    pub pseudo_identifier: String,
    pub shadow_root_type: Box<ShadowRootType>,
    pub frame_id: Box<crate::page::FrameId>,
    pub content_document: Box<Node>,
    pub shadow_roots: Vec<Node>,
    pub template_content: Box<Node>,
    pub pseudo_elements: Vec<Node>,
    pub imported_document: Box<Node>,
    pub distributed_nodes: Vec<BackendNode>,
    pub is_svg: bool,
    pub compatibility_mode: Box<CompatibilityMode>,
    pub assigned_slot: Box<BackendNode>,
    pub is_scrollable: bool,
}
/// A structure to hold the top-level node of a detached tree and an array of its retained descendants.
pub struct DetachedElementInfo {
    pub tree_node: Box<Node>,
    pub retained_node_ids: Vec<NodeId>,
}
/// A structure holding an RGBA color.
pub struct Rgba {
    pub r: i64,
    pub g: i64,
    pub b: i64,
    pub a: u64,
}
/// An array of quad vertices, x immediately followed by y for each point, points clock-wise.
pub struct Quad(Vec<u64>);
/// Box model.
pub struct BoxModel {
    pub content: Box<Quad>,
    pub padding: Box<Quad>,
    pub border: Box<Quad>,
    pub margin: Box<Quad>,
    pub width: i64,
    pub height: i64,
    pub shape_outside: Box<ShapeOutsideInfo>,
}
/// CSS Shape Outside details.
pub struct ShapeOutsideInfo {
    pub bounds: Box<Quad>,
    pub shape: Vec<serde_json::Value>,
    pub margin_shape: Vec<serde_json::Value>,
}
/// Rectangle.
pub struct Rect {
    pub x: u64,
    pub y: u64,
    pub width: u64,
    pub height: u64,
}
/// ⚠️ Experimental
/// Collects class names for the node with given id and all of it's child nodes.
pub struct DomCollectClassNamesFromSubtreeParams {
    pub node_id: Box<NodeId>,
}
/// ⚠️ Experimental
/// Collects class names for the node with given id and all of it's child nodes.
pub type DomCollectClassNamesFromSubtreeReturns = ();
/// ⚠️ Experimental
/** Creates a deep copy of the specified node and places it into the target container before the
given anchor.*/
pub struct DomCopyToParams {
    pub node_id: Box<NodeId>,
    pub target_node_id: Box<NodeId>,
    pub insert_before_node_id: Box<NodeId>,
}
/// ⚠️ Experimental
/** Creates a deep copy of the specified node and places it into the target container before the
given anchor.*/
pub type DomCopyToReturns = ();
/** Describes node given its id, does not require domain to be enabled. Does not start tracking any
objects, can be used for automation.*/
pub struct DomDescribeNodeParams {
    pub node_id: Box<NodeId>,
    pub backend_node_id: Box<BackendNodeId>,
    pub object_id: Box<()>,
    pub depth: i64,
    pub pierce: bool,
}
/** Describes node given its id, does not require domain to be enabled. Does not start tracking any
objects, can be used for automation.*/
pub type DomDescribeNodeReturns = ();
/** Scrolls the specified rect of the given node into view if not already visible.
Note: exactly one between nodeId, backendNodeId and objectId should be passed
to identify the node.*/
pub struct DomScrollIntoViewIfNeededParams {
    pub node_id: Box<NodeId>,
    pub backend_node_id: Box<BackendNodeId>,
    pub object_id: Box<()>,
    pub rect: Box<Rect>,
}
/** Scrolls the specified rect of the given node into view if not already visible.
Note: exactly one between nodeId, backendNodeId and objectId should be passed
to identify the node.*/
pub type DomScrollIntoViewIfNeededReturns = ();
/// Disables DOM agent for the given page.
pub type DomDisableParams = ();
/// Disables DOM agent for the given page.
pub type DomDisableReturns = ();
/// ⚠️ Experimental
/** Discards search results from the session with the given id. `getSearchResults` should no longer
be called for that search.*/
pub struct DomDiscardSearchResultsParams {
    pub search_id: String,
}
/// ⚠️ Experimental
/** Discards search results from the session with the given id. `getSearchResults` should no longer
be called for that search.*/
pub type DomDiscardSearchResultsReturns = ();
/// Enables DOM agent for the given page.
pub struct DomEnableParams {
    pub include_whitespace: String,
}
/// Enables DOM agent for the given page.
pub type DomEnableReturns = ();
/// Focuses the given element.
pub struct DomFocusParams {
    pub node_id: Box<NodeId>,
    pub backend_node_id: Box<BackendNodeId>,
    pub object_id: Box<()>,
}
/// Focuses the given element.
pub type DomFocusReturns = ();
/// Returns attributes for the specified node.
pub struct DomGetAttributesParams {
    pub node_id: Box<NodeId>,
}
/// Returns attributes for the specified node.
pub type DomGetAttributesReturns = ();
/// Returns boxes for the given node.
pub struct DomGetBoxModelParams {
    pub node_id: Box<NodeId>,
    pub backend_node_id: Box<BackendNodeId>,
    pub object_id: Box<()>,
}
/// Returns boxes for the given node.
pub type DomGetBoxModelReturns = ();
/// ⚠️ Experimental
/** Returns quads that describe node position on the page. This method
might return multiple quads for inline nodes.*/
pub struct DomGetContentQuadsParams {
    pub node_id: Box<NodeId>,
    pub backend_node_id: Box<BackendNodeId>,
    pub object_id: Box<()>,
}
/// ⚠️ Experimental
/** Returns quads that describe node position on the page. This method
might return multiple quads for inline nodes.*/
pub type DomGetContentQuadsReturns = ();
/** Returns the root DOM node (and optionally the subtree) to the caller.
Implicitly enables the DOM domain events for the current target.*/
pub struct DomGetDocumentParams {
    pub depth: i64,
    pub pierce: bool,
}
/** Returns the root DOM node (and optionally the subtree) to the caller.
Implicitly enables the DOM domain events for the current target.*/
pub type DomGetDocumentReturns = ();
#[deprecated]
/** Returns the root DOM node (and optionally the subtree) to the caller.
Deprecated, as it is not designed to work well with the rest of the DOM agent.
Use DOMSnapshot.captureSnapshot instead.*/
pub struct DomGetFlattenedDocumentParams {
    pub depth: i64,
    pub pierce: bool,
}
#[deprecated]
/** Returns the root DOM node (and optionally the subtree) to the caller.
Deprecated, as it is not designed to work well with the rest of the DOM agent.
Use DOMSnapshot.captureSnapshot instead.*/
pub type DomGetFlattenedDocumentReturns = ();
/// ⚠️ Experimental
/// Finds nodes with a given computed style in a subtree.
pub struct DomGetNodesForSubtreeByStyleParams {
    pub node_id: Box<NodeId>,
    pub computed_styles: Vec<CssComputedStyleProperty>,
    pub pierce: bool,
}
/// ⚠️ Experimental
/// Finds nodes with a given computed style in a subtree.
pub type DomGetNodesForSubtreeByStyleReturns = ();
/** Returns node id at given location. Depending on whether DOM domain is enabled, nodeId is
either returned or not.*/
pub struct DomGetNodeForLocationParams {
    pub x: i64,
    pub y: i64,
    pub include_user_agent_shadow_dom: bool,
    pub ignore_pointer_events_none: bool,
}
/** Returns node id at given location. Depending on whether DOM domain is enabled, nodeId is
either returned or not.*/
pub type DomGetNodeForLocationReturns = ();
/// Returns node's HTML markup.
pub struct DomGetOuterHtmlParams {
    pub node_id: Box<NodeId>,
    pub backend_node_id: Box<BackendNodeId>,
    pub object_id: Box<()>,
}
/// Returns node's HTML markup.
pub type DomGetOuterHtmlReturns = ();
/// ⚠️ Experimental
/// Returns the id of the nearest ancestor that is a relayout boundary.
pub struct DomGetRelayoutBoundaryParams {
    pub node_id: Box<NodeId>,
}
/// ⚠️ Experimental
/// Returns the id of the nearest ancestor that is a relayout boundary.
pub type DomGetRelayoutBoundaryReturns = ();
/// ⚠️ Experimental
/** Returns search results from given `fromIndex` to given `toIndex` from the search with the given
identifier.*/
pub struct DomGetSearchResultsParams {
    pub search_id: String,
    pub from_index: i64,
    pub to_index: i64,
}
/// ⚠️ Experimental
/** Returns search results from given `fromIndex` to given `toIndex` from the search with the given
identifier.*/
pub type DomGetSearchResultsReturns = ();
/// Hides any highlight.
pub type DomHideHighlightParams = crate::overlay::OverlayHideHighlightParams;
/// Hides any highlight.
pub type DomHideHighlightReturns = crate::overlay::OverlayHideHighlightReturns;
/// Highlights DOM node.
pub type DomHighlightNodeParams = crate::overlay::OverlayHighlightNodeParams;
/// Highlights DOM node.
pub type DomHighlightNodeReturns = crate::overlay::OverlayHighlightNodeReturns;
/// Highlights given rectangle.
pub type DomHighlightRectParams = crate::overlay::OverlayHighlightRectParams;
/// Highlights given rectangle.
pub type DomHighlightRectReturns = crate::overlay::OverlayHighlightRectReturns;
/// ⚠️ Experimental
/// Marks last undoable state.
pub type DomMarkUndoableStateParams = ();
/// ⚠️ Experimental
/// Marks last undoable state.
pub type DomMarkUndoableStateReturns = ();
/// Moves node into the new container, places it before the given anchor.
pub struct DomMoveToParams {
    pub node_id: Box<NodeId>,
    pub target_node_id: Box<NodeId>,
    pub insert_before_node_id: Box<NodeId>,
}
/// Moves node into the new container, places it before the given anchor.
pub type DomMoveToReturns = ();
/// ⚠️ Experimental
/** Searches for a given string in the DOM tree. Use `getSearchResults` to access search results or
`cancelSearch` to end this search session.*/
pub struct DomPerformSearchParams {
    pub query: String,
    pub include_user_agent_shadow_dom: bool,
}
/// ⚠️ Experimental
/** Searches for a given string in the DOM tree. Use `getSearchResults` to access search results or
`cancelSearch` to end this search session.*/
pub type DomPerformSearchReturns = ();
/// ⚠️ Experimental
/// Requests that the node is sent to the caller given its path. // FIXME, use XPath
pub struct DomPushNodeByPathToFrontendParams {
    pub path: String,
}
/// ⚠️ Experimental
/// Requests that the node is sent to the caller given its path. // FIXME, use XPath
pub type DomPushNodeByPathToFrontendReturns = ();
/// ⚠️ Experimental
/// Requests that a batch of nodes is sent to the caller given their backend node ids.
pub struct DomPushNodesByBackendIdsToFrontendParams {
    pub backend_node_ids: Vec<BackendNodeId>,
}
/// ⚠️ Experimental
/// Requests that a batch of nodes is sent to the caller given their backend node ids.
pub type DomPushNodesByBackendIdsToFrontendReturns = ();
/// Executes `querySelector` on a given node.
pub struct DomQuerySelectorParams {
    pub node_id: Box<NodeId>,
    pub selector: String,
}
/// Executes `querySelector` on a given node.
pub type DomQuerySelectorReturns = ();
/// Executes `querySelectorAll` on a given node.
pub struct DomQuerySelectorAllParams {
    pub node_id: Box<NodeId>,
    pub selector: String,
}
/// Executes `querySelectorAll` on a given node.
pub type DomQuerySelectorAllReturns = ();
/// ⚠️ Experimental
/** Returns NodeIds of current top layer elements.
Top layer is rendered closest to the user within a viewport, therefore its elements always
appear on top of all other content.*/
pub type DomGetTopLayerElementsParams = ();
/// ⚠️ Experimental
/** Returns NodeIds of current top layer elements.
Top layer is rendered closest to the user within a viewport, therefore its elements always
appear on top of all other content.*/
pub type DomGetTopLayerElementsReturns = ();
/// ⚠️ Experimental
/// Returns the NodeId of the matched element according to certain relations.
pub struct DomGetElementByRelationParams {
    pub node_id: Box<NodeId>,
    pub relation: String,
}
/// ⚠️ Experimental
/// Returns the NodeId of the matched element according to certain relations.
pub type DomGetElementByRelationReturns = ();
/// ⚠️ Experimental
/// Re-does the last undone action.
pub type DomRedoParams = ();
/// ⚠️ Experimental
/// Re-does the last undone action.
pub type DomRedoReturns = ();
/// Removes attribute with given name from an element with given id.
pub struct DomRemoveAttributeParams {
    pub node_id: Box<NodeId>,
    pub name: String,
}
/// Removes attribute with given name from an element with given id.
pub type DomRemoveAttributeReturns = ();
/// Removes node with given id.
pub struct DomRemoveNodeParams {
    pub node_id: Box<NodeId>,
}
/// Removes node with given id.
pub type DomRemoveNodeReturns = ();
/** Requests that children of the node with given id are returned to the caller in form of
`setChildNodes` events where not only immediate children are retrieved, but all children down to
the specified depth.*/
pub struct DomRequestChildNodesParams {
    pub node_id: Box<NodeId>,
    pub depth: i64,
    pub pierce: bool,
}
/** Requests that children of the node with given id are returned to the caller in form of
`setChildNodes` events where not only immediate children are retrieved, but all children down to
the specified depth.*/
pub type DomRequestChildNodesReturns = ();
/** Requests that the node is sent to the caller given the JavaScript node object reference. All
nodes that form the path from the node to the root are also sent to the client as a series of
`setChildNodes` notifications.*/
pub struct DomRequestNodeParams {
    pub object_id: Box<()>,
}
/** Requests that the node is sent to the caller given the JavaScript node object reference. All
nodes that form the path from the node to the root are also sent to the client as a series of
`setChildNodes` notifications.*/
pub type DomRequestNodeReturns = ();
/// Resolves the JavaScript node object for a given NodeId or BackendNodeId.
pub struct DomResolveNodeParams {
    pub node_id: Box<NodeId>,
    pub backend_node_id: Box<BackendNodeId>,
    pub object_group: String,
    pub execution_context_id: Box<()>,
}
/// Resolves the JavaScript node object for a given NodeId or BackendNodeId.
pub type DomResolveNodeReturns = ();
/// Sets attribute for an element with given id.
pub struct DomSetAttributeValueParams {
    pub node_id: Box<NodeId>,
    pub name: String,
    pub value: String,
}
/// Sets attribute for an element with given id.
pub type DomSetAttributeValueReturns = ();
/** Sets attributes on element with given id. This method is useful when user edits some existing
attribute value and types in several attribute name/value pairs.*/
pub struct DomSetAttributesAsTextParams {
    pub node_id: Box<NodeId>,
    pub text: String,
    pub name: String,
}
/** Sets attributes on element with given id. This method is useful when user edits some existing
attribute value and types in several attribute name/value pairs.*/
pub type DomSetAttributesAsTextReturns = ();
/// Sets files for the given file input element.
pub struct DomSetFileInputFilesParams {
    pub files: Vec<String>,
    pub node_id: Box<NodeId>,
    pub backend_node_id: Box<BackendNodeId>,
    pub object_id: Box<()>,
}
/// Sets files for the given file input element.
pub type DomSetFileInputFilesReturns = ();
/// ⚠️ Experimental
/// Sets if stack traces should be captured for Nodes. See `Node.getNodeStackTraces`. Default is disabled.
pub struct DomSetNodeStackTracesEnabledParams {
    pub enable: bool,
}
/// ⚠️ Experimental
/// Sets if stack traces should be captured for Nodes. See `Node.getNodeStackTraces`. Default is disabled.
pub type DomSetNodeStackTracesEnabledReturns = ();
/// ⚠️ Experimental
/// Gets stack traces associated with a Node. As of now, only provides stack trace for Node creation.
pub struct DomGetNodeStackTracesParams {
    pub node_id: Box<NodeId>,
}
/// ⚠️ Experimental
/// Gets stack traces associated with a Node. As of now, only provides stack trace for Node creation.
pub type DomGetNodeStackTracesReturns = ();
/// ⚠️ Experimental
/** Returns file information for the given
File wrapper.*/
pub struct DomGetFileInfoParams {
    pub object_id: Box<()>,
}
/// ⚠️ Experimental
/** Returns file information for the given
File wrapper.*/
pub type DomGetFileInfoReturns = ();
/// ⚠️ Experimental
/// Returns list of detached nodes
pub type DomGetDetachedDomNodesParams = ();
/// ⚠️ Experimental
/// Returns list of detached nodes
pub type DomGetDetachedDomNodesReturns = ();
/// ⚠️ Experimental
/** Enables console to refer to the node with given id via $x (see Command Line API for more details
$x functions).*/
pub struct DomSetInspectedNodeParams {
    pub node_id: Box<NodeId>,
}
/// ⚠️ Experimental
/** Enables console to refer to the node with given id via $x (see Command Line API for more details
$x functions).*/
pub type DomSetInspectedNodeReturns = ();
/// Sets node name for a node with given id.
pub struct DomSetNodeNameParams {
    pub node_id: Box<NodeId>,
    pub name: String,
}
/// Sets node name for a node with given id.
pub type DomSetNodeNameReturns = ();
/// Sets node value for a node with given id.
pub struct DomSetNodeValueParams {
    pub node_id: Box<NodeId>,
    pub value: String,
}
/// Sets node value for a node with given id.
pub type DomSetNodeValueReturns = ();
/// Sets node HTML markup, returns new node id.
pub struct DomSetOuterHtmlParams {
    pub node_id: Box<NodeId>,
    pub outer_html: String,
}
/// Sets node HTML markup, returns new node id.
pub type DomSetOuterHtmlReturns = ();
/// ⚠️ Experimental
/// Undoes the last performed action.
pub type DomUndoParams = ();
/// ⚠️ Experimental
/// Undoes the last performed action.
pub type DomUndoReturns = ();
/// ⚠️ Experimental
/// Returns iframe node that owns iframe with the given domain.
pub struct DomGetFrameOwnerParams {
    pub frame_id: Box<crate::page::FrameId>,
}
/// ⚠️ Experimental
/// Returns iframe node that owns iframe with the given domain.
pub type DomGetFrameOwnerReturns = ();
/// ⚠️ Experimental
/** Returns the query container of the given node based on container query
conditions: containerName, physical and logical axes, and whether it queries
scroll-state. If no axes are provided and queriesScrollState is false, the
style container is returned, which is the direct parent or the closest
element with a matching container-name.*/
pub struct DomGetContainerForNodeParams {
    pub node_id: Box<NodeId>,
    pub container_name: String,
    pub physical_axes: Box<PhysicalAxes>,
    pub logical_axes: Box<LogicalAxes>,
    pub queries_scroll_state: bool,
}
/// ⚠️ Experimental
/** Returns the query container of the given node based on container query
conditions: containerName, physical and logical axes, and whether it queries
scroll-state. If no axes are provided and queriesScrollState is false, the
style container is returned, which is the direct parent or the closest
element with a matching container-name.*/
pub type DomGetContainerForNodeReturns = ();
/// ⚠️ Experimental
/** Returns the descendants of a container query container that have
container queries against this container.*/
pub struct DomGetQueryingDescendantsForContainerParams {
    pub node_id: Box<NodeId>,
}
/// ⚠️ Experimental
/** Returns the descendants of a container query container that have
container queries against this container.*/
pub type DomGetQueryingDescendantsForContainerReturns = ();
/// ⚠️ Experimental
/** Returns the target anchor element of the given anchor query according to
https://www.w3.org/TR/css-anchor-position-1/#target.*/
pub struct DomGetAnchorElementParams {
    pub node_id: Box<NodeId>,
    pub anchor_specifier: String,
}
/// ⚠️ Experimental
/** Returns the target anchor element of the given anchor query according to
https://www.w3.org/TR/css-anchor-position-1/#target.*/
pub type DomGetAnchorElementReturns = ();
