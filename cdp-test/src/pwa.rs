/** The following types are the replica of
https://crsrc.org/c/chrome/browser/web_applications/proto/web_app_os_integration_state.proto;drc=9910d3be894c8f142c977ba1023f30a656bc13fc;l=67*/
/// <https://chromedevtools.github.io/devtools-protocol/tot/PWA/#type-FileHandlerAccept>
pub struct PwaFileHandlerAccept {
    pub media_type: (),
    pub file_extensions: (),
}
/// <https://chromedevtools.github.io/devtools-protocol/tot/PWA/#type-FileHandler>
pub struct PwaFileHandler {
    pub action: (),
    pub accepts: (),
    pub display_name: (),
}
/// If user prefers opening the app in browser or an app window.
/// <https://chromedevtools.github.io/devtools-protocol/tot/PWA/#type-DisplayMode>
pub enum PwaDisplayMode {
    Standalone,
    Browser,
}
