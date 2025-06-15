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
/// Returns the following OS state for the given manifest id.
pub struct PwaGetOsAppStateParams {
    pub manifest_id: String,
}
/// Returns the following OS state for the given manifest id.
pub type PwaGetOsAppStateReturns = ();
/** Installs the given manifest identity, optionally using the given install_url
or IWA bundle location.

TODO(crbug.com/337872319) Support IWA to meet the following specific
requirement.
IWA-specific install description: If the manifest_id is isolated-app://,
install_url_or_bundle_url is required, and can be either an http(s) URL or
file:// URL pointing to a signed web bundle (.swbn). The .swbn file's
signing key must correspond to manifest_id. If Chrome is not in IWA dev
mode, the installation will fail, regardless of the state of the allowlist.*/
pub struct PwaInstallParams {
    pub manifest_id: String,
    pub install_url_or_bundle_url: String,
}
/** Installs the given manifest identity, optionally using the given install_url
or IWA bundle location.

TODO(crbug.com/337872319) Support IWA to meet the following specific
requirement.
IWA-specific install description: If the manifest_id is isolated-app://,
install_url_or_bundle_url is required, and can be either an http(s) URL or
file:// URL pointing to a signed web bundle (.swbn). The .swbn file's
signing key must correspond to manifest_id. If Chrome is not in IWA dev
mode, the installation will fail, regardless of the state of the allowlist.*/
pub type PwaInstallReturns = ();
/// Uninstalls the given manifest_id and closes any opened app windows.
pub struct PwaUninstallParams {
    pub manifest_id: String,
}
/// Uninstalls the given manifest_id and closes any opened app windows.
pub type PwaUninstallReturns = ();
/** Launches the installed web app, or an url in the same web app instead of the
default start url if it is provided. Returns a page Target.TargetID which
can be used to attach to via Target.attachToTarget or similar APIs.*/
pub struct PwaLaunchParams {
    pub manifest_id: String,
    pub url: String,
}
/** Launches the installed web app, or an url in the same web app instead of the
default start url if it is provided. Returns a page Target.TargetID which
can be used to attach to via Target.attachToTarget or similar APIs.*/
pub type PwaLaunchReturns = ();
/** Opens one or more local files from an installed web app identified by its
manifestId. The web app needs to have file handlers registered to process
the files. The API returns one or more page Target.TargetIDs which can be
used to attach to via Target.attachToTarget or similar APIs.
If some files in the parameters cannot be handled by the web app, they will
be ignored. If none of the files can be handled, this API returns an error.
If no files are provided as the parameter, this API also returns an error.

According to the definition of the file handlers in the manifest file, one
Target.TargetID may represent a page handling one or more files. The order
of the returned Target.TargetIDs is not guaranteed.

TODO(crbug.com/339454034): Check the existences of the input files.*/
pub struct PwaLaunchFilesInAppParams {
    pub manifest_id: String,
    pub files: Vec<String>,
}
/** Opens one or more local files from an installed web app identified by its
manifestId. The web app needs to have file handlers registered to process
the files. The API returns one or more page Target.TargetIDs which can be
used to attach to via Target.attachToTarget or similar APIs.
If some files in the parameters cannot be handled by the web app, they will
be ignored. If none of the files can be handled, this API returns an error.
If no files are provided as the parameter, this API also returns an error.

According to the definition of the file handlers in the manifest file, one
Target.TargetID may represent a page handling one or more files. The order
of the returned Target.TargetIDs is not guaranteed.

TODO(crbug.com/339454034): Check the existences of the input files.*/
pub type PwaLaunchFilesInAppReturns = ();
/** Opens the current page in its web app identified by the manifest id, needs
to be called on a page target. This function returns immediately without
waiting for the app to finish loading.*/
pub struct PwaOpenCurrentPageInAppParams {
    pub manifest_id: String,
}
/** Opens the current page in its web app identified by the manifest id, needs
to be called on a page target. This function returns immediately without
waiting for the app to finish loading.*/
pub type PwaOpenCurrentPageInAppReturns = ();
/** Changes user settings of the web app identified by its manifestId. If the
app was not installed, this command returns an error. Unset parameters will
be ignored; unrecognized values will cause an error.

Unlike the ones defined in the manifest files of the web apps, these
settings are provided by the browser and controlled by the users, they
impact the way the browser handling the web apps.

See the comment of each parameter.*/
pub struct PwaChangeAppUserSettingsParams {
    pub manifest_id: String,
    pub link_capturing: bool,
    pub display_mode: Box<DisplayMode>,
}
/** Changes user settings of the web app identified by its manifestId. If the
app was not installed, this command returns an error. Unset parameters will
be ignored; unrecognized values will cause an error.

Unlike the ones defined in the manifest files of the web apps, these
settings are provided by the browser and controlled by the users, they
impact the way the browser handling the web apps.

See the comment of each parameter.*/
pub type PwaChangeAppUserSettingsReturns = ();
