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
