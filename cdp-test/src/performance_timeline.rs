use crate::dom::*;
use crate::network::*;
/// See https://github.com/WICG/LargestContentfulPaint and largest_contentful_paint.idl
/// <https://chromedevtools.github.io/devtools-protocol/tot/PerformanceTimeline/#type-LargestContentfulPaint>
pub struct PerformanceTimelineLargestContentfulPaint {
    pub render_time: (),
    pub load_time: (),
    pub size: u64,
    pub element_id: String,
    pub url: String,
    pub node_id: (),
}
/// <https://chromedevtools.github.io/devtools-protocol/tot/PerformanceTimeline/#type-LayoutShiftAttribution>
pub struct PerformanceTimelineLayoutShiftAttribution {
    pub previous_rect: (),
    pub current_rect: (),
    pub node_id: (),
}
/// See https://wicg.github.io/layout-instability/#sec-layout-shift and layout_shift.idl
/// <https://chromedevtools.github.io/devtools-protocol/tot/PerformanceTimeline/#type-LayoutShift>
pub struct PerformanceTimelineLayoutShift {
    pub value: u64,
    pub had_recent_input: (),
    pub last_input_time: (),
    pub sources: (),
}
/// <https://chromedevtools.github.io/devtools-protocol/tot/PerformanceTimeline/#type-TimelineEvent>
pub struct PerformanceTimelineTimelineEvent {
    pub frame_id: (),
    pub _type: String,
    pub name: String,
    pub time: (),
    pub duration: u64,
    pub lcp_details: (),
    pub layout_shift_details: (),
}
