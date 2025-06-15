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
pub type FetchDisableParams = ();
/// Disables the fetch domain.
pub type FetchDisableReturns = ();
/** Enables issuing of requestPaused events. A request will be paused until client
calls one of failRequest, fulfillRequest or continueRequest/continueWithAuth.*/
pub struct FetchEnableParams {
    pub patterns: (),
    pub handle_auth_requests: (),
}
/** Enables issuing of requestPaused events. A request will be paused until client
calls one of failRequest, fulfillRequest or continueRequest/continueWithAuth.*/
pub type FetchEnableReturns = ();
/// Causes the request to fail with specified reason.
pub struct FetchFailRequestParams {
    pub request_id: (),
    pub error_reason: (),
}
/// Causes the request to fail with specified reason.
pub type FetchFailRequestReturns = ();
/// Provides response to the request.
pub struct FetchFulfillRequestParams {
    pub request_id: (),
    pub response_code: (),
    pub response_headers: (),
    pub binary_response_headers: (),
    pub body: (),
    pub response_phrase: (),
}
/// Provides response to the request.
pub type FetchFulfillRequestReturns = ();
/// Continues the request, optionally modifying some of its parameters.
pub struct FetchContinueRequestParams {
    pub request_id: (),
    pub url: (),
    pub method: (),
    pub post_data: (),
    pub headers: (),
    pub intercept_response: (),
}
/// Continues the request, optionally modifying some of its parameters.
pub type FetchContinueRequestReturns = ();
/// Continues a request supplying authChallengeResponse following authRequired event.
pub struct FetchContinueWithAuthParams {
    pub request_id: (),
    pub auth_challenge_response: (),
}
/// Continues a request supplying authChallengeResponse following authRequired event.
pub type FetchContinueWithAuthReturns = ();
/// ⚠️ Experimental
/** Continues loading of the paused response, optionally modifying the
response headers. If either responseCode or headers are modified, all of them
must be present.*/
pub struct FetchContinueResponseParams {
    pub request_id: (),
    pub response_code: (),
    pub response_phrase: (),
    pub response_headers: (),
    pub binary_response_headers: (),
}
/// ⚠️ Experimental
/** Continues loading of the paused response, optionally modifying the
response headers. If either responseCode or headers are modified, all of them
must be present.*/
pub type FetchContinueResponseReturns = ();
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
pub struct FetchGetResponseBodyParams {
    pub request_id: (),
}
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
pub type FetchGetResponseBodyReturns = ();
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
pub struct FetchTakeResponseBodyAsStreamParams {
    pub request_id: (),
}
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
pub type FetchTakeResponseBodyAsStreamReturns = ();
