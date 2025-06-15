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
    pub _type: (),
    pub x: (),
    pub y: (),
    pub data: (),
    pub modifiers: (),
}
/// ⚠️ Experimental
/// Dispatches a drag event into the page.
pub type InputDispatchDragEventReturns = ();
/// Dispatches a key event to the page.
pub struct InputDispatchKeyEventParams {
    pub _type: (),
    pub modifiers: (),
    pub timestamp: (),
    pub text: (),
    pub unmodified_text: (),
    pub key_identifier: (),
    pub code: (),
    pub key: (),
    pub windows_virtual_key_code: (),
    pub native_virtual_key_code: (),
    pub auto_repeat: (),
    pub is_keypad: (),
    pub is_system_key: (),
    pub location: (),
    pub commands: (),
}
/// Dispatches a key event to the page.
pub type InputDispatchKeyEventReturns = ();
/// ⚠️ Experimental
/** This method emulates inserting text that doesn't come from a key press,
for example an emoji keyboard or an IME.*/
pub struct InputInsertTextParams {
    pub text: (),
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
    pub text: (),
    pub selection_start: (),
    pub selection_end: (),
    pub replacement_start: (),
    pub replacement_end: (),
}
/// ⚠️ Experimental
/** This method sets the current candidate text for IME.
Use imeCommitComposition to commit the final text.
Use imeSetComposition with empty string as text to cancel composition.*/
pub type InputImeSetCompositionReturns = ();
/// Dispatches a mouse event to the page.
pub struct InputDispatchMouseEventParams {
    pub _type: (),
    pub x: (),
    pub y: (),
    pub modifiers: (),
    pub timestamp: (),
    pub button: (),
    pub buttons: (),
    pub click_count: (),
    pub force: (),
    pub tangential_pressure: (),
    pub tilt_x: (),
    pub tilt_y: (),
    pub twist: (),
    pub delta_x: (),
    pub delta_y: (),
    pub pointer_type: (),
}
/// Dispatches a mouse event to the page.
pub type InputDispatchMouseEventReturns = ();
/// Dispatches a touch event to the page.
pub struct InputDispatchTouchEventParams {
    pub _type: (),
    pub touch_points: (),
    pub modifiers: (),
    pub timestamp: (),
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
    pub _type: (),
    pub x: (),
    pub y: (),
    pub button: (),
    pub timestamp: (),
    pub delta_x: (),
    pub delta_y: (),
    pub modifiers: (),
    pub click_count: (),
}
/// ⚠️ Experimental
/// Emulates touch event from the mouse event parameters.
pub type InputEmulateTouchFromMouseEventReturns = ();
/// Ignores input events (useful while auditing page).
pub struct InputSetIgnoreInputEventsParams {
    pub ignore: (),
}
/// Ignores input events (useful while auditing page).
pub type InputSetIgnoreInputEventsReturns = ();
/// ⚠️ Experimental
/** Prevents default drag and drop behavior and instead emits `Input.dragIntercepted` events.
Drag and drop behavior can be directly controlled via `Input.dispatchDragEvent`.*/
pub struct InputSetInterceptDragsParams {
    pub enabled: (),
}
/// ⚠️ Experimental
/** Prevents default drag and drop behavior and instead emits `Input.dragIntercepted` events.
Drag and drop behavior can be directly controlled via `Input.dispatchDragEvent`.*/
pub type InputSetInterceptDragsReturns = ();
/// ⚠️ Experimental
/// Synthesizes a pinch gesture over a time period by issuing appropriate touch events.
pub struct InputSynthesizePinchGestureParams {
    pub x: (),
    pub y: (),
    pub scale_factor: (),
    pub relative_speed: (),
    pub gesture_source_type: (),
}
/// ⚠️ Experimental
/// Synthesizes a pinch gesture over a time period by issuing appropriate touch events.
pub type InputSynthesizePinchGestureReturns = ();
/// ⚠️ Experimental
/// Synthesizes a scroll gesture over a time period by issuing appropriate touch events.
pub struct InputSynthesizeScrollGestureParams {
    pub x: (),
    pub y: (),
    pub x_distance: (),
    pub y_distance: (),
    pub x_overscroll: (),
    pub y_overscroll: (),
    pub prevent_fling: (),
    pub speed: (),
    pub gesture_source_type: (),
    pub repeat_count: (),
    pub repeat_delay_ms: (),
    pub interaction_marker_name: (),
}
/// ⚠️ Experimental
/// Synthesizes a scroll gesture over a time period by issuing appropriate touch events.
pub type InputSynthesizeScrollGestureReturns = ();
/// ⚠️ Experimental
/// Synthesizes a tap gesture over a time period by issuing appropriate touch events.
pub struct InputSynthesizeTapGestureParams {
    pub x: (),
    pub y: (),
    pub duration: (),
    pub tap_count: (),
    pub gesture_source_type: (),
}
/// ⚠️ Experimental
/// Synthesizes a tap gesture over a time period by issuing appropriate touch events.
pub type InputSynthesizeTapGestureReturns = ();
