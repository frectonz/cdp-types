use crate::common::*;
use crate::network::*;
/// An internal certificate ID value.
pub struct CertificateId(i64);
/** A description of mixed content (HTTP resources on HTTPS pages), as defined by
https://www.w3.org/TR/mixed-content/#categories*/
pub enum MixedContentType {
    Blockable,
    OptionallyBlockable,
    None,
}
/// The security level of a page or resource.
pub enum SecurityState {
    Unknown,
    Neutral,
    Insecure,
    Secure,
    Info,
    InsecureBroken,
}
/// ⚠️ Experimental
/// Details about the security state of the page certificate.
pub struct CertificateSecurityState {
    pub protocol: String,
    pub key_exchange: String,
    pub key_exchange_group: String,
    pub cipher: String,
    pub mac: String,
    pub certificate: Vec<String>,
    pub subject_name: String,
    pub issuer: String,
    pub valid_from: Box<NetworkTimeSinceEpoch>,
    pub valid_to: Box<NetworkTimeSinceEpoch>,
    pub certificate_network_error: String,
    pub certificate_has_weak_signature: bool,
    pub certificate_has_sha1_signature: bool,
    pub modern_ssl: bool,
    pub obsolete_ssl_protocol: bool,
    pub obsolete_ssl_key_exchange: bool,
    pub obsolete_ssl_cipher: bool,
    pub obsolete_ssl_signature: bool,
}
/// ⚠️ Experimental
pub enum SafetyTipStatus {
    BadReputation,
    Lookalike,
}
/// ⚠️ Experimental
pub struct SafetyTipInfo {
    pub safety_tip_status: Box<SafetyTipStatus>,
    pub safe_url: String,
}
/// ⚠️ Experimental
/// Security state information about the page.
pub struct VisibleSecurityState {
    pub security_state: Box<SecurityState>,
    pub certificate_security_state: Box<CertificateSecurityState>,
    pub safety_tip_info: Box<SafetyTipInfo>,
    pub security_state_issue_ids: Vec<String>,
}
/// An explanation of an factor contributing to the security state.
pub struct SecurityStateExplanation {
    pub security_state: Box<SecurityState>,
    pub title: String,
    pub summary: String,
    pub description: String,
    pub mixed_content_type: Box<MixedContentType>,
    pub certificate: Vec<String>,
    pub recommendations: Vec<String>,
}
#[deprecated]
/// Information about insecure content on the page.
pub struct InsecureContentStatus {
    pub ran_mixed_content: bool,
    pub displayed_mixed_content: bool,
    pub contained_mixed_form: bool,
    pub ran_content_with_cert_errors: bool,
    pub displayed_content_with_cert_errors: bool,
    pub ran_insecure_content_style: Box<SecurityState>,
    pub displayed_insecure_content_style: Box<SecurityState>,
}
/** The action to take when a certificate error occurs. continue will continue processing the
request and cancel will cancel the request.*/
pub enum CertificateErrorAction {
    Continue,
    Cancel,
}
/// Disables tracking security state changes.
pub type SecurityDisableParams = ();
/// Disables tracking security state changes.
pub type SecurityDisableReturns = ();
/// Enables tracking security state changes.
pub type SecurityEnableParams = ();
/// Enables tracking security state changes.
pub type SecurityEnableReturns = ();
/// Enable/disable whether all certificate errors should be ignored.
pub struct SecuritySetIgnoreCertificateErrorsParams {
    pub ignore: bool,
}
/// Enable/disable whether all certificate errors should be ignored.
pub type SecuritySetIgnoreCertificateErrorsReturns = ();
#[deprecated]
/// Handles a certificate error that fired a certificateError event.
pub struct SecurityHandleCertificateErrorParams {
    pub event_id: i64,
    pub action: Box<CertificateErrorAction>,
}
#[deprecated]
/// Handles a certificate error that fired a certificateError event.
pub type SecurityHandleCertificateErrorReturns = ();
#[deprecated]
/** Enable/disable overriding certificate errors. If enabled, all certificate error events need to
be handled by the DevTools client and should be answered with `handleCertificateError` commands.*/
pub struct SecuritySetOverrideCertificateErrorsParams {
    pub _override: bool,
}
#[deprecated]
/** Enable/disable overriding certificate errors. If enabled, all certificate error events need to
be handled by the DevTools client and should be answered with `handleCertificateError` commands.*/
pub type SecuritySetOverrideCertificateErrorsReturns = ();
#[deprecated]
/** There is a certificate error. If overriding certificate errors is enabled, then it should be
handled with the `handleCertificateError` command. Note: this event does not fire if the
certificate error has been allowed internally. Only one client per target should override
certificate errors at the same time.*/
pub struct SecurityCertificateErrorEvent {
    pub event_id: i64,
    pub error_type: String,
    pub request_url: String,
}
/// ⚠️ Experimental
/// The security state of the page changed.
pub struct SecurityVisibleSecurityStateChangedEvent {
    pub visible_security_state: Box<VisibleSecurityState>,
}
#[deprecated]
/// The security state of the page changed. No longer being sent.
pub struct SecuritySecurityStateChangedEvent {
    pub security_state: Box<SecurityState>,
    pub scheme_is_cryptographic: bool,
    pub explanations: Vec<SecurityStateExplanation>,
    pub insecure_content_status: Box<InsecureContentStatus>,
    pub summary: String,
}
