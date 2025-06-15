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
    pub account_id: String,
    pub email: String,
    pub name: String,
    pub given_name: String,
    pub picture_url: String,
    pub idp_config_url: String,
    pub idp_login_url: String,
    pub login_state: Box<LoginState>,
    pub terms_of_service_url: String,
    pub privacy_policy_url: String,
}
pub struct FedCmEnableParams {
    test: (),
}
pub type FedCmEnableReturns = ();
pub type FedCmDisableParams = ();
pub type FedCmDisableReturns = ();
pub struct FedCmSelectAccountParams {
    test: (),
    test: (),
}
pub type FedCmSelectAccountReturns = ();
pub struct FedCmClickDialogButtonParams {
    test: (),
    test: (),
}
pub type FedCmClickDialogButtonReturns = ();
pub struct FedCmOpenUrlParams {
    test: (),
    test: (),
    test: (),
}
pub type FedCmOpenUrlReturns = ();
pub struct FedCmDismissDialogParams {
    test: (),
    test: (),
}
pub type FedCmDismissDialogReturns = ();
/** Resets the cooldown time, if any, to allow the next FedCM call to show
a dialog even if one was recently dismissed by the user.*/
pub type FedCmResetCooldownParams = ();
/** Resets the cooldown time, if any, to allow the next FedCM call to show
a dialog even if one was recently dismissed by the user.*/
pub type FedCmResetCooldownReturns = ();
