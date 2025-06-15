use crate::common::*;
/** This is either obtained from another method or specified as `blob:<uuid>` where
`<uuid>` is an UUID of a Blob.*/
pub struct StreamHandle(String);
/// Close the stream, discard any temporary backing storage.
pub struct IoCloseParams {
    pub handle: Box<StreamHandle>,
}
/// Close the stream, discard any temporary backing storage.
pub type IoCloseReturns = ();
/// Read a chunk of the stream
pub struct IoReadParams {
    pub handle: Box<StreamHandle>,
    pub offset: i64,
    pub size: i64,
}
/// Read a chunk of the stream
pub struct IoReadParams {
    pub base64_encoded: bool,
    pub data: String,
    pub eof: bool,
}
/// Return UUID of Blob object specified by a remote object id.
pub struct IoResolveBlobParams {
    pub object_id: Box<()>,
}
/// Return UUID of Blob object specified by a remote object id.
pub struct IoResolveBlobParams {
    pub uuid: String,
}
