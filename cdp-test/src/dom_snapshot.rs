pub use crate::common::*;
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
    pub input_checked: (),
    pub option_selected: (),
    pub backend_node_id: (),
    pub child_node_indexes: (),
    pub attributes: (),
    pub pseudo_element_indexes: (),
    pub layout_node_index: i64,
    pub document_url: String,
    pub base_url: String,
    pub content_language: String,
    pub document_encoding: String,
    pub public_id: String,
    pub system_id: String,
    pub frame_id: (),
    pub content_document_index: i64,
    pub pseudo_type: (),
    pub shadow_root_type: (),
    pub is_clickable: (),
    pub event_listeners: (),
    pub current_source_url: String,
    pub origin_url: String,
    pub scroll_offset_x: u64,
    pub scroll_offset_y: u64,
}
/** Details of post layout rendered text positions. The exact layout should not be regarded as
stable and may change between versions.*/
pub struct InlineTextBox {
    pub bounding_box: (),
    pub start_character_index: i64,
    pub num_characters: i64,
}
/// Details of an element in the DOM tree with a LayoutObject.
pub struct LayoutTreeNode {
    pub dom_node_index: i64,
    pub bounding_box: (),
    pub layout_text: String,
    pub inline_text_nodes: (),
    pub style_index: i64,
    pub paint_order: i64,
    pub is_stacking_context: (),
}
/// A subset of the full ComputedStyle as defined by the request whitelist.
pub struct ComputedStyle {
    pub properties: (),
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
    pub document_url: (),
    pub title: (),
    pub base_url: (),
    pub content_language: (),
    pub encoding_name: (),
    pub public_id: (),
    pub system_id: (),
    pub frame_id: (),
    pub nodes: (),
    pub layout: (),
    pub text_boxes: (),
    pub scroll_offset_x: u64,
    pub scroll_offset_y: u64,
    pub content_width: u64,
    pub content_height: u64,
}
/// Table containing nodes.
pub struct NodeTreeSnapshot {
    pub parent_index: (),
    pub node_type: (),
    pub shadow_root_type: (),
    pub node_name: (),
    pub node_value: (),
    pub backend_node_id: (),
    pub attributes: (),
    pub text_value: (),
    pub input_value: (),
    pub input_checked: (),
    pub option_selected: (),
    pub content_document_index: (),
    pub pseudo_type: (),
    pub pseudo_identifier: (),
    pub is_clickable: (),
    pub current_source_url: (),
    pub origin_url: (),
}
/// Table of details of an element in the DOM tree with a LayoutObject.
pub struct LayoutTreeSnapshot {
    pub node_index: (),
    pub styles: (),
    pub bounds: (),
    pub text: (),
    pub stacking_contexts: (),
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
