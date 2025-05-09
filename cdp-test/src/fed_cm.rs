/** Whether this is a sign-up or sign-in action for this account, i.e.
whether this account has ever been used to sign in to this RP before.*/
/// <https://chromedevtools.github.io/devtools-protocol/tot/FedCm/#type-LoginState>
pub enum FedCmLoginState {
    SignIn,
    SignUp,
}
/// The types of FedCM dialogs.
/// <https://chromedevtools.github.io/devtools-protocol/tot/FedCm/#type-DialogType>
pub enum FedCmDialogType {
    AccountChooser,
    AutoReauthn,
    ConfirmIdpLogin,
    Error,
}
/// The buttons on the FedCM dialog.
/// <https://chromedevtools.github.io/devtools-protocol/tot/FedCm/#type-DialogButton>
pub enum FedCmDialogButton {
    ConfirmIdpLoginContinue,
    ErrorGotIt,
    ErrorMoreDetails,
}
/// The URLs that each account has
/// <https://chromedevtools.github.io/devtools-protocol/tot/FedCm/#type-AccountUrlType>
pub enum FedCmAccountUrlType {
    TermsOfService,
    PrivacyPolicy,
}
/// Corresponds to IdentityRequestAccount
/// <https://chromedevtools.github.io/devtools-protocol/tot/FedCm/#type-Account>
pub struct FedCmAccount {
    pub account_id: (),
    pub email: (),
    pub name: (),
    pub given_name: (),
    pub picture_url: (),
    pub idp_config_url: (),
    pub idp_login_url: (),
    pub login_state: (),
    pub terms_of_service_url: (),
    pub privacy_policy_url: (),
}
