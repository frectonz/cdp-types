use crate::common::*;
use crate::dom::*;
use crate::network::*;
use crate::page::*;
/// See https://github.com/WICG/LargestContentfulPaint and largest_contentful_paint.idl
pub struct LargestContentfulPaint {
    pub render_time: Box<NetworkTimeSinceEpoch>,
    pub load_time: Box<NetworkTimeSinceEpoch>,
    pub size: u64,
    pub element_id: String,
    pub url: String,
    pub node_id: Box<BackendNodeId>,
}
pub struct LayoutShiftAttribution {
    pub previous_rect: Box<Rect>,
    pub current_rect: Box<Rect>,
    pub node_id: Box<BackendNodeId>,
}
/// See https://wicg.github.io/layout-instability/#sec-layout-shift and layout_shift.idl
pub struct LayoutShift {
    pub value: u64,
    pub had_recent_input: bool,
    pub last_input_time: Box<NetworkTimeSinceEpoch>,
    pub sources: Vec<LayoutShiftAttribution>,
}
pub struct TimelineEvent {
    pub frame_id: Box<crate::page::FrameId>,
    pub _type: String,
    pub name: String,
    pub time: Box<NetworkTimeSinceEpoch>,
    pub duration: u64,
    pub lcp_details: Box<LargestContentfulPaint>,
    pub layout_shift_details: Box<LayoutShift>,
}
/** Previously buffered events would be reported before method returns.
See also: timelineEventAdded*/
pub type PerformanceTimelineEnableParams = ();
/** Previously buffered events would be reported before method returns.
See also: timelineEventAdded*/
pub type PerformanceTimelineEnableResults = ();
