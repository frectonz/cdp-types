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
pub type InputDispatchDragEventParams = ();
/// ⚠️ Experimental
/// Dispatches a drag event into the page.
pub type InputDispatchDragEventResults = ();
/// Dispatches a key event to the page.
pub type InputDispatchKeyEventParams = ();
/// Dispatches a key event to the page.
pub type InputDispatchKeyEventResults = ();
/// ⚠️ Experimental
/** This method emulates inserting text that doesn't come from a key press,
for example an emoji keyboard or an IME.*/
pub type InputInsertTextParams = ();
/// ⚠️ Experimental
/** This method emulates inserting text that doesn't come from a key press,
for example an emoji keyboard or an IME.*/
pub type InputInsertTextResults = ();
/// ⚠️ Experimental
/** This method sets the current candidate text for IME.
Use imeCommitComposition to commit the final text.
Use imeSetComposition with empty string as text to cancel composition.*/
pub type InputImeSetCompositionParams = ();
/// ⚠️ Experimental
/** This method sets the current candidate text for IME.
Use imeCommitComposition to commit the final text.
Use imeSetComposition with empty string as text to cancel composition.*/
pub type InputImeSetCompositionResults = ();
/// Dispatches a mouse event to the page.
pub type InputDispatchMouseEventParams = ();
/// Dispatches a mouse event to the page.
pub type InputDispatchMouseEventResults = ();
/// Dispatches a touch event to the page.
pub type InputDispatchTouchEventParams = ();
/// Dispatches a touch event to the page.
pub type InputDispatchTouchEventResults = ();
/// Cancels any active dragging in the page.
pub type InputCancelDraggingParams = ();
/// Cancels any active dragging in the page.
pub type InputCancelDraggingResults = ();
/// ⚠️ Experimental
/// Emulates touch event from the mouse event parameters.
pub type InputEmulateTouchFromMouseEventParams = ();
/// ⚠️ Experimental
/// Emulates touch event from the mouse event parameters.
pub type InputEmulateTouchFromMouseEventResults = ();
/// Ignores input events (useful while auditing page).
pub type InputSetIgnoreInputEventsParams = ();
/// Ignores input events (useful while auditing page).
pub type InputSetIgnoreInputEventsResults = ();
/// ⚠️ Experimental
/** Prevents default drag and drop behavior and instead emits `Input.dragIntercepted` events.
Drag and drop behavior can be directly controlled via `Input.dispatchDragEvent`.*/
pub type InputSetInterceptDragsParams = ();
/// ⚠️ Experimental
/** Prevents default drag and drop behavior and instead emits `Input.dragIntercepted` events.
Drag and drop behavior can be directly controlled via `Input.dispatchDragEvent`.*/
pub type InputSetInterceptDragsResults = ();
/// ⚠️ Experimental
/// Synthesizes a pinch gesture over a time period by issuing appropriate touch events.
pub type InputSynthesizePinchGestureParams = ();
/// ⚠️ Experimental
/// Synthesizes a pinch gesture over a time period by issuing appropriate touch events.
pub type InputSynthesizePinchGestureResults = ();
/// ⚠️ Experimental
/// Synthesizes a scroll gesture over a time period by issuing appropriate touch events.
pub type InputSynthesizeScrollGestureParams = ();
/// ⚠️ Experimental
/// Synthesizes a scroll gesture over a time period by issuing appropriate touch events.
pub type InputSynthesizeScrollGestureResults = ();
/// ⚠️ Experimental
/// Synthesizes a tap gesture over a time period by issuing appropriate touch events.
pub type InputSynthesizeTapGestureParams = ();
/// ⚠️ Experimental
/// Synthesizes a tap gesture over a time period by issuing appropriate touch events.
pub type InputSynthesizeTapGestureResults = ();
