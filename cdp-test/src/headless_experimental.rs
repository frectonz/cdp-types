use crate::common::*;
use crate::page::*;
/// Encoding options for a screenshot.
pub struct ScreenshotParams {
    pub format: String,
    pub quality: i64,
    pub optimize_for_speed: bool,
}
