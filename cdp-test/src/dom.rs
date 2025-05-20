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
pub type DomCollectClassNamesFromSubtree = ();
pub type DomCopyTo = ();
pub type DomDescribeNode = ();
pub type DomScrollIntoViewIfNeeded = ();
pub type DomDisable = ();
pub type DomDiscardSearchResults = ();
pub type DomEnable = ();
pub type DomFocus = ();
pub type DomGetAttributes = ();
pub type DomGetBoxModel = ();
pub type DomGetContentQuads = ();
pub type DomGetDocument = ();
pub type DomGetFlattenedDocument = ();
pub type DomGetNodesForSubtreeByStyle = ();
pub type DomGetNodeForLocation = ();
pub type DomGetOuterHtml = ();
pub type DomGetRelayoutBoundary = ();
pub type DomGetSearchResults = ();
pub type DomHideHighlight = ();
pub type DomHighlightNode = ();
pub type DomHighlightRect = ();
pub type DomMarkUndoableState = ();
pub type DomMoveTo = ();
pub type DomPerformSearch = ();
pub type DomPushNodeByPathToFrontend = ();
pub type DomPushNodesByBackendIdsToFrontend = ();
pub type DomQuerySelector = ();
pub type DomQuerySelectorAll = ();
pub type DomGetTopLayerElements = ();
pub type DomGetElementByRelation = ();
pub type DomRedo = ();
pub type DomRemoveAttribute = ();
pub type DomRemoveNode = ();
pub type DomRequestChildNodes = ();
pub type DomRequestNode = ();
pub type DomResolveNode = ();
pub type DomSetAttributeValue = ();
pub type DomSetAttributesAsText = ();
pub type DomSetFileInputFiles = ();
pub type DomSetNodeStackTracesEnabled = ();
pub type DomGetNodeStackTraces = ();
pub type DomGetFileInfo = ();
pub type DomGetDetachedDomNodes = ();
pub type DomSetInspectedNode = ();
pub type DomSetNodeName = ();
pub type DomSetNodeValue = ();
pub type DomSetOuterHtml = ();
pub type DomUndo = ();
pub type DomGetFrameOwner = ();
pub type DomGetContainerForNode = ();
pub type DomGetQueryingDescendantsForContainer = ();
pub type DomGetAnchorElement = ();
