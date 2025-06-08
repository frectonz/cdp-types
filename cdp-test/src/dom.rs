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
pub type DomCollectClassNamesFromSubtreeParams = ();
/// ⚠️ Experimental
/// Collects class names for the node with given id and all of it's child nodes.
pub type DomCollectClassNamesFromSubtreeResults = ();
/// ⚠️ Experimental
/** Creates a deep copy of the specified node and places it into the target container before the
given anchor.*/
pub type DomCopyToParams = ();
/// ⚠️ Experimental
/** Creates a deep copy of the specified node and places it into the target container before the
given anchor.*/
pub type DomCopyToResults = ();
/** Describes node given its id, does not require domain to be enabled. Does not start tracking any
objects, can be used for automation.*/
pub type DomDescribeNodeParams = ();
/** Describes node given its id, does not require domain to be enabled. Does not start tracking any
objects, can be used for automation.*/
pub type DomDescribeNodeResults = ();
/** Scrolls the specified rect of the given node into view if not already visible.
Note: exactly one between nodeId, backendNodeId and objectId should be passed
to identify the node.*/
pub type DomScrollIntoViewIfNeededParams = ();
/** Scrolls the specified rect of the given node into view if not already visible.
Note: exactly one between nodeId, backendNodeId and objectId should be passed
to identify the node.*/
pub type DomScrollIntoViewIfNeededResults = ();
/// Disables DOM agent for the given page.
pub type DomDisableParams = ();
/// Disables DOM agent for the given page.
pub type DomDisableResults = ();
/// ⚠️ Experimental
/** Discards search results from the session with the given id. `getSearchResults` should no longer
be called for that search.*/
pub type DomDiscardSearchResultsParams = ();
/// ⚠️ Experimental
/** Discards search results from the session with the given id. `getSearchResults` should no longer
be called for that search.*/
pub type DomDiscardSearchResultsResults = ();
/// Enables DOM agent for the given page.
pub type DomEnableParams = ();
/// Enables DOM agent for the given page.
pub type DomEnableResults = ();
/// Focuses the given element.
pub type DomFocusParams = ();
/// Focuses the given element.
pub type DomFocusResults = ();
/// Returns attributes for the specified node.
pub type DomGetAttributesParams = ();
/// Returns attributes for the specified node.
pub type DomGetAttributesResults = ();
/// Returns boxes for the given node.
pub type DomGetBoxModelParams = ();
/// Returns boxes for the given node.
pub type DomGetBoxModelResults = ();
/// ⚠️ Experimental
/** Returns quads that describe node position on the page. This method
might return multiple quads for inline nodes.*/
pub type DomGetContentQuadsParams = ();
/// ⚠️ Experimental
/** Returns quads that describe node position on the page. This method
might return multiple quads for inline nodes.*/
pub type DomGetContentQuadsResults = ();
/** Returns the root DOM node (and optionally the subtree) to the caller.
Implicitly enables the DOM domain events for the current target.*/
pub type DomGetDocumentParams = ();
/** Returns the root DOM node (and optionally the subtree) to the caller.
Implicitly enables the DOM domain events for the current target.*/
pub type DomGetDocumentResults = ();
#[deprecated]
/** Returns the root DOM node (and optionally the subtree) to the caller.
Deprecated, as it is not designed to work well with the rest of the DOM agent.
Use DOMSnapshot.captureSnapshot instead.*/
pub type DomGetFlattenedDocumentParams = ();
#[deprecated]
/** Returns the root DOM node (and optionally the subtree) to the caller.
Deprecated, as it is not designed to work well with the rest of the DOM agent.
Use DOMSnapshot.captureSnapshot instead.*/
pub type DomGetFlattenedDocumentResults = ();
/// ⚠️ Experimental
/// Finds nodes with a given computed style in a subtree.
pub type DomGetNodesForSubtreeByStyleParams = ();
/// ⚠️ Experimental
/// Finds nodes with a given computed style in a subtree.
pub type DomGetNodesForSubtreeByStyleResults = ();
/** Returns node id at given location. Depending on whether DOM domain is enabled, nodeId is
either returned or not.*/
pub type DomGetNodeForLocationParams = ();
/** Returns node id at given location. Depending on whether DOM domain is enabled, nodeId is
either returned or not.*/
pub type DomGetNodeForLocationResults = ();
/// Returns node's HTML markup.
pub type DomGetOuterHtmlParams = ();
/// Returns node's HTML markup.
pub type DomGetOuterHtmlResults = ();
/// ⚠️ Experimental
/// Returns the id of the nearest ancestor that is a relayout boundary.
pub type DomGetRelayoutBoundaryParams = ();
/// ⚠️ Experimental
/// Returns the id of the nearest ancestor that is a relayout boundary.
pub type DomGetRelayoutBoundaryResults = ();
/// ⚠️ Experimental
/** Returns search results from given `fromIndex` to given `toIndex` from the search with the given
identifier.*/
pub type DomGetSearchResultsParams = ();
/// ⚠️ Experimental
/** Returns search results from given `fromIndex` to given `toIndex` from the search with the given
identifier.*/
pub type DomGetSearchResultsResults = ();
/// Hides any highlight.
pub type DomHideHighlightParams = ();
/// Hides any highlight.
pub type DomHideHighlightResults = ();
/// Highlights DOM node.
pub type DomHighlightNodeParams = ();
/// Highlights DOM node.
pub type DomHighlightNodeResults = ();
/// Highlights given rectangle.
pub type DomHighlightRectParams = ();
/// Highlights given rectangle.
pub type DomHighlightRectResults = ();
/// ⚠️ Experimental
/// Marks last undoable state.
pub type DomMarkUndoableStateParams = ();
/// ⚠️ Experimental
/// Marks last undoable state.
pub type DomMarkUndoableStateResults = ();
/// Moves node into the new container, places it before the given anchor.
pub type DomMoveToParams = ();
/// Moves node into the new container, places it before the given anchor.
pub type DomMoveToResults = ();
/// ⚠️ Experimental
/** Searches for a given string in the DOM tree. Use `getSearchResults` to access search results or
`cancelSearch` to end this search session.*/
pub type DomPerformSearchParams = ();
/// ⚠️ Experimental
/** Searches for a given string in the DOM tree. Use `getSearchResults` to access search results or
`cancelSearch` to end this search session.*/
pub type DomPerformSearchResults = ();
/// ⚠️ Experimental
/// Requests that the node is sent to the caller given its path. // FIXME, use XPath
pub type DomPushNodeByPathToFrontendParams = ();
/// ⚠️ Experimental
/// Requests that the node is sent to the caller given its path. // FIXME, use XPath
pub type DomPushNodeByPathToFrontendResults = ();
/// ⚠️ Experimental
/// Requests that a batch of nodes is sent to the caller given their backend node ids.
pub type DomPushNodesByBackendIdsToFrontendParams = ();
/// ⚠️ Experimental
/// Requests that a batch of nodes is sent to the caller given their backend node ids.
pub type DomPushNodesByBackendIdsToFrontendResults = ();
/// Executes `querySelector` on a given node.
pub type DomQuerySelectorParams = ();
/// Executes `querySelector` on a given node.
pub type DomQuerySelectorResults = ();
/// Executes `querySelectorAll` on a given node.
pub type DomQuerySelectorAllParams = ();
/// Executes `querySelectorAll` on a given node.
pub type DomQuerySelectorAllResults = ();
/// ⚠️ Experimental
/** Returns NodeIds of current top layer elements.
Top layer is rendered closest to the user within a viewport, therefore its elements always
appear on top of all other content.*/
pub type DomGetTopLayerElementsParams = ();
/// ⚠️ Experimental
/** Returns NodeIds of current top layer elements.
Top layer is rendered closest to the user within a viewport, therefore its elements always
appear on top of all other content.*/
pub type DomGetTopLayerElementsResults = ();
/// ⚠️ Experimental
/// Returns the NodeId of the matched element according to certain relations.
pub type DomGetElementByRelationParams = ();
/// ⚠️ Experimental
/// Returns the NodeId of the matched element according to certain relations.
pub type DomGetElementByRelationResults = ();
/// ⚠️ Experimental
/// Re-does the last undone action.
pub type DomRedoParams = ();
/// ⚠️ Experimental
/// Re-does the last undone action.
pub type DomRedoResults = ();
/// Removes attribute with given name from an element with given id.
pub type DomRemoveAttributeParams = ();
/// Removes attribute with given name from an element with given id.
pub type DomRemoveAttributeResults = ();
/// Removes node with given id.
pub type DomRemoveNodeParams = ();
/// Removes node with given id.
pub type DomRemoveNodeResults = ();
/** Requests that children of the node with given id are returned to the caller in form of
`setChildNodes` events where not only immediate children are retrieved, but all children down to
the specified depth.*/
pub type DomRequestChildNodesParams = ();
/** Requests that children of the node with given id are returned to the caller in form of
`setChildNodes` events where not only immediate children are retrieved, but all children down to
the specified depth.*/
pub type DomRequestChildNodesResults = ();
/** Requests that the node is sent to the caller given the JavaScript node object reference. All
nodes that form the path from the node to the root are also sent to the client as a series of
`setChildNodes` notifications.*/
pub type DomRequestNodeParams = ();
/** Requests that the node is sent to the caller given the JavaScript node object reference. All
nodes that form the path from the node to the root are also sent to the client as a series of
`setChildNodes` notifications.*/
pub type DomRequestNodeResults = ();
/// Resolves the JavaScript node object for a given NodeId or BackendNodeId.
pub type DomResolveNodeParams = ();
/// Resolves the JavaScript node object for a given NodeId or BackendNodeId.
pub type DomResolveNodeResults = ();
/// Sets attribute for an element with given id.
pub type DomSetAttributeValueParams = ();
/// Sets attribute for an element with given id.
pub type DomSetAttributeValueResults = ();
/** Sets attributes on element with given id. This method is useful when user edits some existing
attribute value and types in several attribute name/value pairs.*/
pub type DomSetAttributesAsTextParams = ();
/** Sets attributes on element with given id. This method is useful when user edits some existing
attribute value and types in several attribute name/value pairs.*/
pub type DomSetAttributesAsTextResults = ();
/// Sets files for the given file input element.
pub type DomSetFileInputFilesParams = ();
/// Sets files for the given file input element.
pub type DomSetFileInputFilesResults = ();
/// ⚠️ Experimental
/// Sets if stack traces should be captured for Nodes. See `Node.getNodeStackTraces`. Default is disabled.
pub type DomSetNodeStackTracesEnabledParams = ();
/// ⚠️ Experimental
/// Sets if stack traces should be captured for Nodes. See `Node.getNodeStackTraces`. Default is disabled.
pub type DomSetNodeStackTracesEnabledResults = ();
/// ⚠️ Experimental
/// Gets stack traces associated with a Node. As of now, only provides stack trace for Node creation.
pub type DomGetNodeStackTracesParams = ();
/// ⚠️ Experimental
/// Gets stack traces associated with a Node. As of now, only provides stack trace for Node creation.
pub type DomGetNodeStackTracesResults = ();
/// ⚠️ Experimental
/** Returns file information for the given
File wrapper.*/
pub type DomGetFileInfoParams = ();
/// ⚠️ Experimental
/** Returns file information for the given
File wrapper.*/
pub type DomGetFileInfoResults = ();
/// ⚠️ Experimental
/// Returns list of detached nodes
pub type DomGetDetachedDomNodesParams = ();
/// ⚠️ Experimental
/// Returns list of detached nodes
pub type DomGetDetachedDomNodesResults = ();
/// ⚠️ Experimental
/** Enables console to refer to the node with given id via $x (see Command Line API for more details
$x functions).*/
pub type DomSetInspectedNodeParams = ();
/// ⚠️ Experimental
/** Enables console to refer to the node with given id via $x (see Command Line API for more details
$x functions).*/
pub type DomSetInspectedNodeResults = ();
/// Sets node name for a node with given id.
pub type DomSetNodeNameParams = ();
/// Sets node name for a node with given id.
pub type DomSetNodeNameResults = ();
/// Sets node value for a node with given id.
pub type DomSetNodeValueParams = ();
/// Sets node value for a node with given id.
pub type DomSetNodeValueResults = ();
/// Sets node HTML markup, returns new node id.
pub type DomSetOuterHtmlParams = ();
/// Sets node HTML markup, returns new node id.
pub type DomSetOuterHtmlResults = ();
/// ⚠️ Experimental
/// Undoes the last performed action.
pub type DomUndoParams = ();
/// ⚠️ Experimental
/// Undoes the last performed action.
pub type DomUndoResults = ();
/// ⚠️ Experimental
/// Returns iframe node that owns iframe with the given domain.
pub type DomGetFrameOwnerParams = ();
/// ⚠️ Experimental
/// Returns iframe node that owns iframe with the given domain.
pub type DomGetFrameOwnerResults = ();
/// ⚠️ Experimental
/** Returns the query container of the given node based on container query
conditions: containerName, physical and logical axes, and whether it queries
scroll-state. If no axes are provided and queriesScrollState is false, the
style container is returned, which is the direct parent or the closest
element with a matching container-name.*/
pub type DomGetContainerForNodeParams = ();
/// ⚠️ Experimental
/** Returns the query container of the given node based on container query
conditions: containerName, physical and logical axes, and whether it queries
scroll-state. If no axes are provided and queriesScrollState is false, the
style container is returned, which is the direct parent or the closest
element with a matching container-name.*/
pub type DomGetContainerForNodeResults = ();
/// ⚠️ Experimental
/** Returns the descendants of a container query container that have
container queries against this container.*/
pub type DomGetQueryingDescendantsForContainerParams = ();
/// ⚠️ Experimental
/** Returns the descendants of a container query container that have
container queries against this container.*/
pub type DomGetQueryingDescendantsForContainerResults = ();
/// ⚠️ Experimental
/** Returns the target anchor element of the given anchor query according to
https://www.w3.org/TR/css-anchor-position-1/#target.*/
pub type DomGetAnchorElementParams = ();
/// ⚠️ Experimental
/** Returns the target anchor element of the given anchor query according to
https://www.w3.org/TR/css-anchor-position-1/#target.*/
pub type DomGetAnchorElementResults = ();
