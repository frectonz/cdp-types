use crate::common::*;
use crate::network::*;
use crate::io::*;
use crate::page::*;
/** Stages of the request to handle. Request will intercept before the request is
sent. Response will intercept after the response is received (but before response
body is received).*/
pub enum RequestStage {
    Request,
    Response,
}
/// Response HTTP header entry
pub struct HeaderEntry {
    pub name: String,
    pub value: String,
}
