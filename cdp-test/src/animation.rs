use crate::dom::*;
/// Animation instance.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Animation/#type-Animation>
pub struct Animation {
    pub id: (),
    pub name: (),
    pub paused_state: (),
    pub play_state: (),
    pub playback_rate: (),
    pub start_time: (),
    pub current_time: (),
    pub _type: (),
    pub source: (),
    pub css_id: (),
    pub view_or_scroll_timeline: (),
}
/// Timeline instance
/// <https://chromedevtools.github.io/devtools-protocol/tot/Animation/#type-ViewOrScrollTimeline>
pub struct AnimationViewOrScrollTimeline {
    pub source_node_id: (),
    pub start_offset: (),
    pub end_offset: (),
    pub subject_node_id: (),
    pub axis: (),
}
/// AnimationEffect instance
/// <https://chromedevtools.github.io/devtools-protocol/tot/Animation/#type-AnimationEffect>
pub struct AnimationEffect {
    pub delay: (),
    pub end_delay: (),
    pub iteration_start: (),
    pub iterations: (),
    pub duration: (),
    pub direction: (),
    pub fill: (),
    pub backend_node_id: (),
    pub keyframes_rule: (),
    pub easing: (),
}
/// Keyframes Rule
/// <https://chromedevtools.github.io/devtools-protocol/tot/Animation/#type-KeyframesRule>
pub struct AnimationKeyframesRule {
    pub name: (),
    pub keyframes: (),
}
/// Keyframe Style
/// <https://chromedevtools.github.io/devtools-protocol/tot/Animation/#type-KeyframeStyle>
pub struct AnimationKeyframeStyle {
    pub offset: (),
    pub easing: (),
}
