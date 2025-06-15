use crate::common::*;
/** This is either obtained from another method or specified as `blob:<uuid>` where
`<uuid>` is an UUID of a Blob.*/
pub struct StreamHandle(String);
/// Close the stream, discard any temporary backing storage.
pub struct IoCloseParams {
    pub handle: (),
}
/// Close the stream, discard any temporary backing storage.
pub type IoCloseReturns = ();
/// Read a chunk of the stream
pub struct IoReadParams {
    pub handle: (),
    pub offset: (),
    pub size: (),
}
/// Read a chunk of the stream
pub type IoReadReturns = ();
/// Return UUID of Blob object specified by a remote object id.
pub struct IoResolveBlobParams {
    pub object_id: (),
}
/// Return UUID of Blob object specified by a remote object id.
pub type IoResolveBlobReturns = ();
