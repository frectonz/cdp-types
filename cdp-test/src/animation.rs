pub use crate::common::*;
use crate::dom::*;
/// Animation instance.
pub struct Animation {
    pub id: String,
    pub name: String,
    pub paused_state: (),
    pub play_state: String,
    pub playback_rate: u64,
    pub start_time: u64,
    pub current_time: u64,
    pub _type: String,
    pub source: (),
    pub css_id: String,
    pub view_or_scroll_timeline: (),
}
/// Timeline instance
pub struct ViewOrScrollTimeline {
    pub source_node_id: (),
    pub start_offset: u64,
    pub end_offset: u64,
    pub subject_node_id: (),
    pub axis: (),
}
/// AnimationEffect instance
pub struct AnimationEffect {
    pub delay: u64,
    pub end_delay: u64,
    pub iteration_start: u64,
    pub iterations: u64,
    pub duration: u64,
    pub direction: String,
    pub fill: String,
    pub backend_node_id: (),
    pub keyframes_rule: (),
    pub easing: String,
}
/// Keyframes Rule
pub struct KeyframesRule {
    pub name: String,
    pub keyframes: (),
}
/// Keyframe Style
pub struct KeyframeStyle {
    pub offset: String,
    pub easing: String,
}
