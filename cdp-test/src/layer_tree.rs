use crate::common::*;
use crate::dom::*;
/// Unique Layer identifier.
pub struct LayerId(String);
/// Unique snapshot identifier.
pub struct SnapshotId(String);
/// Rectangle where scrolling happens on the main thread.
pub struct ScrollRect {
    pub rect: Box<DomRect>,
    pub _type: Box<String>,
}
/// Sticky position constraints.
pub struct StickyPositionConstraint {
    pub sticky_box_rect: Box<DomRect>,
    pub containing_block_rect: Box<DomRect>,
    pub nearest_layer_shifting_sticky_box: Box<LayerId>,
    pub nearest_layer_shifting_containing_block: Box<LayerId>,
}
/// Serialized fragment of layer picture along with its offset within the layer.
pub struct PictureTile {
    pub x: Box<u64>,
    pub y: Box<u64>,
    pub picture: Box<String>,
}
/// Information about a compositing layer.
pub struct Layer {
    pub layer_id: Box<LayerId>,
    pub parent_layer_id: Box<LayerId>,
    pub backend_node_id: Box<DomBackendNodeId>,
    pub offset_x: Box<u64>,
    pub offset_y: Box<u64>,
    pub width: Box<u64>,
    pub height: Box<u64>,
    pub transform: (),
    pub anchor_x: Box<u64>,
    pub anchor_y: Box<u64>,
    pub anchor_z: Box<u64>,
    pub paint_count: Box<i64>,
    pub draws_content: (),
    pub invisible: (),
    pub scroll_rects: (),
    pub sticky_position_constraint: Box<StickyPositionConstraint>,
}
/// Array of timings, one per paint step.
pub struct PaintProfile(Vec<u64>);
