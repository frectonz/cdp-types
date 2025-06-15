use crate::common::*;
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
    pub context_id: Box<GraphObjectId>,
    pub context_type: Box<ContextType>,
    pub context_state: Box<ContextState>,
    pub realtime_data: Box<ContextRealtimeData>,
    pub callback_buffer_size: u64,
    pub max_output_channel_count: u64,
    pub sample_rate: u64,
}
/// Protocol object for AudioListener
pub struct AudioListener {
    pub listener_id: Box<GraphObjectId>,
    pub context_id: Box<GraphObjectId>,
}
/// Protocol object for AudioNode
pub struct AudioNode {
    pub node_id: Box<GraphObjectId>,
    pub context_id: Box<GraphObjectId>,
    pub node_type: Box<NodeType>,
    pub number_of_inputs: u64,
    pub number_of_outputs: u64,
    pub channel_count: u64,
    pub channel_count_mode: Box<ChannelCountMode>,
    pub channel_interpretation: Box<ChannelInterpretation>,
}
/// Protocol object for AudioParam
pub struct AudioParam {
    pub param_id: Box<GraphObjectId>,
    pub node_id: Box<GraphObjectId>,
    pub context_id: Box<GraphObjectId>,
    pub param_type: Box<ParamType>,
    pub rate: Box<AutomationRate>,
    pub default_value: u64,
    pub min_value: u64,
    pub max_value: u64,
}
/// Enables the WebAudio domain and starts sending context lifetime events.
pub type WebAudioEnableParams = ();
/// Enables the WebAudio domain and starts sending context lifetime events.
pub type WebAudioEnableReturns = ();
/// Disables the WebAudio domain.
pub type WebAudioDisableParams = ();
/// Disables the WebAudio domain.
pub type WebAudioDisableReturns = ();
/// Fetch the realtime data from the registered contexts.
pub struct WebAudioGetRealtimeDataParams {
    pub context_id: Box<GraphObjectId>,
}
/// Fetch the realtime data from the registered contexts.
pub struct WebAudioGetRealtimeDataParams {
    pub realtime_data: Box<ContextRealtimeData>,
}
/// Notifies that a new BaseAudioContext has been created.
pub struct WebAudioContextCreatedEvent {
    pub context: Box<BaseAudioContext>,
}
/// Notifies that an existing BaseAudioContext will be destroyed.
pub struct WebAudioContextWillBeDestroyedEvent {
    pub context_id: Box<GraphObjectId>,
}
/// Notifies that existing BaseAudioContext has changed some properties (id stays the same)..
pub struct WebAudioContextChangedEvent {
    pub context: Box<BaseAudioContext>,
}
/// Notifies that the construction of an AudioListener has finished.
pub struct WebAudioAudioListenerCreatedEvent {
    pub listener: Box<AudioListener>,
}
/// Notifies that a new AudioListener has been created.
pub struct WebAudioAudioListenerWillBeDestroyedEvent {
    pub context_id: Box<GraphObjectId>,
    pub listener_id: Box<GraphObjectId>,
}
/// Notifies that a new AudioNode has been created.
pub struct WebAudioAudioNodeCreatedEvent {
    pub node: Box<AudioNode>,
}
/// Notifies that an existing AudioNode has been destroyed.
pub struct WebAudioAudioNodeWillBeDestroyedEvent {
    pub context_id: Box<GraphObjectId>,
    pub node_id: Box<GraphObjectId>,
}
/// Notifies that a new AudioParam has been created.
pub struct WebAudioAudioParamCreatedEvent {
    pub param: Box<AudioParam>,
}
/// Notifies that an existing AudioParam has been destroyed.
pub struct WebAudioAudioParamWillBeDestroyedEvent {
    pub context_id: Box<GraphObjectId>,
    pub node_id: Box<GraphObjectId>,
    pub param_id: Box<GraphObjectId>,
}
/// Notifies that two AudioNodes are connected.
pub struct WebAudioNodesConnectedEvent {
    pub context_id: Box<GraphObjectId>,
    pub source_id: Box<GraphObjectId>,
    pub destination_id: Box<GraphObjectId>,
    pub source_output_index: u64,
    pub destination_input_index: u64,
}
/// Notifies that AudioNodes are disconnected. The destination can be null, and it means all the outgoing connections from the source are disconnected.
pub struct WebAudioNodesDisconnectedEvent {
    pub context_id: Box<GraphObjectId>,
    pub source_id: Box<GraphObjectId>,
    pub destination_id: Box<GraphObjectId>,
    pub source_output_index: u64,
    pub destination_input_index: u64,
}
/// Notifies that an AudioNode is connected to an AudioParam.
pub struct WebAudioNodeParamConnectedEvent {
    pub context_id: Box<GraphObjectId>,
    pub source_id: Box<GraphObjectId>,
    pub destination_id: Box<GraphObjectId>,
    pub source_output_index: u64,
}
/// Notifies that an AudioNode is disconnected to an AudioParam.
pub struct WebAudioNodeParamDisconnectedEvent {
    pub context_id: Box<GraphObjectId>,
    pub source_id: Box<GraphObjectId>,
    pub destination_id: Box<GraphObjectId>,
    pub source_output_index: u64,
}
