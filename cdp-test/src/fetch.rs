use crate::network::*;
use crate::io::*;
use crate::page::*;
/** Unique request identifier.
Note that this does not identify individual HTTP requests that are part of
a network request.*/
/// <https://chromedevtools.github.io/devtools-protocol/tot/Fetch/#type-RequestId>
pub struct FetchRequestId(String);
/** Stages of the request to handle. Request will intercept before the request is
sent. Response will intercept after the response is received (but before response
body is received).*/
/// <https://chromedevtools.github.io/devtools-protocol/tot/Fetch/#type-RequestStage>
pub enum FetchRequestStage {
    Request,
    Response,
}
/// <https://chromedevtools.github.io/devtools-protocol/tot/Fetch/#type-RequestPattern>
pub struct FetchRequestPattern {
    pub url_pattern: String,
    pub resource_type: (),
    pub request_stage: (),
}
/// Response HTTP header entry
/// <https://chromedevtools.github.io/devtools-protocol/tot/Fetch/#type-HeaderEntry>
pub struct FetchHeaderEntry {
    pub name: String,
    pub value: String,
}
/// Authorization challenge for HTTP status code 401 or 407.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Fetch/#type-AuthChallenge>
pub struct FetchAuthChallenge {
    pub source: String,
    pub origin: String,
    pub scheme: String,
    pub realm: String,
}
/// Response to an AuthChallenge.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Fetch/#type-AuthChallengeResponse>
pub struct FetchAuthChallengeResponse {
    pub response: String,
    pub username: String,
    pub password: String,
}
