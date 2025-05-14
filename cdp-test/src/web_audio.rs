pub use crate::common::*;
/// An unique ID for a graph object (AudioContext, AudioNode, AudioParam) in Web Audio API
pub struct GraphObjectId(String);
/// Enum of BaseAudioContext types
pub enum ContextType {
    Realtime,
    Offline,
}
/// Enum of AudioContextState from the spec
pub enum ContextState {
    Suspended,
    Running,
    Closed,
    Interrupted,
}
/// Enum of AudioNode types
pub struct NodeType(String);
/// Enum of AudioNode::ChannelCountMode from the spec
pub enum ChannelCountMode {
    ClampedMax,
    Explicit,
    Max,
}
/// Enum of AudioNode::ChannelInterpretation from the spec
pub enum ChannelInterpretation {
    Discrete,
    Speakers,
}
/// Enum of AudioParam types
pub struct ParamType(String);
/// Enum of AudioParam::AutomationRate from the spec
pub enum AutomationRate {
    ARate,
    KRate,
}
/// Fields in AudioContext that change in real-time.
pub struct ContextRealtimeData {
    pub current_time: u64,
    pub render_capacity: u64,
    pub callback_interval_mean: u64,
    pub callback_interval_variance: u64,
}
/// Protocol object for BaseAudioContext
pub struct BaseAudioContext {
    pub context_id: (),
    pub context_type: (),
    pub context_state: (),
    pub realtime_data: (),
    pub callback_buffer_size: u64,
    pub max_output_channel_count: u64,
    pub sample_rate: u64,
}
/// Protocol object for AudioListener
pub struct AudioListener {
    pub listener_id: (),
    pub context_id: (),
}
/// Protocol object for AudioNode
pub struct AudioNode {
    pub node_id: (),
    pub context_id: (),
    pub node_type: (),
    pub number_of_inputs: u64,
    pub number_of_outputs: u64,
    pub channel_count: u64,
    pub channel_count_mode: (),
    pub channel_interpretation: (),
}
/// Protocol object for AudioParam
pub struct AudioParam {
    pub param_id: (),
    pub node_id: (),
    pub context_id: (),
    pub param_type: (),
    pub rate: (),
    pub default_value: u64,
    pub min_value: u64,
    pub max_value: u64,
}
