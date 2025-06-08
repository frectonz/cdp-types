use crate::common::*;
use crate::page::*;
/// Encoding options for a screenshot.
pub struct ScreenshotParams {
    pub format: String,
    pub quality: i64,
    pub optimize_for_speed: bool,
}
/** Sends a BeginFrame to the target and returns when the frame was completed. Optionally captures a
screenshot from the resulting frame. Requires that the target was created with enabled
BeginFrameControl. Designed for use with --run-all-compositor-stages-before-draw, see also
https://goo.gle/chrome-headless-rendering for more background.*/
pub type HeadlessExperimentalBeginFrame = ();
#[deprecated]
/// Disables headless events for the target.
pub type HeadlessExperimentalDisable = ();
#[deprecated]
/// Enables headless events for the target.
pub type HeadlessExperimentalEnable = ();
