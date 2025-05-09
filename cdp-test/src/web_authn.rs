/// <https://chromedevtools.github.io/devtools-protocol/tot/WebAuthn/#type-AuthenticatorId>
pub struct WebAuthnAuthenticatorId(String);
/// <https://chromedevtools.github.io/devtools-protocol/tot/WebAuthn/#type-AuthenticatorProtocol>
pub enum WebAuthnAuthenticatorProtocol {
    U2f,
    Ctap2,
}
/// <https://chromedevtools.github.io/devtools-protocol/tot/WebAuthn/#type-Ctap2Version>
pub enum WebAuthnCtap2Version {
    Ctap20,
    Ctap21,
}
/// <https://chromedevtools.github.io/devtools-protocol/tot/WebAuthn/#type-AuthenticatorTransport>
pub enum WebAuthnAuthenticatorTransport {
    Usb,
    Nfc,
    Ble,
    Cable,
    Internal,
}
/// <https://chromedevtools.github.io/devtools-protocol/tot/WebAuthn/#type-VirtualAuthenticatorOptions>
pub struct WebAuthnVirtualAuthenticatorOptions {
    pub protocol: (),
    pub ctap2_version: (),
    pub transport: (),
    pub has_resident_key: (),
    pub has_user_verification: (),
    pub has_large_blob: (),
    pub has_cred_blob: (),
    pub has_min_pin_length: (),
    pub has_prf: (),
    pub automatic_presence_simulation: (),
    pub is_user_verified: (),
    pub default_backup_eligibility: (),
    pub default_backup_state: (),
}
/// <https://chromedevtools.github.io/devtools-protocol/tot/WebAuthn/#type-Credential>
pub struct WebAuthnCredential {
    pub credential_id: String,
    pub is_resident_credential: (),
    pub rp_id: String,
    pub private_key: String,
    pub user_handle: String,
    pub sign_count: i64,
    pub large_blob: String,
    pub backup_eligibility: (),
    pub backup_state: (),
    pub user_name: String,
    pub user_display_name: String,
}
