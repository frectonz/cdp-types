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
pub struct WebAuthnEnableParams {
    pub enable_ui: (),
}
/** Enable the WebAuthn domain and start intercepting credential storage and
retrieval with a virtual authenticator.*/
pub type WebAuthnEnableReturns = ();
/// Disable the WebAuthn domain.
pub type WebAuthnDisableParams = ();
/// Disable the WebAuthn domain.
pub type WebAuthnDisableReturns = ();
/// Creates and adds a virtual authenticator.
pub struct WebAuthnAddVirtualAuthenticatorParams {
    pub options: (),
}
/// Creates and adds a virtual authenticator.
pub type WebAuthnAddVirtualAuthenticatorReturns = ();
/// Resets parameters isBogusSignature, isBadUV, isBadUP to false if they are not present.
pub struct WebAuthnSetResponseOverrideBitsParams {
    pub authenticator_id: (),
    pub is_bogus_signature: (),
    pub is_bad_uv: (),
    pub is_bad_up: (),
}
/// Resets parameters isBogusSignature, isBadUV, isBadUP to false if they are not present.
pub type WebAuthnSetResponseOverrideBitsReturns = ();
/// Removes the given authenticator.
pub struct WebAuthnRemoveVirtualAuthenticatorParams {
    pub authenticator_id: (),
}
/// Removes the given authenticator.
pub type WebAuthnRemoveVirtualAuthenticatorReturns = ();
/// Adds the credential to the specified authenticator.
pub struct WebAuthnAddCredentialParams {
    pub authenticator_id: (),
    pub credential: (),
}
/// Adds the credential to the specified authenticator.
pub type WebAuthnAddCredentialReturns = ();
/** Returns a single credential stored in the given virtual authenticator that
matches the credential ID.*/
pub struct WebAuthnGetCredentialParams {
    pub authenticator_id: (),
    pub credential_id: (),
}
/** Returns a single credential stored in the given virtual authenticator that
matches the credential ID.*/
pub type WebAuthnGetCredentialReturns = ();
/// Returns all the credentials stored in the given virtual authenticator.
pub struct WebAuthnGetCredentialsParams {
    pub authenticator_id: (),
}
/// Returns all the credentials stored in the given virtual authenticator.
pub type WebAuthnGetCredentialsReturns = ();
/// Removes a credential from the authenticator.
pub struct WebAuthnRemoveCredentialParams {
    pub authenticator_id: (),
    pub credential_id: (),
}
/// Removes a credential from the authenticator.
pub type WebAuthnRemoveCredentialReturns = ();
/// Clears all the credentials from the specified device.
pub struct WebAuthnClearCredentialsParams {
    pub authenticator_id: (),
}
/// Clears all the credentials from the specified device.
pub type WebAuthnClearCredentialsReturns = ();
/** Sets whether User Verification succeeds or fails for an authenticator.
The default is true.*/
pub struct WebAuthnSetUserVerifiedParams {
    pub authenticator_id: (),
    pub is_user_verified: (),
}
/** Sets whether User Verification succeeds or fails for an authenticator.
The default is true.*/
pub type WebAuthnSetUserVerifiedReturns = ();
/** Sets whether tests of user presence will succeed immediately (if true) or fail to resolve (if false) for an authenticator.
The default is true.*/
pub struct WebAuthnSetAutomaticPresenceSimulationParams {
    pub authenticator_id: (),
    pub enabled: (),
}
/** Sets whether tests of user presence will succeed immediately (if true) or fail to resolve (if false) for an authenticator.
The default is true.*/
pub type WebAuthnSetAutomaticPresenceSimulationReturns = ();
/** Allows setting credential properties.
https://w3c.github.io/webauthn/#sctn-automation-set-credential-properties*/
pub struct WebAuthnSetCredentialPropertiesParams {
    pub authenticator_id: (),
    pub credential_id: (),
    pub backup_eligibility: (),
    pub backup_state: (),
}
/** Allows setting credential properties.
https://w3c.github.io/webauthn/#sctn-automation-set-credential-properties*/
pub type WebAuthnSetCredentialPropertiesReturns = ();
