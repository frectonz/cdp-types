/// Unique DOM node identifier.
/// <https://chromedevtools.github.io/devtools-protocol/tot/DOM/#type-NodeId>
pub struct DomNodeId(i64);
/** Unique DOM node identifier used to reference a node that may not have been pushed to the
front-end.*/
/// <https://chromedevtools.github.io/devtools-protocol/tot/DOM/#type-BackendNodeId>
pub struct DomBackendNodeId(i64);
/// Backend node with a friendly name.
/// <https://chromedevtools.github.io/devtools-protocol/tot/DOM/#type-BackendNode>
pub struct DomBackendNode {
    pub node_type: i64,
    pub node_name: String,
    pub backend_node_id: (),
}
/// Pseudo element type.
/// <https://chromedevtools.github.io/devtools-protocol/tot/DOM/#type-PseudoType>
pub enum DomPseudoType {
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
/// <https://chromedevtools.github.io/devtools-protocol/tot/DOM/#type-ShadowRootType>
pub enum DomShadowRootType {
    UserAgent,
    Open,
    Closed,
}
/// Document compatibility mode.
/// <https://chromedevtools.github.io/devtools-protocol/tot/DOM/#type-CompatibilityMode>
pub enum DomCompatibilityMode {
    QuirksMode,
    LimitedQuirksMode,
    NoQuirksMode,
}
/// ContainerSelector physical axes
/// <https://chromedevtools.github.io/devtools-protocol/tot/DOM/#type-PhysicalAxes>
pub enum DomPhysicalAxes {
    Horizontal,
    Vertical,
    Both,
}
/// ContainerSelector logical axes
/// <https://chromedevtools.github.io/devtools-protocol/tot/DOM/#type-LogicalAxes>
pub enum DomLogicalAxes {
    Inline,
    Block,
    Both,
}
/// Physical scroll orientation
/// <https://chromedevtools.github.io/devtools-protocol/tot/DOM/#type-ScrollOrientation>
pub enum DomScrollOrientation {
    Horizontal,
    Vertical,
}
/** DOM interaction is implemented in terms of mirror objects that represent the actual DOM nodes.
DOMNode is a base node mirror type.*/
/// <https://chromedevtools.github.io/devtools-protocol/tot/DOM/#type-Node>
pub struct DomNode {
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
/// <https://chromedevtools.github.io/devtools-protocol/tot/DOM/#type-DetachedElementInfo>
pub struct DomDetachedElementInfo {
    pub tree_node: (),
    pub retained_node_ids: (),
}
/// A structure holding an RGBA color.
/// <https://chromedevtools.github.io/devtools-protocol/tot/DOM/#type-RGBA>
pub struct Domrgba {
    pub r: i64,
    pub g: i64,
    pub b: i64,
    pub a: u64,
}
/// An array of quad vertices, x immediately followed by y for each point, points clock-wise.
/// <https://chromedevtools.github.io/devtools-protocol/tot/DOM/#type-Quad>
pub struct DomQuad(Vec<u64>);
/// Box model.
/// <https://chromedevtools.github.io/devtools-protocol/tot/DOM/#type-BoxModel>
pub struct DomBoxModel {
    pub content: (),
    pub padding: (),
    pub border: (),
    pub margin: (),
    pub width: i64,
    pub height: i64,
    pub shape_outside: (),
}
/// CSS Shape Outside details.
/// <https://chromedevtools.github.io/devtools-protocol/tot/DOM/#type-ShapeOutsideInfo>
pub struct DomShapeOutsideInfo {
    pub bounds: (),
    pub shape: (),
    pub margin_shape: (),
}
/// Rectangle.
/// <https://chromedevtools.github.io/devtools-protocol/tot/DOM/#type-Rect>
pub struct DomRect {
    pub x: u64,
    pub y: u64,
    pub width: u64,
    pub height: u64,
}
/// <https://chromedevtools.github.io/devtools-protocol/tot/DOM/#type-CSSComputedStyleProperty>
pub struct DomcssComputedStyleProperty {
    pub name: String,
    pub value: String,
}
