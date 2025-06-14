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
pub struct HeadlessExperimentalBeginFrameParams {
    pub frame_time_ticks: u64,
    pub interval: u64,
    pub no_display_updates: bool,
    pub screenshot: Box<ScreenshotParams>,
}
/** Sends a BeginFrame to the target and returns when the frame was completed. Optionally captures a
screenshot from the resulting frame. Requires that the target was created with enabled
BeginFrameControl. Designed for use with --run-all-compositor-stages-before-draw, see also
https://goo.gle/chrome-headless-rendering for more background.*/
pub struct HeadlessExperimentalBeginFrameParams {
    pub has_damage: bool,
    pub screenshot_data: String,
}
#[deprecated]
/// Disables headless events for the target.
pub type HeadlessExperimentalDisableParams = ();
#[deprecated]
/// Disables headless events for the target.
pub type HeadlessExperimentalDisableReturns = ();
#[deprecated]
/// Enables headless events for the target.
pub type HeadlessExperimentalEnableParams = ();
#[deprecated]
/// Enables headless events for the target.
pub type HeadlessExperimentalEnableReturns = ();
