use crate::common::*;
pub struct TouchPoint {
    pub x: Box<u64>,
    pub y: Box<u64>,
    pub radius_x: Box<u64>,
    pub radius_y: Box<u64>,
    pub rotation_angle: Box<u64>,
    pub force: Box<u64>,
    pub tangential_pressure: Box<u64>,
    pub tilt_x: Box<u64>,
    pub tilt_y: Box<u64>,
    pub twist: Box<i64>,
    pub id: Box<u64>,
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
    pub mime_type: Box<String>,
    pub data: Box<String>,
    pub title: Box<String>,
    pub base_url: Box<String>,
}
/// ⚠️ Experimental
pub struct DragData {
    pub items: (),
    pub files: (),
    pub drag_operations_mask: Box<i64>,
}
