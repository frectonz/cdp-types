pub use crate::common::*;
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
    pub items: (),
    pub files: (),
    pub drag_operations_mask: i64,
}
