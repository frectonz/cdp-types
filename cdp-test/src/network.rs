use crate::common::*;
use crate::security::*;
use crate::io::*;
/// Resource type as it was perceived by the rendering engine.
pub enum ResourceType {
    Document,
    Stylesheet,
    Image,
    Media,
    Font,
    Script,
    TextTrack,
    Xhr,
    Fetch,
    Prefetch,
    EventSource,
    WebSocket,
    Manifest,
    SignedExchange,
    Ping,
    CspViolationReport,
    Preflight,
    Other,
}
/// Unique loader identifier.
pub struct LoaderId(String);
/// Unique intercepted request identifier.
pub struct InterceptionId(String);
/// Network level fetch failure reason.
pub enum ErrorReason {
    Failed,
    Aborted,
    TimedOut,
    AccessDenied,
    ConnectionClosed,
    ConnectionReset,
    ConnectionRefused,
    ConnectionAborted,
    ConnectionFailed,
    NameNotResolved,
    InternetDisconnected,
    AddressUnreachable,
    BlockedByClient,
    BlockedByResponse,
}
/// Monotonically increasing time in seconds since an arbitrary point in the past.
pub struct MonotonicTime(u64);
/// Request / response headers as keys / values of JSON object.
pub struct Headers(serde_json::Map<String, serde_json::Value>);
/// The underlying connection technology that the browser is supposedly using.
pub enum ConnectionType {
    None,
    Cellular2g,
    Cellular3g,
    Cellular4g,
    Bluetooth,
    Ethernet,
    Wifi,
    Wimax,
    Other,
}
/** Represents the cookie's 'SameSite' status:
https://tools.ietf.org/html/draft-west-first-party-cookies*/
pub enum CookieSameSite {
    Strict,
    Lax,
    None,
}
/// ⚠️ Experimental
/** Represents the cookie's 'Priority' status:
https://tools.ietf.org/html/draft-west-cookie-priority-00*/
pub enum CookiePriority {
    Low,
    Medium,
    High,
}
/// ⚠️ Experimental
/** Represents the source scheme of the origin that originally set the cookie.
A value of "Unset" allows protocol clients to emulate legacy cookie scope for the scheme.
This is a temporary ability and it will be removed in the future.*/
pub enum CookieSourceScheme {
    Unset,
    NonSecure,
    Secure,
}
/// Timing information for the request.
pub struct ResourceTiming {
    pub request_time: u64,
    pub proxy_start: u64,
    pub proxy_end: u64,
    pub dns_start: u64,
    pub dns_end: u64,
    pub connect_start: u64,
    pub connect_end: u64,
    pub ssl_start: u64,
    pub ssl_end: u64,
    pub worker_start: u64,
    pub worker_ready: u64,
    pub worker_fetch_start: u64,
    pub worker_respond_with_settled: u64,
    pub worker_router_evaluation_start: u64,
    pub worker_cache_lookup_start: u64,
    pub send_start: u64,
    pub send_end: u64,
    pub push_start: u64,
    pub push_end: u64,
    pub receive_headers_start: u64,
    pub receive_headers_end: u64,
}
/// Loading priority of a resource request.
pub enum ResourcePriority {
    VeryLow,
    Low,
    Medium,
    High,
    VeryHigh,
}
/// Post data entry for HTTP request
pub struct PostDataEntry {
    pub bytes: String,
}
/// HTTP request data.
pub struct Request {
    pub url: String,
    pub url_fragment: String,
    pub method: String,
    pub headers: Box<Headers>,
    pub post_data: String,
    pub has_post_data: bool,
    pub post_data_entries: Vec<PostDataEntry>,
    pub mixed_content_type: Box<MixedContentType>,
    pub initial_priority: Box<ResourcePriority>,
    pub referrer_policy: String,
    pub is_link_preload: bool,
    pub trust_token_params: Box<TrustTokenParams>,
    pub is_same_site: bool,
}
/// Details of a signed certificate timestamp (SCT).
pub struct SignedCertificateTimestamp {
    pub status: String,
    pub origin: String,
    pub log_description: String,
    pub log_id: String,
    pub timestamp: u64,
    pub hash_algorithm: String,
    pub signature_algorithm: String,
    pub signature_data: String,
}
/// Security details about a request.
pub struct SecurityDetails {
    pub protocol: String,
    pub key_exchange: String,
    pub key_exchange_group: String,
    pub cipher: String,
    pub mac: String,
    pub certificate_id: Box<CertificateId>,
    pub subject_name: String,
    pub san_list: Vec<String>,
    pub issuer: String,
    pub valid_from: Box<NetworkTimeSinceEpoch>,
    pub valid_to: Box<NetworkTimeSinceEpoch>,
    pub signed_certificate_timestamp_list: Vec<SignedCertificateTimestamp>,
    pub certificate_transparency_compliance: Box<CertificateTransparencyCompliance>,
    pub server_signature_algorithm: i64,
    pub encrypted_client_hello: bool,
}
/// Whether the request complied with Certificate Transparency policy.
pub enum CertificateTransparencyCompliance {
    Unknown,
    NotCompliant,
    Compliant,
}
/// The reason why request was blocked.
pub enum BlockedReason {
    Other,
    Csp,
    MixedContent,
    Origin,
    Inspector,
    SubresourceFilter,
    ContentType,
    CoepFrameResourceNeedsCoepHeader,
    CoopSandboxedIframeCannotNavigateToCoopPage,
    CorpNotSameOrigin,
    CorpNotSameOriginAfterDefaultedToSameOriginByCoep,
    CorpNotSameOriginAfterDefaultedToSameOriginByDip,
    CorpNotSameOriginAfterDefaultedToSameOriginByCoepAndDip,
    CorpNotSameSite,
    SriMessageSignatureMismatch,
}
/// The reason why request was blocked.
pub enum CorsError {
    DisallowedByMode,
    InvalidResponse,
    WildcardOriginNotAllowed,
    MissingAllowOriginHeader,
    MultipleAllowOriginValues,
    InvalidAllowOriginValue,
    AllowOriginMismatch,
    InvalidAllowCredentials,
    CorsDisabledScheme,
    PreflightInvalidStatus,
    PreflightDisallowedRedirect,
    PreflightWildcardOriginNotAllowed,
    PreflightMissingAllowOriginHeader,
    PreflightMultipleAllowOriginValues,
    PreflightInvalidAllowOriginValue,
    PreflightAllowOriginMismatch,
    PreflightInvalidAllowCredentials,
    PreflightMissingAllowExternal,
    PreflightInvalidAllowExternal,
    PreflightMissingAllowPrivateNetwork,
    PreflightInvalidAllowPrivateNetwork,
    InvalidAllowMethodsPreflightResponse,
    InvalidAllowHeadersPreflightResponse,
    MethodDisallowedByPreflightResponse,
    HeaderDisallowedByPreflightResponse,
    RedirectContainsCredentials,
    InsecurePrivateNetwork,
    InvalidPrivateNetworkAccess,
    UnexpectedPrivateNetworkAccess,
    NoCorsRedirectModeNotFollow,
    PreflightMissingPrivateNetworkAccessId,
    PreflightMissingPrivateNetworkAccessName,
    PrivateNetworkAccessPermissionUnavailable,
    PrivateNetworkAccessPermissionDenied,
    LocalNetworkAccessPermissionDenied,
}
pub struct CorsErrorStatus {
    pub cors_error: Box<CorsError>,
    pub failed_parameter: String,
}
/// Source of serviceworker response.
pub enum ServiceWorkerResponseSource {
    CacheStorage,
    HttpCache,
    FallbackCode,
    Network,
}
/// ⚠️ Experimental
/** Determines what type of Trust Token operation is executed and
depending on the type, some additional parameters. The values
are specified in third_party/blink/renderer/core/fetch/trust_token.idl.*/
pub struct TrustTokenParams {
    pub operation: Box<TrustTokenOperationType>,
    pub refresh_policy: String,
    pub issuers: Vec<String>,
}
/// ⚠️ Experimental
pub enum TrustTokenOperationType {
    Issuance,
    Redemption,
    Signing,
}
/// ⚠️ Experimental
/// The reason why Chrome uses a specific transport protocol for HTTP semantics.
pub enum AlternateProtocolUsage {
    AlternativeJobWonWithoutRace,
    AlternativeJobWonRace,
    MainJobWonRace,
    MappingMissing,
    Broken,
    DnsAlpnH3JobWonWithoutRace,
    DnsAlpnH3JobWonRace,
    UnspecifiedReason,
}
/// Source of service worker router.
pub enum ServiceWorkerRouterSource {
    Network,
    Cache,
    FetchEvent,
    RaceNetworkAndFetchHandler,
    RaceNetworkAndCache,
}
/// ⚠️ Experimental
pub struct ServiceWorkerRouterInfo {
    pub rule_id_matched: i64,
    pub matched_source_type: Box<ServiceWorkerRouterSource>,
    pub actual_source_type: Box<ServiceWorkerRouterSource>,
}
/// HTTP response data.
pub struct Response {
    pub url: String,
    pub status: i64,
    pub status_text: String,
    pub headers: Box<Headers>,
    pub headers_text: String,
    pub mime_type: String,
    pub charset: String,
    pub request_headers: Box<Headers>,
    pub request_headers_text: String,
    pub connection_reused: bool,
    pub connection_id: u64,
    pub remote_ip_address: String,
    pub remote_port: i64,
    pub from_disk_cache: bool,
    pub from_service_worker: bool,
    pub from_prefetch_cache: bool,
    pub from_early_hints: bool,
    pub service_worker_router_info: Box<ServiceWorkerRouterInfo>,
    pub encoded_data_length: u64,
    pub timing: Box<ResourceTiming>,
    pub service_worker_response_source: Box<ServiceWorkerResponseSource>,
    pub response_time: Box<NetworkTimeSinceEpoch>,
    pub cache_storage_cache_name: String,
    pub protocol: String,
    pub alternate_protocol_usage: Box<AlternateProtocolUsage>,
    pub security_state: Box<SecurityState>,
    pub security_details: Box<SecurityDetails>,
}
/// WebSocket request data.
pub struct WebSocketRequest {
    pub headers: Box<Headers>,
}
/// WebSocket response data.
pub struct WebSocketResponse {
    pub status: i64,
    pub status_text: String,
    pub headers: Box<Headers>,
    pub headers_text: String,
    pub request_headers: Box<Headers>,
    pub request_headers_text: String,
}
/// WebSocket message data. This represents an entire WebSocket message, not just a fragmented frame as the name suggests.
pub struct WebSocketFrame {
    pub opcode: u64,
    pub mask: bool,
    pub payload_data: String,
}
/// Information about the cached resource.
pub struct CachedResource {
    pub url: String,
    pub _type: Box<ResourceType>,
    pub response: Box<Response>,
    pub body_size: u64,
}
/// Information about the request initiator.
pub struct Initiator {
    pub _type: String,
    pub stack: Box<()>,
    pub url: String,
    pub line_number: u64,
    pub column_number: u64,
    pub request_id: Box<NetworkRequestId>,
}
/// ⚠️ Experimental
/** cookiePartitionKey object
The representation of the components of the key that are created by the cookiePartitionKey class contained in net/cookies/cookie_partition_key.h.*/
pub struct CookiePartitionKey {
    pub top_level_site: String,
    pub has_cross_site_ancestor: bool,
}
/// Cookie object
pub struct Cookie {
    pub name: String,
    pub value: String,
    pub domain: String,
    pub path: String,
    pub expires: u64,
    pub size: i64,
    pub http_only: bool,
    pub secure: bool,
    pub session: bool,
    pub same_site: Box<CookieSameSite>,
    pub priority: Box<CookiePriority>,
    pub same_party: bool,
    pub source_scheme: Box<CookieSourceScheme>,
    pub source_port: i64,
    pub partition_key: Box<CookiePartitionKey>,
    pub partition_key_opaque: bool,
}
/// ⚠️ Experimental
/// Types of reasons why a cookie may not be stored from a response.
pub enum SetCookieBlockedReason {
    SecureOnly,
    SameSiteStrict,
    SameSiteLax,
    SameSiteUnspecifiedTreatedAsLax,
    SameSiteNoneInsecure,
    UserPreferences,
    ThirdPartyPhaseout,
    ThirdPartyBlockedInFirstPartySet,
    SyntaxError,
    SchemeNotSupported,
    OverwriteSecure,
    InvalidDomain,
    InvalidPrefix,
    UnknownError,
    SchemefulSameSiteStrict,
    SchemefulSameSiteLax,
    SchemefulSameSiteUnspecifiedTreatedAsLax,
    SamePartyFromCrossPartyContext,
    SamePartyConflictsWithOtherAttributes,
    NameValuePairExceedsMaxSize,
    DisallowedCharacter,
    NoCookieContent,
}
/// ⚠️ Experimental
/// Types of reasons why a cookie may not be sent with a request.
pub enum CookieBlockedReason {
    SecureOnly,
    NotOnPath,
    DomainMismatch,
    SameSiteStrict,
    SameSiteLax,
    SameSiteUnspecifiedTreatedAsLax,
    SameSiteNoneInsecure,
    UserPreferences,
    ThirdPartyPhaseout,
    ThirdPartyBlockedInFirstPartySet,
    UnknownError,
    SchemefulSameSiteStrict,
    SchemefulSameSiteLax,
    SchemefulSameSiteUnspecifiedTreatedAsLax,
    SamePartyFromCrossPartyContext,
    NameValuePairExceedsMaxSize,
    PortMismatch,
    SchemeMismatch,
    AnonymousContext,
}
/// ⚠️ Experimental
/// Types of reasons why a cookie should have been blocked by 3PCD but is exempted for the request.
pub enum CookieExemptionReason {
    None,
    UserSetting,
    TpcdMetadata,
    TpcdDeprecationTrial,
    TopLevelTpcdDeprecationTrial,
    TpcdHeuristics,
    EnterprisePolicy,
    StorageAccess,
    TopLevelStorageAccess,
    Scheme,
    SameSiteNoneCookiesInSandbox,
}
/// ⚠️ Experimental
/// A cookie which was not stored from a response with the corresponding reason.
pub struct BlockedSetCookieWithReason {
    pub blocked_reasons: Vec<SetCookieBlockedReason>,
    pub cookie_line: String,
    pub cookie: Box<Cookie>,
}
/// ⚠️ Experimental
/** A cookie should have been blocked by 3PCD but is exempted and stored from a response with the
corresponding reason. A cookie could only have at most one exemption reason.*/
pub struct ExemptedSetCookieWithReason {
    pub exemption_reason: Box<CookieExemptionReason>,
    pub cookie_line: String,
    pub cookie: Box<Cookie>,
}
/// ⚠️ Experimental
/** A cookie associated with the request which may or may not be sent with it.
Includes the cookies itself and reasons for blocking or exemption.*/
pub struct AssociatedCookie {
    pub cookie: Box<Cookie>,
    pub blocked_reasons: Vec<CookieBlockedReason>,
    pub exemption_reason: Box<CookieExemptionReason>,
}
/// Cookie parameter object
pub struct CookieParam {
    pub name: String,
    pub value: String,
    pub url: String,
    pub domain: String,
    pub path: String,
    pub secure: bool,
    pub http_only: bool,
    pub same_site: Box<CookieSameSite>,
    pub expires: Box<NetworkTimeSinceEpoch>,
    pub priority: Box<CookiePriority>,
    pub same_party: bool,
    pub source_scheme: Box<CookieSourceScheme>,
    pub source_port: i64,
    pub partition_key: Box<CookiePartitionKey>,
}
/// ⚠️ Experimental
/** Stages of the interception to begin intercepting. Request will intercept before the request is
sent. Response will intercept after the response is received.*/
pub enum InterceptionStage {
    Request,
    HeadersReceived,
}
/// ⚠️ Experimental
/** Information about a signed exchange signature.
https://wicg.github.io/webpackage/draft-yasskin-httpbis-origin-signed-exchanges-impl.html#rfc.section.3.1*/
pub struct SignedExchangeSignature {
    pub label: String,
    pub signature: String,
    pub integrity: String,
    pub cert_url: String,
    pub cert_sha256: String,
    pub validity_url: String,
    pub date: i64,
    pub expires: i64,
    pub certificates: Vec<String>,
}
/// ⚠️ Experimental
/** Information about a signed exchange header.
https://wicg.github.io/webpackage/draft-yasskin-httpbis-origin-signed-exchanges-impl.html#cbor-representation*/
pub struct SignedExchangeHeader {
    pub request_url: String,
    pub response_code: i64,
    pub response_headers: Box<Headers>,
    pub signatures: Vec<SignedExchangeSignature>,
    pub header_integrity: String,
}
/// ⚠️ Experimental
/// Field type for a signed exchange related error.
pub enum SignedExchangeErrorField {
    SignatureSig,
    SignatureIntegrity,
    SignatureCertUrl,
    SignatureCertSha256,
    SignatureValidityUrl,
    SignatureTimestamps,
}
/// ⚠️ Experimental
/// Information about a signed exchange response.
pub struct SignedExchangeError {
    pub message: String,
    pub signature_index: i64,
    pub error_field: Box<SignedExchangeErrorField>,
}
/// ⚠️ Experimental
/// Information about a signed exchange response.
pub struct SignedExchangeInfo {
    pub outer_response: Box<Response>,
    pub header: Box<SignedExchangeHeader>,
    pub security_details: Box<SecurityDetails>,
    pub errors: Vec<SignedExchangeError>,
}
/// ⚠️ Experimental
/// List of content encodings supported by the backend.
pub enum ContentEncoding {
    Deflate,
    Gzip,
    Br,
    Zstd,
}
/// ⚠️ Experimental
pub enum DirectSocketDnsQueryType {
    Ipv4,
    Ipv6,
}
/// ⚠️ Experimental
pub struct DirectTcpSocketOptions {
    pub no_delay: bool,
    pub keep_alive_delay: u64,
    pub send_buffer_size: u64,
    pub receive_buffer_size: u64,
    pub dns_query_type: Box<DirectSocketDnsQueryType>,
}
/// ⚠️ Experimental
pub enum PrivateNetworkRequestPolicy {
    Allow,
    BlockFromInsecureToMorePrivate,
    WarnFromInsecureToMorePrivate,
    PreflightBlock,
    PreflightWarn,
    PermissionBlock,
    PermissionWarn,
}
/// ⚠️ Experimental
pub enum IpAddressSpace {
    Local,
    Private,
    Public,
    Unknown,
}
/// ⚠️ Experimental
pub struct ConnectTiming {
    pub request_time: u64,
}
/// ⚠️ Experimental
pub struct ClientSecurityState {
    pub initiator_is_secure_context: bool,
    pub initiator_ip_address_space: Box<IpAddressSpace>,
    pub private_network_request_policy: Box<PrivateNetworkRequestPolicy>,
}
/// ⚠️ Experimental
pub enum CrossOriginOpenerPolicyValue {
    SameOrigin,
    SameOriginAllowPopups,
    RestrictProperties,
    UnsafeNone,
    SameOriginPlusCoep,
    RestrictPropertiesPlusCoep,
    NoopenerAllowPopups,
}
/// ⚠️ Experimental
pub struct CrossOriginOpenerPolicyStatus {
    pub value: Box<CrossOriginOpenerPolicyValue>,
    pub report_only_value: Box<CrossOriginOpenerPolicyValue>,
    pub reporting_endpoint: String,
    pub report_only_reporting_endpoint: String,
}
/// ⚠️ Experimental
pub enum CrossOriginEmbedderPolicyValue {
    None,
    Credentialless,
    RequireCorp,
}
/// ⚠️ Experimental
pub struct CrossOriginEmbedderPolicyStatus {
    pub value: Box<CrossOriginEmbedderPolicyValue>,
    pub report_only_value: Box<CrossOriginEmbedderPolicyValue>,
    pub reporting_endpoint: String,
    pub report_only_reporting_endpoint: String,
}
/// ⚠️ Experimental
pub enum ContentSecurityPolicySource {
    Http,
    Meta,
}
/// ⚠️ Experimental
pub struct ContentSecurityPolicyStatus {
    pub effective_directives: String,
    pub is_enforced: bool,
    pub source: Box<ContentSecurityPolicySource>,
}
/// ⚠️ Experimental
pub struct SecurityIsolationStatus {
    pub coop: Box<CrossOriginOpenerPolicyStatus>,
    pub coep: Box<CrossOriginEmbedderPolicyStatus>,
    pub csp: Vec<ContentSecurityPolicyStatus>,
}
/// ⚠️ Experimental
/// The status of a Reporting API report.
pub enum ReportStatus {
    Queued,
    Pending,
    MarkedForRemoval,
    Success,
}
/// ⚠️ Experimental
pub struct ReportId(String);
/// ⚠️ Experimental
/// An object representing a report generated by the Reporting API.
pub struct ReportingApiReport {
    pub id: Box<ReportId>,
    pub initiator_url: String,
    pub destination: String,
    pub _type: String,
    pub timestamp: Box<NetworkTimeSinceEpoch>,
    pub depth: i64,
    pub completed_attempts: i64,
    pub body: serde_json::Map<String, serde_json::Value>,
    pub status: Box<ReportStatus>,
}
/// ⚠️ Experimental
pub struct ReportingApiEndpoint {
    pub url: String,
    pub group_name: String,
}
/// ⚠️ Experimental
/// An object providing the result of a network resource load.
pub struct LoadNetworkResourcePageResult {
    pub success: bool,
    pub net_error: u64,
    pub net_error_name: String,
    pub http_status_code: u64,
    pub stream: Box<StreamHandle>,
    pub headers: Box<Headers>,
}
/// ⚠️ Experimental
/** An options object that may be extended later to better support CORS,
CORB and streaming.*/
pub struct LoadNetworkResourceOptions {
    pub disable_cache: bool,
    pub include_credentials: bool,
}
/// ⚠️ Experimental
/// Sets a list of content encodings that will be accepted. Empty list means no encoding is accepted.
pub struct NetworkSetAcceptedEncodingsParams {
    pub encodings: (),
}
/// ⚠️ Experimental
/// Sets a list of content encodings that will be accepted. Empty list means no encoding is accepted.
pub type NetworkSetAcceptedEncodingsReturns = ();
/// ⚠️ Experimental
/// Clears accepted encodings set by setAcceptedEncodings
pub type NetworkClearAcceptedEncodingsOverrideParams = ();
/// ⚠️ Experimental
/// Clears accepted encodings set by setAcceptedEncodings
pub type NetworkClearAcceptedEncodingsOverrideReturns = ();
#[deprecated]
/// Tells whether clearing browser cache is supported.
pub type NetworkCanClearBrowserCacheParams = ();
#[deprecated]
/// Tells whether clearing browser cache is supported.
pub type NetworkCanClearBrowserCacheReturns = ();
#[deprecated]
/// Tells whether clearing browser cookies is supported.
pub type NetworkCanClearBrowserCookiesParams = ();
#[deprecated]
/// Tells whether clearing browser cookies is supported.
pub type NetworkCanClearBrowserCookiesReturns = ();
#[deprecated]
/// Tells whether emulation of network conditions is supported.
pub type NetworkCanEmulateNetworkConditionsParams = ();
#[deprecated]
/// Tells whether emulation of network conditions is supported.
pub type NetworkCanEmulateNetworkConditionsReturns = ();
/// Clears browser cache.
pub type NetworkClearBrowserCacheParams = ();
/// Clears browser cache.
pub type NetworkClearBrowserCacheReturns = ();
/// Clears browser cookies.
pub type NetworkClearBrowserCookiesParams = ();
/// Clears browser cookies.
pub type NetworkClearBrowserCookiesReturns = ();
#[deprecated]
/// ⚠️ Experimental
/** Response to Network.requestIntercepted which either modifies the request to continue with any
modifications, or blocks it, or completes it with the provided response bytes. If a network
fetch occurs as a result which encounters a redirect an additional Network.requestIntercepted
event will be sent with the same InterceptionId.
Deprecated, use Fetch.continueRequest, Fetch.fulfillRequest and Fetch.failRequest instead.*/
pub struct NetworkContinueInterceptedRequestParams {
    pub interception_id: (),
    pub error_reason: (),
    pub raw_response: (),
    pub url: (),
    pub method: (),
    pub post_data: (),
    pub headers: (),
    pub auth_challenge_response: (),
}
#[deprecated]
/// ⚠️ Experimental
/** Response to Network.requestIntercepted which either modifies the request to continue with any
modifications, or blocks it, or completes it with the provided response bytes. If a network
fetch occurs as a result which encounters a redirect an additional Network.requestIntercepted
event will be sent with the same InterceptionId.
Deprecated, use Fetch.continueRequest, Fetch.fulfillRequest and Fetch.failRequest instead.*/
pub type NetworkContinueInterceptedRequestReturns = ();
/// Deletes browser cookies with matching name and url or domain/path/partitionKey pair.
pub struct NetworkDeleteCookiesParams {
    pub name: (),
    pub url: (),
    pub domain: (),
    pub path: (),
    pub partition_key: (),
}
/// Deletes browser cookies with matching name and url or domain/path/partitionKey pair.
pub type NetworkDeleteCookiesReturns = ();
/// Disables network tracking, prevents network events from being sent to the client.
pub type NetworkDisableParams = ();
/// Disables network tracking, prevents network events from being sent to the client.
pub type NetworkDisableReturns = ();
/// Activates emulation of network conditions.
pub struct NetworkEmulateNetworkConditionsParams {
    pub offline: (),
    pub latency: (),
    pub download_throughput: (),
    pub upload_throughput: (),
    pub connection_type: (),
    pub packet_loss: (),
    pub packet_queue_length: (),
    pub packet_reordering: (),
}
/// Activates emulation of network conditions.
pub type NetworkEmulateNetworkConditionsReturns = ();
/// Enables network tracking, network events will now be delivered to the client.
pub struct NetworkEnableParams {
    pub max_total_buffer_size: (),
    pub max_resource_buffer_size: (),
    pub max_post_data_size: (),
}
/// Enables network tracking, network events will now be delivered to the client.
pub type NetworkEnableReturns = ();
#[deprecated]
/** Returns all browser cookies. Depending on the backend support, will return detailed cookie
information in the `cookies` field.
Deprecated. Use Storage.getCookies instead.*/
pub type NetworkGetAllCookiesParams = ();
#[deprecated]
/** Returns all browser cookies. Depending on the backend support, will return detailed cookie
information in the `cookies` field.
Deprecated. Use Storage.getCookies instead.*/
pub type NetworkGetAllCookiesReturns = ();
/// ⚠️ Experimental
/// Returns the DER-encoded certificate.
pub struct NetworkGetCertificateParams {
    pub origin: (),
}
/// ⚠️ Experimental
/// Returns the DER-encoded certificate.
pub type NetworkGetCertificateReturns = ();
/** Returns all browser cookies for the current URL. Depending on the backend support, will return
detailed cookie information in the `cookies` field.*/
pub struct NetworkGetCookiesParams {
    pub urls: (),
}
/** Returns all browser cookies for the current URL. Depending on the backend support, will return
detailed cookie information in the `cookies` field.*/
pub type NetworkGetCookiesReturns = ();
/// Returns content served for the given request.
pub struct NetworkGetResponseBodyParams {
    pub request_id: (),
}
/// Returns content served for the given request.
pub type NetworkGetResponseBodyReturns = ();
/// Returns post data sent with the request. Returns an error when no data was sent with the request.
pub struct NetworkGetRequestPostDataParams {
    pub request_id: (),
}
/// Returns post data sent with the request. Returns an error when no data was sent with the request.
pub type NetworkGetRequestPostDataReturns = ();
/// ⚠️ Experimental
/// Returns content served for the given currently intercepted request.
pub struct NetworkGetResponseBodyForInterceptionParams {
    pub interception_id: (),
}
/// ⚠️ Experimental
/// Returns content served for the given currently intercepted request.
pub type NetworkGetResponseBodyForInterceptionReturns = ();
/// ⚠️ Experimental
/** Returns a handle to the stream representing the response body. Note that after this command,
the intercepted request can't be continued as is -- you either need to cancel it or to provide
the response body. The stream only supports sequential read, IO.read will fail if the position
is specified.*/
pub struct NetworkTakeResponseBodyForInterceptionAsStreamParams {
    pub interception_id: (),
}
/// ⚠️ Experimental
/** Returns a handle to the stream representing the response body. Note that after this command,
the intercepted request can't be continued as is -- you either need to cancel it or to provide
the response body. The stream only supports sequential read, IO.read will fail if the position
is specified.*/
pub type NetworkTakeResponseBodyForInterceptionAsStreamReturns = ();
/// ⚠️ Experimental
/** This method sends a new XMLHttpRequest which is identical to the original one. The following
parameters should be identical: method, url, async, request body, extra headers, withCredentials
attribute, user, password.*/
pub struct NetworkReplayXhrParams {
    pub request_id: (),
}
/// ⚠️ Experimental
/** This method sends a new XMLHttpRequest which is identical to the original one. The following
parameters should be identical: method, url, async, request body, extra headers, withCredentials
attribute, user, password.*/
pub type NetworkReplayXhrReturns = ();
/// ⚠️ Experimental
/// Searches for given string in response content.
pub struct NetworkSearchInResponseBodyParams {
    pub request_id: (),
    pub query: (),
    pub case_sensitive: (),
    pub is_regex: (),
}
/// ⚠️ Experimental
/// Searches for given string in response content.
pub type NetworkSearchInResponseBodyReturns = ();
/// ⚠️ Experimental
/// Blocks URLs from loading.
pub struct NetworkSetBlockedUrLsParams {
    pub urls: (),
}
/// ⚠️ Experimental
/// Blocks URLs from loading.
pub type NetworkSetBlockedUrLsReturns = ();
/// Toggles ignoring of service worker for each request.
pub struct NetworkSetBypassServiceWorkerParams {
    pub bypass: (),
}
/// Toggles ignoring of service worker for each request.
pub type NetworkSetBypassServiceWorkerReturns = ();
/// Toggles ignoring cache for each request. If `true`, cache will not be used.
pub struct NetworkSetCacheDisabledParams {
    pub cache_disabled: (),
}
/// Toggles ignoring cache for each request. If `true`, cache will not be used.
pub type NetworkSetCacheDisabledReturns = ();
/// Sets a cookie with the given cookie data; may overwrite equivalent cookies if they exist.
pub struct NetworkSetCookieParams {
    pub name: (),
    pub value: (),
    pub url: (),
    pub domain: (),
    pub path: (),
    pub secure: (),
    pub http_only: (),
    pub same_site: (),
    pub expires: (),
    pub priority: (),
    pub same_party: (),
    pub source_scheme: (),
    pub source_port: (),
    pub partition_key: (),
}
/// Sets a cookie with the given cookie data; may overwrite equivalent cookies if they exist.
pub type NetworkSetCookieReturns = ();
/// Sets given cookies.
pub struct NetworkSetCookiesParams {
    pub cookies: (),
}
/// Sets given cookies.
pub type NetworkSetCookiesReturns = ();
/// Specifies whether to always send extra HTTP headers with the requests from this page.
pub struct NetworkSetExtraHttpHeadersParams {
    pub headers: (),
}
/// Specifies whether to always send extra HTTP headers with the requests from this page.
pub type NetworkSetExtraHttpHeadersReturns = ();
/// ⚠️ Experimental
/// Specifies whether to attach a page script stack id in requests
pub struct NetworkSetAttachDebugStackParams {
    pub enabled: (),
}
/// ⚠️ Experimental
/// Specifies whether to attach a page script stack id in requests
pub type NetworkSetAttachDebugStackReturns = ();
#[deprecated]
/// ⚠️ Experimental
/** Sets the requests to intercept that match the provided patterns and optionally resource types.
Deprecated, please use Fetch.enable instead.*/
pub struct NetworkSetRequestInterceptionParams {
    pub patterns: (),
}
#[deprecated]
/// ⚠️ Experimental
/** Sets the requests to intercept that match the provided patterns and optionally resource types.
Deprecated, please use Fetch.enable instead.*/
pub type NetworkSetRequestInterceptionReturns = ();
/// Allows overriding user agent with the given string.
pub struct NetworkSetUserAgentOverrideParams {
    pub user_agent: (),
    pub accept_language: (),
    pub platform: (),
    pub user_agent_metadata: (),
}
/// Allows overriding user agent with the given string.
pub type NetworkSetUserAgentOverrideReturns = crate::emulation::EmulationSetUserAgentOverrideReturns;
/// ⚠️ Experimental
/** Enables streaming of the response for the given requestId.
If enabled, the dataReceived event contains the data that was received during streaming.*/
pub struct NetworkStreamResourceContentParams {
    pub request_id: (),
}
/// ⚠️ Experimental
/** Enables streaming of the response for the given requestId.
If enabled, the dataReceived event contains the data that was received during streaming.*/
pub type NetworkStreamResourceContentReturns = ();
/// ⚠️ Experimental
/// Returns information about the COEP/COOP isolation status.
pub struct NetworkGetSecurityIsolationStatusParams {
    pub frame_id: (),
}
/// ⚠️ Experimental
/// Returns information about the COEP/COOP isolation status.
pub type NetworkGetSecurityIsolationStatusReturns = ();
/// ⚠️ Experimental
/** Enables tracking for the Reporting API, events generated by the Reporting API will now be delivered to the client.
Enabling triggers 'reportingApiReportAdded' for all existing reports.*/
pub struct NetworkEnableReportingApiParams {
    pub enable: (),
}
/// ⚠️ Experimental
/** Enables tracking for the Reporting API, events generated by the Reporting API will now be delivered to the client.
Enabling triggers 'reportingApiReportAdded' for all existing reports.*/
pub type NetworkEnableReportingApiReturns = ();
/// ⚠️ Experimental
/// Fetches the resource and returns the content.
pub struct NetworkLoadNetworkResourceParams {
    pub frame_id: (),
    pub url: (),
    pub options: (),
}
/// ⚠️ Experimental
/// Fetches the resource and returns the content.
pub type NetworkLoadNetworkResourceReturns = ();
/// ⚠️ Experimental
/** Sets Controls for third-party cookie access
Page reload is required before the new cookie bahavior will be observed*/
pub struct NetworkSetCookieControlsParams {
    pub enable_third_party_cookie_restriction: (),
    pub disable_third_party_cookie_metadata: (),
    pub disable_third_party_cookie_heuristics: (),
}
/// ⚠️ Experimental
/** Sets Controls for third-party cookie access
Page reload is required before the new cookie bahavior will be observed*/
pub type NetworkSetCookieControlsReturns = ();
