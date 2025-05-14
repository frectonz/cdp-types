use crate::common::*;
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
    pub protocol: Box<String>,
    pub key_exchange: Box<String>,
    pub key_exchange_group: Box<String>,
    pub cipher: Box<String>,
    pub mac: Box<String>,
    pub certificate: (),
    pub subject_name: Box<String>,
    pub issuer: Box<String>,
    pub valid_from: Box<NetworkTimeSinceEpoch>,
    pub valid_to: Box<NetworkTimeSinceEpoch>,
    pub certificate_network_error: Box<String>,
    pub certificate_has_weak_signature: (),
    pub certificate_has_sha1_signature: (),
    pub modern_ssl: (),
    pub obsolete_ssl_protocol: (),
    pub obsolete_ssl_key_exchange: (),
    pub obsolete_ssl_cipher: (),
    pub obsolete_ssl_signature: (),
}
/// ⚠️ Experimental
pub enum SafetyTipStatus {
    BadReputation,
    Lookalike,
}
/// ⚠️ Experimental
pub struct SafetyTipInfo {
    pub safety_tip_status: Box<SafetyTipStatus>,
    pub safe_url: Box<String>,
}
/// ⚠️ Experimental
/// Security state information about the page.
pub struct VisibleSecurityState {
    pub security_state: Box<SecurityState>,
    pub certificate_security_state: Box<CertificateSecurityState>,
    pub safety_tip_info: Box<SafetyTipInfo>,
    pub security_state_issue_ids: (),
}
/// An explanation of an factor contributing to the security state.
pub struct SecurityStateExplanation {
    pub security_state: Box<SecurityState>,
    pub title: Box<String>,
    pub summary: Box<String>,
    pub description: Box<String>,
    pub mixed_content_type: Box<MixedContentType>,
    pub certificate: (),
    pub recommendations: (),
}
#[deprecated]
/// Information about insecure content on the page.
pub struct InsecureContentStatus {
    pub ran_mixed_content: (),
    pub displayed_mixed_content: (),
    pub contained_mixed_form: (),
    pub ran_content_with_cert_errors: (),
    pub displayed_content_with_cert_errors: (),
    pub ran_insecure_content_style: Box<SecurityState>,
    pub displayed_insecure_content_style: Box<SecurityState>,
}
/** The action to take when a certificate error occurs. continue will continue processing the
request and cancel will cancel the request.*/
pub enum CertificateErrorAction {
    Continue,
    Cancel,
}
