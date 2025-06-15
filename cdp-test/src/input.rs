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
/// ⚠️ Experimental
/// Dispatches a drag event into the page.
pub struct InputDispatchDragEventParams {
    pub _type: String,
    pub x: u64,
    pub y: u64,
    pub data: Box<DragData>,
    pub modifiers: i64,
}
/// ⚠️ Experimental
/// Dispatches a drag event into the page.
pub type InputDispatchDragEventReturns = ();
/// Dispatches a key event to the page.
pub struct InputDispatchKeyEventParams {
    pub _type: String,
    pub modifiers: i64,
    pub timestamp: Box<NetworkTimeSinceEpoch>,
    pub text: String,
    pub unmodified_text: String,
    pub key_identifier: String,
    pub code: String,
    pub key: String,
    pub windows_virtual_key_code: i64,
    pub native_virtual_key_code: i64,
    pub auto_repeat: bool,
    pub is_keypad: bool,
    pub is_system_key: bool,
    pub location: i64,
    pub commands: Vec<String>,
}
/// Dispatches a key event to the page.
pub type InputDispatchKeyEventReturns = ();
/// ⚠️ Experimental
/** This method emulates inserting text that doesn't come from a key press,
for example an emoji keyboard or an IME.*/
pub struct InputInsertTextParams {
    pub text: String,
}
/// ⚠️ Experimental
/** This method emulates inserting text that doesn't come from a key press,
for example an emoji keyboard or an IME.*/
pub type InputInsertTextReturns = ();
/// ⚠️ Experimental
/** This method sets the current candidate text for IME.
Use imeCommitComposition to commit the final text.
Use imeSetComposition with empty string as text to cancel composition.*/
pub struct InputImeSetCompositionParams {
    pub text: String,
    pub selection_start: i64,
    pub selection_end: i64,
    pub replacement_start: i64,
    pub replacement_end: i64,
}
/// ⚠️ Experimental
/** This method sets the current candidate text for IME.
Use imeCommitComposition to commit the final text.
Use imeSetComposition with empty string as text to cancel composition.*/
pub type InputImeSetCompositionReturns = ();
/// Dispatches a mouse event to the page.
pub struct InputDispatchMouseEventParams {
    pub _type: String,
    pub x: u64,
    pub y: u64,
    pub modifiers: i64,
    pub timestamp: Box<NetworkTimeSinceEpoch>,
    pub button: Box<MouseButton>,
    pub buttons: i64,
    pub click_count: i64,
    pub force: u64,
    pub tangential_pressure: u64,
    pub tilt_x: u64,
    pub tilt_y: u64,
    pub twist: i64,
    pub delta_x: u64,
    pub delta_y: u64,
    pub pointer_type: String,
}
/// Dispatches a mouse event to the page.
pub type InputDispatchMouseEventReturns = ();
/// Dispatches a touch event to the page.
pub struct InputDispatchTouchEventParams {
    pub _type: String,
    pub touch_points: Vec<TouchPoint>,
    pub modifiers: i64,
    pub timestamp: Box<NetworkTimeSinceEpoch>,
}
/// Dispatches a touch event to the page.
pub type InputDispatchTouchEventReturns = ();
/// Cancels any active dragging in the page.
pub type InputCancelDraggingParams = ();
/// Cancels any active dragging in the page.
pub type InputCancelDraggingReturns = ();
/// ⚠️ Experimental
/// Emulates touch event from the mouse event parameters.
pub struct InputEmulateTouchFromMouseEventParams {
    pub _type: String,
    pub x: i64,
    pub y: i64,
    pub button: Box<MouseButton>,
    pub timestamp: Box<NetworkTimeSinceEpoch>,
    pub delta_x: u64,
    pub delta_y: u64,
    pub modifiers: i64,
    pub click_count: i64,
}
/// ⚠️ Experimental
/// Emulates touch event from the mouse event parameters.
pub type InputEmulateTouchFromMouseEventReturns = ();
/// Ignores input events (useful while auditing page).
pub struct InputSetIgnoreInputEventsParams {
    pub ignore: bool,
}
/// Ignores input events (useful while auditing page).
pub type InputSetIgnoreInputEventsReturns = ();
/// ⚠️ Experimental
/** Prevents default drag and drop behavior and instead emits `Input.dragIntercepted` events.
Drag and drop behavior can be directly controlled via `Input.dispatchDragEvent`.*/
pub struct InputSetInterceptDragsParams {
    pub enabled: bool,
}
/// ⚠️ Experimental
/** Prevents default drag and drop behavior and instead emits `Input.dragIntercepted` events.
Drag and drop behavior can be directly controlled via `Input.dispatchDragEvent`.*/
pub type InputSetInterceptDragsReturns = ();
/// ⚠️ Experimental
/// Synthesizes a pinch gesture over a time period by issuing appropriate touch events.
pub struct InputSynthesizePinchGestureParams {
    pub x: u64,
    pub y: u64,
    pub scale_factor: u64,
    pub relative_speed: i64,
    pub gesture_source_type: Box<GestureSourceType>,
}
/// ⚠️ Experimental
/// Synthesizes a pinch gesture over a time period by issuing appropriate touch events.
pub type InputSynthesizePinchGestureReturns = ();
/// ⚠️ Experimental
/// Synthesizes a scroll gesture over a time period by issuing appropriate touch events.
pub struct InputSynthesizeScrollGestureParams {
    pub x: u64,
    pub y: u64,
    pub x_distance: u64,
    pub y_distance: u64,
    pub x_overscroll: u64,
    pub y_overscroll: u64,
    pub prevent_fling: bool,
    pub speed: i64,
    pub gesture_source_type: Box<GestureSourceType>,
    pub repeat_count: i64,
    pub repeat_delay_ms: i64,
    pub interaction_marker_name: String,
}
/// ⚠️ Experimental
/// Synthesizes a scroll gesture over a time period by issuing appropriate touch events.
pub type InputSynthesizeScrollGestureReturns = ();
/// ⚠️ Experimental
/// Synthesizes a tap gesture over a time period by issuing appropriate touch events.
pub struct InputSynthesizeTapGestureParams {
    pub x: u64,
    pub y: u64,
    pub duration: i64,
    pub tap_count: i64,
    pub gesture_source_type: Box<GestureSourceType>,
}
/// ⚠️ Experimental
/// Synthesizes a tap gesture over a time period by issuing appropriate touch events.
pub type InputSynthesizeTapGestureReturns = ();
