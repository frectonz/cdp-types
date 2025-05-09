use crate::dom::*;
/// Animation instance.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Animation/#type-Animation>
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
/// <https://chromedevtools.github.io/devtools-protocol/tot/Animation/#type-ViewOrScrollTimeline>
pub struct AnimationViewOrScrollTimeline {
    pub source_node_id: (),
    pub start_offset: u64,
    pub end_offset: u64,
    pub subject_node_id: (),
    pub axis: (),
}
/// AnimationEffect instance
/// <https://chromedevtools.github.io/devtools-protocol/tot/Animation/#type-AnimationEffect>
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
/// <https://chromedevtools.github.io/devtools-protocol/tot/Animation/#type-KeyframesRule>
pub struct AnimationKeyframesRule {
    pub name: String,
    pub keyframes: (),
}
/// Keyframe Style
/// <https://chromedevtools.github.io/devtools-protocol/tot/Animation/#type-KeyframeStyle>
pub struct AnimationKeyframeStyle {
    pub offset: String,
    pub easing: String,
}
