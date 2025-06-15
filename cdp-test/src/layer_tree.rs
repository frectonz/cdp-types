use crate::common::*;
use crate::dom::*;
/// Unique Layer identifier.
pub struct LayerId(String);
/// Unique snapshot identifier.
pub struct SnapshotId(String);
/// Rectangle where scrolling happens on the main thread.
pub struct ScrollRect {
    pub rect: Box<Rect>,
    pub _type: String,
}
/// Sticky position constraints.
pub struct StickyPositionConstraint {
    pub sticky_box_rect: Box<Rect>,
    pub containing_block_rect: Box<Rect>,
    pub nearest_layer_shifting_sticky_box: Box<LayerId>,
    pub nearest_layer_shifting_containing_block: Box<LayerId>,
}
/// Serialized fragment of layer picture along with its offset within the layer.
pub struct PictureTile {
    pub x: u64,
    pub y: u64,
    pub picture: String,
}
/// Information about a compositing layer.
pub struct Layer {
    pub layer_id: Box<LayerId>,
    pub parent_layer_id: Box<LayerId>,
    pub backend_node_id: Box<BackendNodeId>,
    pub offset_x: u64,
    pub offset_y: u64,
    pub width: u64,
    pub height: u64,
    pub transform: Vec<u64>,
    pub anchor_x: u64,
    pub anchor_y: u64,
    pub anchor_z: u64,
    pub paint_count: i64,
    pub draws_content: bool,
    pub invisible: bool,
    pub scroll_rects: Vec<ScrollRect>,
    pub sticky_position_constraint: Box<StickyPositionConstraint>,
}
/// Array of timings, one per paint step.
pub struct PaintProfile(Vec<u64>);
/// Provides the reasons why the given layer was composited.
pub struct LayerTreeCompositingReasonsParams {
    pub layer_id: Box<LayerId>,
}
/// Provides the reasons why the given layer was composited.
pub struct LayerTreeCompositingReasonsParams {
    pub compositing_reasons: Vec<String>,
    pub compositing_reason_ids: Vec<String>,
}
/// Disables compositing tree inspection.
pub type LayerTreeDisableParams = ();
/// Disables compositing tree inspection.
pub type LayerTreeDisableReturns = ();
/// Enables compositing tree inspection.
pub type LayerTreeEnableParams = ();
/// Enables compositing tree inspection.
pub type LayerTreeEnableReturns = ();
/// Returns the snapshot identifier.
pub struct LayerTreeLoadSnapshotParams {
    pub tiles: Vec<PictureTile>,
}
/// Returns the snapshot identifier.
pub struct LayerTreeLoadSnapshotParams {
    pub snapshot_id: Box<SnapshotId>,
}
/// Returns the layer snapshot identifier.
pub struct LayerTreeMakeSnapshotParams {
    pub layer_id: Box<LayerId>,
}
/// Returns the layer snapshot identifier.
pub struct LayerTreeMakeSnapshotParams {
    pub snapshot_id: Box<SnapshotId>,
}
pub struct LayerTreeProfileSnapshotParams {
    pub snapshot_id: Box<SnapshotId>,
    pub min_repeat_count: i64,
    pub min_duration: u64,
    pub clip_rect: Box<Rect>,
}
pub struct LayerTreeProfileSnapshotParams {
    pub timings: Vec<PaintProfile>,
}
/// Releases layer snapshot captured by the back-end.
pub struct LayerTreeReleaseSnapshotParams {
    pub snapshot_id: Box<SnapshotId>,
}
/// Releases layer snapshot captured by the back-end.
pub type LayerTreeReleaseSnapshotReturns = ();
/// Replays the layer snapshot and returns the resulting bitmap.
pub struct LayerTreeReplaySnapshotParams {
    pub snapshot_id: Box<SnapshotId>,
    pub from_step: i64,
    pub to_step: i64,
    pub scale: u64,
}
/// Replays the layer snapshot and returns the resulting bitmap.
pub struct LayerTreeReplaySnapshotParams {
    pub data_url: String,
}
/// Replays the layer snapshot and returns canvas log.
pub struct LayerTreeSnapshotCommandLogParams {
    pub snapshot_id: Box<SnapshotId>,
}
/// Replays the layer snapshot and returns canvas log.
pub struct LayerTreeSnapshotCommandLogParams {
    pub command_log: Vec<serde_json::Map<String, serde_json::Value>>,
}
pub struct LayerTreeLayerPaintedEvent {
    pub layer_id: Box<LayerId>,
    pub clip: Box<Rect>,
}
pub struct LayerTreeLayerTreeDidChangeEvent {
    pub layers: Vec<Layer>,
}
