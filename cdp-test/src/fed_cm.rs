use crate::common::*;
/** Whether this is a sign-up or sign-in action for this account, i.e.
whether this account has ever been used to sign in to this RP before.*/
pub enum LoginState {
    SignIn,
    SignUp,
}
/// The buttons on the FedCM dialog.
pub enum DialogButton {
    ConfirmIdpLoginContinue,
    ErrorGotIt,
    ErrorMoreDetails,
}
/// The URLs that each account has
pub enum AccountUrlType {
    TermsOfService,
    PrivacyPolicy,
}
/// Corresponds to IdentityRequestAccount
pub struct Account {
    pub account_id: Box<String>,
    pub email: Box<String>,
    pub name: Box<String>,
    pub given_name: Box<String>,
    pub picture_url: Box<String>,
    pub idp_config_url: Box<String>,
    pub idp_login_url: Box<String>,
    pub login_state: Box<LoginState>,
    pub terms_of_service_url: Box<String>,
    pub privacy_policy_url: Box<String>,
}
