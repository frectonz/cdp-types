/// <https://chromedevtools.github.io/devtools-protocol/tot/Input/#type-TouchPoint>
pub struct InputTouchPoint {
    pub x: u64,
    pub y: u64,
    pub radius_x: u64,
    pub radius_y: u64,
    pub rotation_angle: u64,
    pub force: u64,
    pub tangential_pressure: u64,
    pub tilt_x: u64,
    pub tilt_y: u64,
    pub twist: i64,
    pub id: u64,
}
/// ⚠️ Experimental
/// <https://chromedevtools.github.io/devtools-protocol/tot/Input/#type-GestureSourceType>
pub enum InputGestureSourceType {
    Default,
    Touch,
    Mouse,
}
/// <https://chromedevtools.github.io/devtools-protocol/tot/Input/#type-MouseButton>
pub enum InputMouseButton {
    None,
    Left,
    Middle,
    Right,
    Back,
    Forward,
}
/// UTC time in seconds, counted from January 1, 1970.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Input/#type-TimeSinceEpoch>
pub struct InputTimeSinceEpoch(u64);
/// ⚠️ Experimental
/// <https://chromedevtools.github.io/devtools-protocol/tot/Input/#type-DragDataItem>
pub struct InputDragDataItem {
    pub mime_type: String,
    pub data: String,
    pub title: String,
    pub base_url: String,
}
/// ⚠️ Experimental
/// <https://chromedevtools.github.io/devtools-protocol/tot/Input/#type-DragData>
pub struct InputDragData {
    pub items: (),
    pub files: (),
    pub drag_operations_mask: i64,
}
