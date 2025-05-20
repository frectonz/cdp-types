use crate::common::*;
pub struct AuthenticatorId(String);
pub enum AuthenticatorProtocol {
    U2f,
    Ctap2,
}
pub enum Ctap2Version {
    Ctap20,
    Ctap21,
}
pub enum AuthenticatorTransport {
    Usb,
    Nfc,
    Ble,
    Cable,
    Internal,
}
pub struct VirtualAuthenticatorOptions {
    pub protocol: Box<AuthenticatorProtocol>,
    pub ctap2_version: Box<Ctap2Version>,
    pub transport: Box<AuthenticatorTransport>,
    pub has_resident_key: bool,
    pub has_user_verification: bool,
    pub has_large_blob: bool,
    pub has_cred_blob: bool,
    pub has_min_pin_length: bool,
    pub has_prf: bool,
    pub automatic_presence_simulation: bool,
    pub is_user_verified: bool,
    pub default_backup_eligibility: bool,
    pub default_backup_state: bool,
}
pub struct Credential {
    pub credential_id: String,
    pub is_resident_credential: bool,
    pub rp_id: String,
    pub private_key: String,
    pub user_handle: String,
    pub sign_count: i64,
    pub large_blob: String,
    pub backup_eligibility: bool,
    pub backup_state: bool,
    pub user_name: String,
    pub user_display_name: String,
}
pub type WebAuthnEnable = ();
pub type WebAuthnDisable = ();
pub type WebAuthnAddVirtualAuthenticator = ();
pub type WebAuthnSetResponseOverrideBits = ();
pub type WebAuthnRemoveVirtualAuthenticator = ();
pub type WebAuthnAddCredential = ();
pub type WebAuthnGetCredential = ();
pub type WebAuthnGetCredentials = ();
pub type WebAuthnRemoveCredential = ();
pub type WebAuthnClearCredentials = ();
pub type WebAuthnSetUserVerified = ();
pub type WebAuthnSetAutomaticPresenceSimulation = ();
pub type WebAuthnSetCredentialProperties = ();
