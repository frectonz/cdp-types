pub use crate::common::*;
use crate::dom::*;
/// Unique Layer identifier.
pub struct LayerId(String);
/// Unique snapshot identifier.
pub struct SnapshotId(String);
/// Rectangle where scrolling happens on the main thread.
pub struct ScrollRect {
    pub rect: (),
    pub _type: String,
}
/// Sticky position constraints.
pub struct StickyPositionConstraint {
    pub sticky_box_rect: (),
    pub containing_block_rect: (),
    pub nearest_layer_shifting_sticky_box: (),
    pub nearest_layer_shifting_containing_block: (),
}
/// Serialized fragment of layer picture along with its offset within the layer.
pub struct PictureTile {
    pub x: u64,
    pub y: u64,
    pub picture: String,
}
/// Information about a compositing layer.
pub struct Layer {
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
pub struct PaintProfile(Vec<u64>);
