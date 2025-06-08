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
/** Enable the WebAuthn domain and start intercepting credential storage and
retrieval with a virtual authenticator.*/
pub type WebAuthnEnable = ();
/// Disable the WebAuthn domain.
pub type WebAuthnDisable = ();
/// Creates and adds a virtual authenticator.
pub type WebAuthnAddVirtualAuthenticator = ();
/// Resets parameters isBogusSignature, isBadUV, isBadUP to false if they are not present.
pub type WebAuthnSetResponseOverrideBits = ();
/// Removes the given authenticator.
pub type WebAuthnRemoveVirtualAuthenticator = ();
/// Adds the credential to the specified authenticator.
pub type WebAuthnAddCredential = ();
/** Returns a single credential stored in the given virtual authenticator that
matches the credential ID.*/
pub type WebAuthnGetCredential = ();
/// Returns all the credentials stored in the given virtual authenticator.
pub type WebAuthnGetCredentials = ();
/// Removes a credential from the authenticator.
pub type WebAuthnRemoveCredential = ();
/// Clears all the credentials from the specified device.
pub type WebAuthnClearCredentials = ();
/** Sets whether User Verification succeeds or fails for an authenticator.
The default is true.*/
pub type WebAuthnSetUserVerified = ();
/** Sets whether tests of user presence will succeed immediately (if true) or fail to resolve (if false) for an authenticator.
The default is true.*/
pub type WebAuthnSetAutomaticPresenceSimulation = ();
/** Allows setting credential properties.
https://w3c.github.io/webauthn/#sctn-automation-set-credential-properties*/
pub type WebAuthnSetCredentialProperties = ();
