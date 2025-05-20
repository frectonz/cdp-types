use crate::common::*;
pub struct TouchPoint {
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
pub enum GestureSourceType {
    Default,
    Touch,
    Mouse,
}
pub enum MouseButton {
    None,
    Left,
    Middle,
    Right,
    Back,
    Forward,
}
/// ⚠️ Experimental
pub struct DragDataItem {
    pub mime_type: String,
    pub data: String,
    pub title: String,
    pub base_url: String,
}
/// ⚠️ Experimental
pub struct DragData {
    pub items: Vec<DragDataItem>,
    pub files: Vec<String>,
    pub drag_operations_mask: i64,
}
pub type InputDispatchDragEvent = ();
pub type InputDispatchKeyEvent = ();
pub type InputInsertText = ();
pub type InputImeSetComposition = ();
pub type InputDispatchMouseEvent = ();
pub type InputDispatchTouchEvent = ();
pub type InputCancelDragging = ();
pub type InputEmulateTouchFromMouseEvent = ();
pub type InputSetIgnoreInputEvents = ();
pub type InputSetInterceptDrags = ();
pub type InputSynthesizePinchGesture = ();
pub type InputSynthesizeScrollGesture = ();
pub type InputSynthesizeTapGesture = ();
