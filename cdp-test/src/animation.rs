use crate::common::*;
use crate::dom::*;
/// Animation instance.
pub struct Animation {
    pub id: Box<String>,
    pub name: Box<String>,
    pub paused_state: (),
    pub play_state: Box<String>,
    pub playback_rate: Box<u64>,
    pub start_time: Box<u64>,
    pub current_time: Box<u64>,
    pub _type: Box<String>,
    pub source: Box<AnimationEffect>,
    pub css_id: Box<String>,
    pub view_or_scroll_timeline: Box<ViewOrScrollTimeline>,
}
/// Timeline instance
pub struct ViewOrScrollTimeline {
    pub source_node_id: Box<BackendNodeId>,
    pub start_offset: Box<u64>,
    pub end_offset: Box<u64>,
    pub subject_node_id: Box<BackendNodeId>,
    pub axis: Box<ScrollOrientation>,
}
/// AnimationEffect instance
pub struct AnimationEffect {
    pub delay: Box<u64>,
    pub end_delay: Box<u64>,
    pub iteration_start: Box<u64>,
    pub iterations: Box<u64>,
    pub duration: Box<u64>,
    pub direction: Box<String>,
    pub fill: Box<String>,
    pub backend_node_id: Box<BackendNodeId>,
    pub keyframes_rule: Box<KeyframesRule>,
    pub easing: Box<String>,
}
/// Keyframes Rule
pub struct KeyframesRule {
    pub name: Box<String>,
    pub keyframes: (),
}
/// Keyframe Style
pub struct KeyframeStyle {
    pub offset: Box<String>,
    pub easing: Box<String>,
}
