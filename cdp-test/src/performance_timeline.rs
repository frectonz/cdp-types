use crate::common::*;
use crate::dom::*;
use crate::network::*;
/// See https://github.com/WICG/LargestContentfulPaint and largest_contentful_paint.idl
pub struct LargestContentfulPaint {
    pub render_time: Box<NetworkTimeSinceEpoch>,
    pub load_time: Box<NetworkTimeSinceEpoch>,
    pub size: Box<u64>,
    pub element_id: Box<String>,
    pub url: Box<String>,
    pub node_id: Box<BackendNodeId>,
}
pub struct LayoutShiftAttribution {
    pub previous_rect: Box<Rect>,
    pub current_rect: Box<Rect>,
    pub node_id: Box<BackendNodeId>,
}
/// See https://wicg.github.io/layout-instability/#sec-layout-shift and layout_shift.idl
pub struct LayoutShift {
    pub value: Box<u64>,
    pub had_recent_input: (),
    pub last_input_time: Box<NetworkTimeSinceEpoch>,
    pub sources: (),
}
pub struct TimelineEvent {
    pub frame_id: Box<crate::page::FrameId>,
    pub _type: Box<String>,
    pub name: Box<String>,
    pub time: Box<NetworkTimeSinceEpoch>,
    pub duration: Box<u64>,
    pub lcp_details: Box<LargestContentfulPaint>,
    pub layout_shift_details: Box<LayoutShift>,
}
