use crate::common::*;
use crate::dom::*;
/// Animation instance.
pub struct Animation {
    pub id: String,
    pub name: String,
    pub paused_state: bool,
    pub play_state: String,
    pub playback_rate: u64,
    pub start_time: u64,
    pub current_time: u64,
    pub _type: String,
    pub source: Box<AnimationEffect>,
    pub css_id: String,
    pub view_or_scroll_timeline: Box<ViewOrScrollTimeline>,
}
/// Timeline instance
pub struct ViewOrScrollTimeline {
    pub source_node_id: Box<BackendNodeId>,
    pub start_offset: u64,
    pub end_offset: u64,
    pub subject_node_id: Box<BackendNodeId>,
    pub axis: Box<ScrollOrientation>,
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
    pub backend_node_id: Box<BackendNodeId>,
    pub keyframes_rule: Box<KeyframesRule>,
    pub easing: String,
}
/// Keyframes Rule
pub struct KeyframesRule {
    pub name: String,
    pub keyframes: Vec<KeyframeStyle>,
}
/// Keyframe Style
pub struct KeyframeStyle {
    pub offset: String,
    pub easing: String,
}
/// Disables animation domain notifications.
pub type AnimationDisableParams = ();
/// Disables animation domain notifications.
pub type AnimationDisableReturns = ();
/// Enables animation domain notifications.
pub type AnimationEnableParams = ();
/// Enables animation domain notifications.
pub type AnimationEnableReturns = ();
/// Returns the current time of the an animation.
pub struct AnimationGetCurrentTimeParams {
    pub id: String,
}
/// Returns the current time of the an animation.
pub struct AnimationGetCurrentTimeParams {
    pub current_time: u64,
}
/// Gets the playback rate of the document timeline.
pub type AnimationGetPlaybackRateParams = ();
/// Gets the playback rate of the document timeline.
pub struct AnimationGetPlaybackRateParams {
    pub playback_rate: u64,
}
/// Releases a set of animations to no longer be manipulated.
pub struct AnimationReleaseAnimationsParams {
    pub animations: Vec<String>,
}
/// Releases a set of animations to no longer be manipulated.
pub type AnimationReleaseAnimationsReturns = ();
/// Gets the remote object of the Animation.
pub struct AnimationResolveAnimationParams {
    pub animation_id: String,
}
/// Gets the remote object of the Animation.
pub struct AnimationResolveAnimationParams {
    pub remote_object: Box<()>,
}
/// Seek a set of animations to a particular time within each animation.
pub struct AnimationSeekAnimationsParams {
    pub animations: Vec<String>,
    pub current_time: u64,
}
/// Seek a set of animations to a particular time within each animation.
pub type AnimationSeekAnimationsReturns = ();
/// Sets the paused state of a set of animations.
pub struct AnimationSetPausedParams {
    pub animations: Vec<String>,
    pub paused: bool,
}
/// Sets the paused state of a set of animations.
pub type AnimationSetPausedReturns = ();
/// Sets the playback rate of the document timeline.
pub struct AnimationSetPlaybackRateParams {
    pub playback_rate: u64,
}
/// Sets the playback rate of the document timeline.
pub type AnimationSetPlaybackRateReturns = ();
/// Sets the timing of an animation node.
pub struct AnimationSetTimingParams {
    pub animation_id: String,
    pub duration: u64,
    pub delay: u64,
}
/// Sets the timing of an animation node.
pub type AnimationSetTimingReturns = ();
