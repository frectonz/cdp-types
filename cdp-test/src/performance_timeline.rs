pub use crate::common::*;
use crate::dom::*;
use crate::network::*;
/// See https://github.com/WICG/LargestContentfulPaint and largest_contentful_paint.idl
pub struct LargestContentfulPaint {
    pub render_time: (),
    pub load_time: (),
    pub size: u64,
    pub element_id: String,
    pub url: String,
    pub node_id: (),
}
pub struct LayoutShiftAttribution {
    pub previous_rect: (),
    pub current_rect: (),
    pub node_id: (),
}
/// See https://wicg.github.io/layout-instability/#sec-layout-shift and layout_shift.idl
pub struct LayoutShift {
    pub value: u64,
    pub had_recent_input: (),
    pub last_input_time: (),
    pub sources: (),
}
pub struct TimelineEvent {
    pub frame_id: (),
    pub _type: String,
    pub name: String,
    pub time: (),
    pub duration: u64,
    pub lcp_details: (),
    pub layout_shift_details: (),
}
