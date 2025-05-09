/// <https://chromedevtools.github.io/devtools-protocol/tot/Input/#type-TouchPoint>
pub struct InputTouchPoint {
    pub x: (),
    pub y: (),
    pub radius_x: (),
    pub radius_y: (),
    pub rotation_angle: (),
    pub force: (),
    pub tangential_pressure: (),
    pub tilt_x: (),
    pub tilt_y: (),
    pub twist: (),
    pub id: (),
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
    pub mime_type: (),
    pub data: (),
    pub title: (),
    pub base_url: (),
}
/// ⚠️ Experimental
/// <https://chromedevtools.github.io/devtools-protocol/tot/Input/#type-DragData>
pub struct InputDragData {
    pub items: (),
    pub files: (),
    pub drag_operations_mask: (),
}
