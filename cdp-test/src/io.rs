/** This is either obtained from another method or specified as `blob:<uuid>` where
`<uuid>` is an UUID of a Blob.*/
/// <https://chromedevtools.github.io/devtools-protocol/tot/IO/#type-StreamHandle>
pub struct IoStreamHandle(String);
