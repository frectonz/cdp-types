use crate::common::*;
/** This is either obtained from another method or specified as `blob:<uuid>` where
`<uuid>` is an UUID of a Blob.*/
pub struct StreamHandle(String);
/// Close the stream, discard any temporary backing storage.
pub type IoCloseParams = ();
/// Close the stream, discard any temporary backing storage.
pub type IoCloseResults = ();
/// Read a chunk of the stream
pub type IoReadParams = ();
/// Read a chunk of the stream
pub type IoReadResults = ();
/// Return UUID of Blob object specified by a remote object id.
pub type IoResolveBlobParams = ();
/// Return UUID of Blob object specified by a remote object id.
pub type IoResolveBlobResults = ();
