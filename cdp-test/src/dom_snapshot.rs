use crate::common::*;
use crate::css::*;
use crate::dom::*;
use crate::dom_debugger::*;
use crate::page::*;
/// A Node in the DOM tree.
pub struct DomNode {
    pub node_type: Box<i64>,
    pub node_name: Box<String>,
    pub node_value: Box<String>,
    pub text_value: Box<String>,
    pub input_value: Box<String>,
    pub input_checked: (),
    pub option_selected: (),
    pub backend_node_id: Box<BackendNodeId>,
    pub child_node_indexes: (),
    pub attributes: (),
    pub pseudo_element_indexes: (),
    pub layout_node_index: Box<i64>,
    pub document_url: Box<String>,
    pub base_url: Box<String>,
    pub content_language: Box<String>,
    pub document_encoding: Box<String>,
    pub public_id: Box<String>,
    pub system_id: Box<String>,
    pub frame_id: Box<crate::page::FrameId>,
    pub content_document_index: Box<i64>,
    pub pseudo_type: Box<PseudoType>,
    pub shadow_root_type: Box<ShadowRootType>,
    pub is_clickable: (),
    pub event_listeners: (),
    pub current_source_url: Box<String>,
    pub origin_url: Box<String>,
    pub scroll_offset_x: Box<u64>,
    pub scroll_offset_y: Box<u64>,
}
/** Details of post layout rendered text positions. The exact layout should not be regarded as
stable and may change between versions.*/
pub struct InlineTextBox {
    pub bounding_box: Box<Rect>,
    pub start_character_index: Box<i64>,
    pub num_characters: Box<i64>,
}
/// Details of an element in the DOM tree with a LayoutObject.
pub struct LayoutTreeNode {
    pub dom_node_index: Box<i64>,
    pub bounding_box: Box<Rect>,
    pub layout_text: Box<String>,
    pub inline_text_nodes: (),
    pub style_index: Box<i64>,
    pub paint_order: Box<i64>,
    pub is_stacking_context: (),
}
/// A subset of the full ComputedStyle as defined by the request whitelist.
pub struct ComputedStyle {
    pub properties: (),
}
/// A name/value pair.
pub struct NameValue {
    pub name: Box<String>,
    pub value: Box<String>,
}
/// Index of the string in the strings table.
pub struct StringIndex(i64);
/// Index of the string in the strings table.
pub struct ArrayOfStrings(Vec<StringIndex>);
/// Data that is only present on rare nodes.
pub struct RareStringData {
    pub index: (),
    pub value: (),
}
pub struct RareBooleanData {
    pub index: (),
}
pub struct RareIntegerData {
    pub index: (),
    pub value: (),
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
    pub scroll_offset_x: Box<u64>,
    pub scroll_offset_y: Box<u64>,
    pub content_width: Box<u64>,
    pub content_height: Box<u64>,
}
/// Table containing nodes.
pub struct NodeTreeSnapshot {
    pub parent_index: (),
    pub node_type: (),
    pub shadow_root_type: Box<RareStringData>,
    pub node_name: (),
    pub node_value: (),
    pub backend_node_id: (),
    pub attributes: (),
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
    pub node_index: (),
    pub styles: (),
    pub bounds: (),
    pub text: (),
    pub stacking_contexts: Box<RareBooleanData>,
    pub paint_orders: (),
    pub offset_rects: (),
    pub scroll_rects: (),
    pub client_rects: (),
    pub blended_background_colors: (),
    pub text_color_opacities: (),
}
/** Table of details of the post layout rendered text positions. The exact layout should not be regarded as
stable and may change between versions.*/
pub struct TextBoxSnapshot {
    pub layout_index: (),
    pub bounds: (),
    pub start: (),
    pub length: (),
}
