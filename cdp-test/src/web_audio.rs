/// An unique ID for a graph object (AudioContext, AudioNode, AudioParam) in Web Audio API
/// <https://chromedevtools.github.io/devtools-protocol/tot/WebAudio/#type-GraphObjectId>
pub struct WebAudioGraphObjectId(String);
/// Enum of BaseAudioContext types
/// <https://chromedevtools.github.io/devtools-protocol/tot/WebAudio/#type-ContextType>
pub enum WebAudioContextType {
    Realtime,
    Offline,
}
/// Enum of AudioContextState from the spec
/// <https://chromedevtools.github.io/devtools-protocol/tot/WebAudio/#type-ContextState>
pub enum WebAudioContextState {
    Suspended,
    Running,
    Closed,
    Interrupted,
}
/// Enum of AudioNode types
/// <https://chromedevtools.github.io/devtools-protocol/tot/WebAudio/#type-NodeType>
pub struct WebAudioNodeType(String);
/// Enum of AudioNode::ChannelCountMode from the spec
/// <https://chromedevtools.github.io/devtools-protocol/tot/WebAudio/#type-ChannelCountMode>
pub enum WebAudioChannelCountMode {
    ClampedMax,
    Explicit,
    Max,
}
/// Enum of AudioNode::ChannelInterpretation from the spec
/// <https://chromedevtools.github.io/devtools-protocol/tot/WebAudio/#type-ChannelInterpretation>
pub enum WebAudioChannelInterpretation {
    Discrete,
    Speakers,
}
/// Enum of AudioParam types
/// <https://chromedevtools.github.io/devtools-protocol/tot/WebAudio/#type-ParamType>
pub struct WebAudioParamType(String);
/// Enum of AudioParam::AutomationRate from the spec
/// <https://chromedevtools.github.io/devtools-protocol/tot/WebAudio/#type-AutomationRate>
pub enum WebAudioAutomationRate {
    ARate,
    KRate,
}
/// Fields in AudioContext that change in real-time.
/// <https://chromedevtools.github.io/devtools-protocol/tot/WebAudio/#type-ContextRealtimeData>
pub struct WebAudioContextRealtimeData {
    pub current_time: u64,
    pub render_capacity: u64,
    pub callback_interval_mean: u64,
    pub callback_interval_variance: u64,
}
/// Protocol object for BaseAudioContext
/// <https://chromedevtools.github.io/devtools-protocol/tot/WebAudio/#type-BaseAudioContext>
pub struct WebAudioBaseAudioContext {
    pub context_id: (),
    pub context_type: (),
    pub context_state: (),
    pub realtime_data: (),
    pub callback_buffer_size: u64,
    pub max_output_channel_count: u64,
    pub sample_rate: u64,
}
/// Protocol object for AudioListener
/// <https://chromedevtools.github.io/devtools-protocol/tot/WebAudio/#type-AudioListener>
pub struct WebAudioAudioListener {
    pub listener_id: (),
    pub context_id: (),
}
/// Protocol object for AudioNode
/// <https://chromedevtools.github.io/devtools-protocol/tot/WebAudio/#type-AudioNode>
pub struct WebAudioAudioNode {
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
/// <https://chromedevtools.github.io/devtools-protocol/tot/WebAudio/#type-AudioParam>
pub struct WebAudioAudioParam {
    pub param_id: (),
    pub node_id: (),
    pub context_id: (),
    pub param_type: (),
    pub rate: (),
    pub default_value: u64,
    pub min_value: u64,
    pub max_value: u64,
}
