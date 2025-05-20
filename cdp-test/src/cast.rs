use crate::common::*;
pub struct Sink {
    pub name: String,
    pub id: String,
    pub session: String,
}
pub type CastEnable = ();
pub type CastDisable = ();
pub type CastSetSinkToUse = ();
pub type CastStartDesktopMirroring = ();
pub type CastStartTabMirroring = ();
pub type CastStopCasting = ();
