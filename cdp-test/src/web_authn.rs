pub use crate::common::*;
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
pub struct Credential {
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
