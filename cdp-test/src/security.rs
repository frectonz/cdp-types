/// An internal certificate ID value.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Security/#type-CertificateId>
pub struct SecurityCertificateId(i64);
/** A description of mixed content (HTTP resources on HTTPS pages), as defined by
https://www.w3.org/TR/mixed-content/#categories*/
/// <https://chromedevtools.github.io/devtools-protocol/tot/Security/#type-MixedContentType>
pub enum SecurityMixedContentType {
    Blockable,
    OptionallyBlockable,
    None,
}
/// The security level of a page or resource.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Security/#type-SecurityState>
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
/// <https://chromedevtools.github.io/devtools-protocol/tot/Security/#type-CertificateSecurityState>
pub struct SecurityCertificateSecurityState {
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
/// <https://chromedevtools.github.io/devtools-protocol/tot/Security/#type-SafetyTipStatus>
pub enum SecuritySafetyTipStatus {
    BadReputation,
    Lookalike,
}
/// ⚠️ Experimental
/// <https://chromedevtools.github.io/devtools-protocol/tot/Security/#type-SafetyTipInfo>
pub struct SecuritySafetyTipInfo {
    pub safety_tip_status: (),
    pub safe_url: String,
}
/// ⚠️ Experimental
/// Security state information about the page.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Security/#type-VisibleSecurityState>
pub struct SecurityVisibleSecurityState {
    pub security_state: (),
    pub certificate_security_state: (),
    pub safety_tip_info: (),
    pub security_state_issue_ids: (),
}
/// An explanation of an factor contributing to the security state.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Security/#type-SecurityStateExplanation>
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
/// <https://chromedevtools.github.io/devtools-protocol/tot/Security/#type-InsecureContentStatus>
pub struct SecurityInsecureContentStatus {
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
/// <https://chromedevtools.github.io/devtools-protocol/tot/Security/#type-CertificateErrorAction>
pub enum SecurityCertificateErrorAction {
    Continue,
    Cancel,
}
