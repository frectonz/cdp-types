use crate::common::*;
use crate::css::*;
use crate::dom::*;
use crate::dom_debugger::*;
use crate::page::*;
/// A Node in the DOM tree.
pub struct DomNode {
    pub node_type: i64,
    pub node_name: String,
    pub node_value: String,
    pub text_value: String,
    pub input_value: String,
    pub input_checked: bool,
    pub option_selected: bool,
    pub backend_node_id: Box<BackendNodeId>,
    pub child_node_indexes: Vec<i64>,
    pub attributes: Vec<NameValue>,
    pub pseudo_element_indexes: Vec<i64>,
    pub layout_node_index: i64,
    pub document_url: String,
    pub base_url: String,
    pub content_language: String,
    pub document_encoding: String,
    pub public_id: String,
    pub system_id: String,
    pub frame_id: Box<crate::page::FrameId>,
    pub content_document_index: i64,
    pub pseudo_type: Box<PseudoType>,
    pub shadow_root_type: Box<ShadowRootType>,
    pub is_clickable: bool,
    pub event_listeners: Vec<EventListener>,
    pub current_source_url: String,
    pub origin_url: String,
    pub scroll_offset_x: u64,
    pub scroll_offset_y: u64,
}
/** Details of post layout rendered text positions. The exact layout should not be regarded as
stable and may change between versions.*/
pub struct InlineTextBox {
    pub bounding_box: Box<Rect>,
    pub start_character_index: i64,
    pub num_characters: i64,
}
/// Details of an element in the DOM tree with a LayoutObject.
pub struct LayoutTreeNode {
    pub dom_node_index: i64,
    pub bounding_box: Box<Rect>,
    pub layout_text: String,
    pub inline_text_nodes: Vec<InlineTextBox>,
    pub style_index: i64,
    pub paint_order: i64,
    pub is_stacking_context: bool,
}
/// A subset of the full ComputedStyle as defined by the request whitelist.
pub struct ComputedStyle {
    pub properties: Vec<NameValue>,
}
/// A name/value pair.
pub struct NameValue {
    pub name: String,
    pub value: String,
}
/// Index of the string in the strings table.
pub struct StringIndex(i64);
/// Index of the string in the strings table.
pub struct ArrayOfStrings(Vec<StringIndex>);
/// Data that is only present on rare nodes.
pub struct RareStringData {
    pub index: Vec<i64>,
    pub value: Vec<StringIndex>,
}
pub struct RareBooleanData {
    pub index: Vec<i64>,
}
pub struct RareIntegerData {
    pub index: Vec<i64>,
    pub value: Vec<i64>,
}
pub struct Rectangle(Vec<u64>);
/// Document snapshot.
pub struct DocumentSnapshot {
    pub document_url: Box<StringIndex>,
    pub title: Box<StringIndex>,
    pub base_url: Box<StringIndex>,
    pub content_language: Box<StringIndex>,
    pub encoding_name: Box<StringIndex>,
    pub public_id: Box<StringIndex>,
    pub system_id: Box<StringIndex>,
    pub frame_id: Box<StringIndex>,
    pub nodes: Box<NodeTreeSnapshot>,
    pub layout: Box<LayoutTreeSnapshot>,
    pub text_boxes: Box<TextBoxSnapshot>,
    pub scroll_offset_x: u64,
    pub scroll_offset_y: u64,
    pub content_width: u64,
    pub content_height: u64,
}
/// Table containing nodes.
pub struct NodeTreeSnapshot {
    pub parent_index: Vec<i64>,
    pub node_type: Vec<i64>,
    pub shadow_root_type: Box<RareStringData>,
    pub node_name: Vec<StringIndex>,
    pub node_value: Vec<StringIndex>,
    pub backend_node_id: Vec<BackendNodeId>,
    pub attributes: Vec<ArrayOfStrings>,
    pub text_value: Box<RareStringData>,
    pub input_value: Box<RareStringData>,
    pub input_checked: Box<RareBooleanData>,
    pub option_selected: Box<RareBooleanData>,
    pub content_document_index: Box<RareIntegerData>,
    pub pseudo_type: Box<RareStringData>,
    pub pseudo_identifier: Box<RareStringData>,
    pub is_clickable: Box<RareBooleanData>,
    pub current_source_url: Box<RareStringData>,
    pub origin_url: Box<RareStringData>,
}
/// Table of details of an element in the DOM tree with a LayoutObject.
pub struct LayoutTreeSnapshot {
    pub node_index: Vec<i64>,
    pub styles: Vec<ArrayOfStrings>,
    pub bounds: Vec<Rectangle>,
    pub text: Vec<StringIndex>,
    pub stacking_contexts: Box<RareBooleanData>,
    pub paint_orders: Vec<i64>,
    pub offset_rects: Vec<Rectangle>,
    pub scroll_rects: Vec<Rectangle>,
    pub client_rects: Vec<Rectangle>,
    pub blended_background_colors: Vec<StringIndex>,
    pub text_color_opacities: Vec<u64>,
}
/** Table of details of the post layout rendered text positions. The exact layout should not be regarded as
stable and may change between versions.*/
pub struct TextBoxSnapshot {
    pub layout_index: Vec<i64>,
    pub bounds: Vec<Rectangle>,
    pub start: Vec<i64>,
    pub length: Vec<i64>,
}
/// Disables DOM snapshot agent for the given page.
pub type DomSnapshotDisableParams = ();
/// Disables DOM snapshot agent for the given page.
pub type DomSnapshotDisableReturns = ();
/// Enables DOM snapshot agent for the given page.
pub type DomSnapshotEnableParams = ();
/// Enables DOM snapshot agent for the given page.
pub type DomSnapshotEnableReturns = ();
#[deprecated]
/** Returns a document snapshot, including the full DOM tree of the root node (including iframes,
template contents, and imported documents) in a flattened array, as well as layout and
white-listed computed style information for the nodes. Shadow DOM in the returned DOM tree is
flattened.*/
pub struct DomSnapshotGetSnapshotParams {
    test: (),
    test: (),
    test: (),
    test: (),
}
#[deprecated]
/** Returns a document snapshot, including the full DOM tree of the root node (including iframes,
template contents, and imported documents) in a flattened array, as well as layout and
white-listed computed style information for the nodes. Shadow DOM in the returned DOM tree is
flattened.*/
pub type DomSnapshotGetSnapshotReturns = ();
/** Returns a document snapshot, including the full DOM tree of the root node (including iframes,
template contents, and imported documents) in a flattened array, as well as layout and
white-listed computed style information for the nodes. Shadow DOM in the returned DOM tree is
flattened.*/
pub struct DomSnapshotCaptureSnapshotParams {
    test: (),
    test: (),
    test: (),
    test: (),
    test: (),
}
/** Returns a document snapshot, including the full DOM tree of the root node (including iframes,
template contents, and imported documents) in a flattened array, as well as layout and
white-listed computed style information for the nodes. Shadow DOM in the returned DOM tree is
flattened.*/
pub type DomSnapshotCaptureSnapshotReturns = ();
