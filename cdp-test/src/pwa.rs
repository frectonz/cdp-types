use crate::common::*;
/** The following types are the replica of
https://crsrc.org/c/chrome/browser/web_applications/proto/web_app_os_integration_state.proto;drc=9910d3be894c8f142c977ba1023f30a656bc13fc;l=67*/
pub struct FileHandlerAccept {
    pub media_type: String,
    pub file_extensions: Vec<String>,
}
/// If user prefers opening the app in browser or an app window.
pub enum DisplayMode {
    Standalone,
    Browser,
}
pub type PwaGetOsAppState = ();
pub type PwaInstall = ();
pub type PwaUninstall = ();
pub type PwaLaunch = ();
pub type PwaLaunchFilesInApp = ();
pub type PwaOpenCurrentPageInApp = ();
pub type PwaChangeAppUserSettings = ();
