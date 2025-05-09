use crate::page::*;
/// Encoding options for a screenshot.
/// <https://chromedevtools.github.io/devtools-protocol/tot/HeadlessExperimental/#type-ScreenshotParams>
pub struct HeadlessExperimentalScreenshotParams {
    pub format: String,
    pub quality: i64,
    pub optimize_for_speed: (),
}
