use crate::common::*;
/// Unique DOM node identifier.
pub struct NodeId(i64);
/** Unique DOM node identifier used to reference a node that may not have been pushed to the
front-end.*/
pub struct BackendNodeId(i64);
/// Backend node with a friendly name.
pub struct BackendNode {
    pub node_type: Box<i64>,
    pub node_name: Box<String>,
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
    pub node_type: Box<i64>,
    pub node_name: Box<String>,
    pub local_name: Box<String>,
    pub node_value: Box<String>,
    pub child_node_count: Box<i64>,
    pub children: (),
    pub attributes: (),
    pub document_url: Box<String>,
    pub base_url: Box<String>,
    pub public_id: Box<String>,
    pub system_id: Box<String>,
    pub internal_subset: Box<String>,
    pub xml_version: Box<String>,
    pub name: Box<String>,
    pub value: Box<String>,
    pub pseudo_type: Box<PseudoType>,
    pub pseudo_identifier: Box<String>,
    pub shadow_root_type: Box<ShadowRootType>,
    pub frame_id: Box<FrameId>,
    pub content_document: Box<Node>,
    pub shadow_roots: (),
    pub template_content: Box<Node>,
    pub pseudo_elements: (),
    pub imported_document: Box<Node>,
    pub distributed_nodes: (),
    pub is_svg: (),
    pub compatibility_mode: Box<CompatibilityMode>,
    pub assigned_slot: Box<BackendNode>,
    pub is_scrollable: (),
}
/// A structure to hold the top-level node of a detached tree and an array of its retained descendants.
pub struct DetachedElementInfo {
    pub tree_node: Box<Node>,
    pub retained_node_ids: (),
}
/// A structure holding an RGBA color.
pub struct Rgba {
    pub r: Box<i64>,
    pub g: Box<i64>,
    pub b: Box<i64>,
    pub a: Box<u64>,
}
/// An array of quad vertices, x immediately followed by y for each point, points clock-wise.
pub struct Quad(Vec<u64>);
/// Box model.
pub struct BoxModel {
    pub content: Box<Quad>,
    pub padding: Box<Quad>,
    pub border: Box<Quad>,
    pub margin: Box<Quad>,
    pub width: Box<i64>,
    pub height: Box<i64>,
    pub shape_outside: Box<ShapeOutsideInfo>,
}
/// CSS Shape Outside details.
pub struct ShapeOutsideInfo {
    pub bounds: Box<Quad>,
    pub shape: (),
    pub margin_shape: (),
}
/// Rectangle.
pub struct Rect {
    pub x: Box<u64>,
    pub y: Box<u64>,
    pub width: Box<u64>,
    pub height: Box<u64>,
}
