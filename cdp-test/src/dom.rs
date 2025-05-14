pub use crate::common::*;
/// Unique DOM node identifier.
pub struct NodeId(i64);
/** Unique DOM node identifier used to reference a node that may not have been pushed to the
front-end.*/
pub struct BackendNodeId(i64);
/// Backend node with a friendly name.
pub struct BackendNode {
    pub node_type: i64,
    pub node_name: String,
    pub backend_node_id: (),
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
    pub node_id: (),
    pub parent_id: (),
    pub backend_node_id: (),
    pub node_type: i64,
    pub node_name: String,
    pub local_name: String,
    pub node_value: String,
    pub child_node_count: i64,
    pub children: (),
    pub attributes: (),
    pub document_url: String,
    pub base_url: String,
    pub public_id: String,
    pub system_id: String,
    pub internal_subset: String,
    pub xml_version: String,
    pub name: String,
    pub value: String,
    pub pseudo_type: (),
    pub pseudo_identifier: String,
    pub shadow_root_type: (),
    pub frame_id: (),
    pub content_document: (),
    pub shadow_roots: (),
    pub template_content: (),
    pub pseudo_elements: (),
    pub imported_document: (),
    pub distributed_nodes: (),
    pub is_svg: (),
    pub compatibility_mode: (),
    pub assigned_slot: (),
    pub is_scrollable: (),
}
/// A structure to hold the top-level node of a detached tree and an array of its retained descendants.
pub struct DetachedElementInfo {
    pub tree_node: (),
    pub retained_node_ids: (),
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
    pub content: (),
    pub padding: (),
    pub border: (),
    pub margin: (),
    pub width: i64,
    pub height: i64,
    pub shape_outside: (),
}
/// CSS Shape Outside details.
pub struct ShapeOutsideInfo {
    pub bounds: (),
    pub shape: (),
    pub margin_shape: (),
}
/// Rectangle.
pub struct Rect {
    pub x: u64,
    pub y: u64,
    pub width: u64,
    pub height: u64,
}
