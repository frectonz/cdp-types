use crate::common::*;
use crate::page::*;
/// Encoding options for a screenshot.
pub struct ScreenshotParams {
    pub format: Box<String>,
    pub quality: Box<i64>,
    pub optimize_for_speed: (),
}
