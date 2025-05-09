use crate::dom::*;
/// Unique Layer identifier.
/// <https://chromedevtools.github.io/devtools-protocol/tot/LayerTree/#type-LayerId>
pub struct LayerTreeLayerId(String);
/// Unique snapshot identifier.
/// <https://chromedevtools.github.io/devtools-protocol/tot/LayerTree/#type-SnapshotId>
pub struct LayerTreeSnapshotId(String);
/// Rectangle where scrolling happens on the main thread.
/// <https://chromedevtools.github.io/devtools-protocol/tot/LayerTree/#type-ScrollRect>
pub struct LayerTreeScrollRect {
    pub rect: (),
    pub _type: String,
}
/// Sticky position constraints.
/// <https://chromedevtools.github.io/devtools-protocol/tot/LayerTree/#type-StickyPositionConstraint>
pub struct LayerTreeStickyPositionConstraint {
    pub sticky_box_rect: (),
    pub containing_block_rect: (),
    pub nearest_layer_shifting_sticky_box: (),
    pub nearest_layer_shifting_containing_block: (),
}
/// Serialized fragment of layer picture along with its offset within the layer.
/// <https://chromedevtools.github.io/devtools-protocol/tot/LayerTree/#type-PictureTile>
pub struct LayerTreePictureTile {
    pub x: u64,
    pub y: u64,
    pub picture: String,
}
/// Information about a compositing layer.
/// <https://chromedevtools.github.io/devtools-protocol/tot/LayerTree/#type-Layer>
pub struct LayerTreeLayer {
    pub layer_id: (),
    pub parent_layer_id: (),
    pub backend_node_id: (),
    pub offset_x: u64,
    pub offset_y: u64,
    pub width: u64,
    pub height: u64,
    pub transform: (),
    pub anchor_x: u64,
    pub anchor_y: u64,
    pub anchor_z: u64,
    pub paint_count: i64,
    pub draws_content: (),
    pub invisible: (),
    pub scroll_rects: (),
    pub sticky_position_constraint: (),
}
/// Array of timings, one per paint step.
/// <https://chromedevtools.github.io/devtools-protocol/tot/LayerTree/#type-PaintProfile>
pub struct LayerTreePaintProfile(Vec<u64>);
