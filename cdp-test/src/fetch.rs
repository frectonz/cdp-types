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
/// Disables the fetch domain.
pub type FetchDisable = ();
/** Enables issuing of requestPaused events. A request will be paused until client
calls one of failRequest, fulfillRequest or continueRequest/continueWithAuth.*/
pub type FetchEnable = ();
/// Causes the request to fail with specified reason.
pub type FetchFailRequest = ();
/// Provides response to the request.
pub type FetchFulfillRequest = ();
/// Continues the request, optionally modifying some of its parameters.
pub type FetchContinueRequest = ();
/// Continues a request supplying authChallengeResponse following authRequired event.
pub type FetchContinueWithAuth = ();
/// ⚠️ Experimental
/** Continues loading of the paused response, optionally modifying the
response headers. If either responseCode or headers are modified, all of them
must be present.*/
pub type FetchContinueResponse = ();
/** Causes the body of the response to be received from the server and
returned as a single string. May only be issued for a request that
is paused in the Response stage and is mutually exclusive with
takeResponseBodyForInterceptionAsStream. Calling other methods that
affect the request or disabling fetch domain before body is received
results in an undefined behavior.
Note that the response body is not available for redirects. Requests
paused in the _redirect received_ state may be differentiated by
`responseCode` and presence of `location` response header, see
comments to `requestPaused` for details.*/
pub type FetchGetResponseBody = ();
/** Returns a handle to the stream representing the response body.
The request must be paused in the HeadersReceived stage.
Note that after this command the request can't be continued
as is -- client either needs to cancel it or to provide the
response body.
The stream only supports sequential read, IO.read will fail if the position
is specified.
This method is mutually exclusive with getResponseBody.
Calling other methods that affect the request or disabling fetch
domain before body is received results in an undefined behavior.*/
pub type FetchTakeResponseBodyAsStream = ();
