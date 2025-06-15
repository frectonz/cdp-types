use crate::common::*;
/// Request browser port binding.
pub struct TetheringBindParams {
    pub port: i64,
}
/// Request browser port binding.
pub type TetheringBindReturns = ();
/// Request browser port unbinding.
pub struct TetheringUnbindParams {
    pub port: i64,
}
/// Request browser port unbinding.
pub type TetheringUnbindReturns = ();
/// Informs that port was successfully bound and got a specified connection id.
pub struct TetheringAcceptedEvent {
    pub port: i64,
    pub connection_id: String,
}
