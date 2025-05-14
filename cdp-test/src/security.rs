pub use crate::common::*;
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
    pub certificate: (),
    pub subject_name: String,
    pub issuer: String,
    pub valid_from: (),
    pub valid_to: (),
    pub certificate_network_error: String,
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
    pub safety_tip_status: (),
    pub safe_url: String,
}
/// ⚠️ Experimental
/// Security state information about the page.
pub struct VisibleSecurityState {
    pub security_state: (),
    pub certificate_security_state: (),
    pub safety_tip_info: (),
    pub security_state_issue_ids: (),
}
/// An explanation of an factor contributing to the security state.
pub struct SecurityStateExplanation {
    pub security_state: (),
    pub title: String,
    pub summary: String,
    pub description: String,
    pub mixed_content_type: (),
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
    pub ran_insecure_content_style: (),
    pub displayed_insecure_content_style: (),
}
/** The action to take when a certificate error occurs. continue will continue processing the
request and cancel will cancel the request.*/
pub enum CertificateErrorAction {
    Continue,
    Cancel,
}
