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
    pub patterns: Vec<()>,
    pub handle_auth_requests: bool,
}
/** Enables issuing of requestPaused events. A request will be paused until client
calls one of failRequest, fulfillRequest or continueRequest/continueWithAuth.*/
pub type FetchEnableReturns = ();
/// Causes the request to fail with specified reason.
pub struct FetchFailRequestParams {
    pub request_id: Box<NetworkRequestId>,
    pub error_reason: Box<ErrorReason>,
}
/// Causes the request to fail with specified reason.
pub type FetchFailRequestReturns = ();
/// Provides response to the request.
pub struct FetchFulfillRequestParams {
    pub request_id: Box<NetworkRequestId>,
    pub response_code: i64,
    pub response_headers: Vec<HeaderEntry>,
    pub binary_response_headers: String,
    pub body: String,
    pub response_phrase: String,
}
/// Provides response to the request.
pub type FetchFulfillRequestReturns = ();
/// Continues the request, optionally modifying some of its parameters.
pub struct FetchContinueRequestParams {
    pub request_id: Box<NetworkRequestId>,
    pub url: String,
    pub method: String,
    pub post_data: String,
    pub headers: Vec<HeaderEntry>,
    pub intercept_response: bool,
}
/// Continues the request, optionally modifying some of its parameters.
pub type FetchContinueRequestReturns = ();
/// Continues a request supplying authChallengeResponse following authRequired event.
pub struct FetchContinueWithAuthParams {
    pub request_id: Box<NetworkRequestId>,
    pub auth_challenge_response: Box<()>,
}
/// Continues a request supplying authChallengeResponse following authRequired event.
pub type FetchContinueWithAuthReturns = ();
/// ⚠️ Experimental
/** Continues loading of the paused response, optionally modifying the
response headers. If either responseCode or headers are modified, all of them
must be present.*/
pub struct FetchContinueResponseParams {
    pub request_id: Box<NetworkRequestId>,
    pub response_code: i64,
    pub response_phrase: String,
    pub response_headers: Vec<HeaderEntry>,
    pub binary_response_headers: String,
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
    pub request_id: Box<NetworkRequestId>,
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
pub struct FetchGetResponseBodyParams {
    pub body: String,
    pub base64_encoded: bool,
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
pub struct FetchTakeResponseBodyAsStreamParams {
    pub request_id: Box<NetworkRequestId>,
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
pub struct FetchTakeResponseBodyAsStreamParams {
    pub stream: Box<StreamHandle>,
}
/** Issued when the domain is enabled and the request URL matches the
specified filter. The request is paused until the client responds
with one of continueRequest, failRequest or fulfillRequest.
The stage of the request can be determined by presence of responseErrorReason
and responseStatusCode -- the request is at the response stage if either
of these fields is present and in the request stage otherwise.
Redirect responses and subsequent requests are reported similarly to regular
responses and requests. Redirect responses may be distinguished by the value
of `responseStatusCode` (which is one of 301, 302, 303, 307, 308) along with
presence of the `location` header. Requests resulting from a redirect will
have `redirectedRequestId` field set.*/
pub struct FetchRequestPausedEvent {
    pub request_id: Box<NetworkRequestId>,
    pub request: Box<Request>,
    pub frame_id: Box<crate::page::FrameId>,
    pub resource_type: Box<ResourceType>,
    pub response_error_reason: Box<ErrorReason>,
    pub response_status_code: i64,
    pub response_status_text: String,
    pub response_headers: Vec<HeaderEntry>,
    pub network_id: Box<NetworkRequestId>,
    pub redirected_request_id: Box<NetworkRequestId>,
}
/** Issued when the domain is enabled with handleAuthRequests set to true.
The request is paused until client responds with continueWithAuth.*/
pub struct FetchAuthRequiredEvent {
    pub request_id: Box<NetworkRequestId>,
    pub request: Box<Request>,
    pub frame_id: Box<crate::page::FrameId>,
    pub resource_type: Box<ResourceType>,
    pub auth_challenge: Box<AuthChallenge>,
}
