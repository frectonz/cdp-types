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
    pub encodings: Vec<ContentEncoding>,
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
pub struct NetworkCanClearBrowserCacheParams {
    pub result: bool,
}
#[deprecated]
/// Tells whether clearing browser cookies is supported.
pub type NetworkCanClearBrowserCookiesParams = ();
#[deprecated]
/// Tells whether clearing browser cookies is supported.
pub struct NetworkCanClearBrowserCookiesParams {
    pub result: bool,
}
#[deprecated]
/// Tells whether emulation of network conditions is supported.
pub type NetworkCanEmulateNetworkConditionsParams = ();
#[deprecated]
/// Tells whether emulation of network conditions is supported.
pub struct NetworkCanEmulateNetworkConditionsParams {
    pub result: bool,
}
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
    pub interception_id: Box<InterceptionId>,
    pub error_reason: Box<ErrorReason>,
    pub raw_response: String,
    pub url: String,
    pub method: String,
    pub post_data: String,
    pub headers: Box<Headers>,
    pub auth_challenge_response: Box<()>,
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
    pub name: String,
    pub url: String,
    pub domain: String,
    pub path: String,
    pub partition_key: Box<CookiePartitionKey>,
}
/// Deletes browser cookies with matching name and url or domain/path/partitionKey pair.
pub type NetworkDeleteCookiesReturns = ();
/// Disables network tracking, prevents network events from being sent to the client.
pub type NetworkDisableParams = ();
/// Disables network tracking, prevents network events from being sent to the client.
pub type NetworkDisableReturns = ();
/// Activates emulation of network conditions.
pub struct NetworkEmulateNetworkConditionsParams {
    pub offline: bool,
    pub latency: u64,
    pub download_throughput: u64,
    pub upload_throughput: u64,
    pub connection_type: Box<ConnectionType>,
    pub packet_loss: u64,
    pub packet_queue_length: i64,
    pub packet_reordering: bool,
}
/// Activates emulation of network conditions.
pub type NetworkEmulateNetworkConditionsReturns = ();
/// Enables network tracking, network events will now be delivered to the client.
pub struct NetworkEnableParams {
    pub max_total_buffer_size: i64,
    pub max_resource_buffer_size: i64,
    pub max_post_data_size: i64,
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
pub struct NetworkGetAllCookiesParams {
    pub cookies: Vec<Cookie>,
}
/// ⚠️ Experimental
/// Returns the DER-encoded certificate.
pub struct NetworkGetCertificateParams {
    pub origin: String,
}
/// ⚠️ Experimental
/// Returns the DER-encoded certificate.
pub struct NetworkGetCertificateParams {
    pub table_names: Vec<String>,
}
/** Returns all browser cookies for the current URL. Depending on the backend support, will return
detailed cookie information in the `cookies` field.*/
pub struct NetworkGetCookiesParams {
    pub urls: Vec<String>,
}
/** Returns all browser cookies for the current URL. Depending on the backend support, will return
detailed cookie information in the `cookies` field.*/
pub struct NetworkGetCookiesParams {
    pub cookies: Vec<Cookie>,
}
/// Returns content served for the given request.
pub struct NetworkGetResponseBodyParams {
    pub request_id: Box<NetworkRequestId>,
}
/// Returns content served for the given request.
pub struct NetworkGetResponseBodyParams {
    pub body: String,
    pub base64_encoded: bool,
}
/// Returns post data sent with the request. Returns an error when no data was sent with the request.
pub struct NetworkGetRequestPostDataParams {
    pub request_id: Box<NetworkRequestId>,
}
/// Returns post data sent with the request. Returns an error when no data was sent with the request.
pub struct NetworkGetRequestPostDataParams {
    pub post_data: String,
}
/// ⚠️ Experimental
/// Returns content served for the given currently intercepted request.
pub struct NetworkGetResponseBodyForInterceptionParams {
    pub interception_id: Box<InterceptionId>,
}
/// ⚠️ Experimental
/// Returns content served for the given currently intercepted request.
pub struct NetworkGetResponseBodyForInterceptionParams {
    pub body: String,
    pub base64_encoded: bool,
}
/// ⚠️ Experimental
/** Returns a handle to the stream representing the response body. Note that after this command,
the intercepted request can't be continued as is -- you either need to cancel it or to provide
the response body. The stream only supports sequential read, IO.read will fail if the position
is specified.*/
pub struct NetworkTakeResponseBodyForInterceptionAsStreamParams {
    pub interception_id: Box<InterceptionId>,
}
/// ⚠️ Experimental
/** Returns a handle to the stream representing the response body. Note that after this command,
the intercepted request can't be continued as is -- you either need to cancel it or to provide
the response body. The stream only supports sequential read, IO.read will fail if the position
is specified.*/
pub struct NetworkTakeResponseBodyForInterceptionAsStreamParams {
    pub stream: Box<StreamHandle>,
}
/// ⚠️ Experimental
/** This method sends a new XMLHttpRequest which is identical to the original one. The following
parameters should be identical: method, url, async, request body, extra headers, withCredentials
attribute, user, password.*/
pub struct NetworkReplayXhrParams {
    pub request_id: Box<NetworkRequestId>,
}
/// ⚠️ Experimental
/** This method sends a new XMLHttpRequest which is identical to the original one. The following
parameters should be identical: method, url, async, request body, extra headers, withCredentials
attribute, user, password.*/
pub type NetworkReplayXhrReturns = ();
/// ⚠️ Experimental
/// Searches for given string in response content.
pub struct NetworkSearchInResponseBodyParams {
    pub request_id: Box<NetworkRequestId>,
    pub query: String,
    pub case_sensitive: bool,
    pub is_regex: bool,
}
/// ⚠️ Experimental
/// Searches for given string in response content.
pub struct NetworkSearchInResponseBodyParams {
    pub result: Vec<SearchMatch>,
}
/// ⚠️ Experimental
/// Blocks URLs from loading.
pub struct NetworkSetBlockedUrLsParams {
    pub urls: Vec<String>,
}
/// ⚠️ Experimental
/// Blocks URLs from loading.
pub type NetworkSetBlockedUrLsReturns = ();
/// Toggles ignoring of service worker for each request.
pub struct NetworkSetBypassServiceWorkerParams {
    pub bypass: bool,
}
/// Toggles ignoring of service worker for each request.
pub type NetworkSetBypassServiceWorkerReturns = ();
/// Toggles ignoring cache for each request. If `true`, cache will not be used.
pub struct NetworkSetCacheDisabledParams {
    pub cache_disabled: bool,
}
/// Toggles ignoring cache for each request. If `true`, cache will not be used.
pub type NetworkSetCacheDisabledReturns = ();
/// Sets a cookie with the given cookie data; may overwrite equivalent cookies if they exist.
pub struct NetworkSetCookieParams {
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
/// Sets a cookie with the given cookie data; may overwrite equivalent cookies if they exist.
pub struct NetworkSetCookieParams {
    pub success: bool,
}
/// Sets given cookies.
pub struct NetworkSetCookiesParams {
    pub cookies: Vec<CookieParam>,
}
/// Sets given cookies.
pub type NetworkSetCookiesReturns = ();
/// Specifies whether to always send extra HTTP headers with the requests from this page.
pub struct NetworkSetExtraHttpHeadersParams {
    pub headers: Box<Headers>,
}
/// Specifies whether to always send extra HTTP headers with the requests from this page.
pub type NetworkSetExtraHttpHeadersReturns = ();
/// ⚠️ Experimental
/// Specifies whether to attach a page script stack id in requests
pub struct NetworkSetAttachDebugStackParams {
    pub enabled: bool,
}
/// ⚠️ Experimental
/// Specifies whether to attach a page script stack id in requests
pub type NetworkSetAttachDebugStackReturns = ();
#[deprecated]
/// ⚠️ Experimental
/** Sets the requests to intercept that match the provided patterns and optionally resource types.
Deprecated, please use Fetch.enable instead.*/
pub struct NetworkSetRequestInterceptionParams {
    pub patterns: Vec<()>,
}
#[deprecated]
/// ⚠️ Experimental
/** Sets the requests to intercept that match the provided patterns and optionally resource types.
Deprecated, please use Fetch.enable instead.*/
pub type NetworkSetRequestInterceptionReturns = ();
/// Allows overriding user agent with the given string.
pub struct NetworkSetUserAgentOverrideParams {
    pub user_agent: String,
    pub accept_language: String,
    pub platform: String,
    pub user_agent_metadata: Box<crate::emulation::UserAgentMetadata>,
}
/// Allows overriding user agent with the given string.
pub type NetworkSetUserAgentOverrideReturns = crate::emulation::EmulationSetUserAgentOverrideReturns;
/// ⚠️ Experimental
/** Enables streaming of the response for the given requestId.
If enabled, the dataReceived event contains the data that was received during streaming.*/
pub struct NetworkStreamResourceContentParams {
    pub request_id: Box<NetworkRequestId>,
}
/// ⚠️ Experimental
/** Enables streaming of the response for the given requestId.
If enabled, the dataReceived event contains the data that was received during streaming.*/
pub struct NetworkStreamResourceContentParams {
    pub buffered_data: String,
}
/// ⚠️ Experimental
/// Returns information about the COEP/COOP isolation status.
pub struct NetworkGetSecurityIsolationStatusParams {
    pub frame_id: Box<crate::page::FrameId>,
}
/// ⚠️ Experimental
/// Returns information about the COEP/COOP isolation status.
pub struct NetworkGetSecurityIsolationStatusParams {
    pub status: Box<SecurityIsolationStatus>,
}
/// ⚠️ Experimental
/** Enables tracking for the Reporting API, events generated by the Reporting API will now be delivered to the client.
Enabling triggers 'reportingApiReportAdded' for all existing reports.*/
pub struct NetworkEnableReportingApiParams {
    pub enable: bool,
}
/// ⚠️ Experimental
/** Enables tracking for the Reporting API, events generated by the Reporting API will now be delivered to the client.
Enabling triggers 'reportingApiReportAdded' for all existing reports.*/
pub type NetworkEnableReportingApiReturns = ();
/// ⚠️ Experimental
/// Fetches the resource and returns the content.
pub struct NetworkLoadNetworkResourceParams {
    pub frame_id: Box<crate::page::FrameId>,
    pub url: String,
    pub options: Box<LoadNetworkResourceOptions>,
}
/// ⚠️ Experimental
/// Fetches the resource and returns the content.
pub struct NetworkLoadNetworkResourceParams {
    pub resource: Box<LoadNetworkResourcePageResult>,
}
/// ⚠️ Experimental
/** Sets Controls for third-party cookie access
Page reload is required before the new cookie bahavior will be observed*/
pub struct NetworkSetCookieControlsParams {
    pub enable_third_party_cookie_restriction: bool,
    pub disable_third_party_cookie_metadata: bool,
    pub disable_third_party_cookie_heuristics: bool,
}
/// ⚠️ Experimental
/** Sets Controls for third-party cookie access
Page reload is required before the new cookie bahavior will be observed*/
pub type NetworkSetCookieControlsReturns = ();
/// Fired when data chunk was received over the network.
pub struct NetworkDataReceivedEvent {
    pub request_id: Box<NetworkRequestId>,
    pub timestamp: Box<MonotonicTime>,
    pub data_length: i64,
    pub encoded_data_length: i64,
    pub data: String,
}
/// Fired when EventSource message is received.
pub struct NetworkEventSourceMessageReceivedEvent {
    pub request_id: Box<NetworkRequestId>,
    pub timestamp: Box<MonotonicTime>,
    pub event_name: String,
    pub event_id: String,
    pub data: String,
}
/// Fired when HTTP request has failed to load.
pub struct NetworkLoadingFailedEvent {
    pub request_id: Box<NetworkRequestId>,
    pub timestamp: Box<MonotonicTime>,
    pub _type: Box<ResourceType>,
    pub error_text: String,
    pub canceled: bool,
    pub blocked_reason: Box<BlockedReason>,
    pub cors_error_status: Box<CorsErrorStatus>,
}
/// Fired when HTTP request has finished loading.
pub struct NetworkLoadingFinishedEvent {
    pub request_id: Box<NetworkRequestId>,
    pub timestamp: Box<MonotonicTime>,
    pub encoded_data_length: u64,
}
#[deprecated]
/// ⚠️ Experimental
/** Details of an intercepted HTTP request, which must be either allowed, blocked, modified or
mocked.
Deprecated, use Fetch.requestPaused instead.*/
pub struct NetworkRequestInterceptedEvent {
    pub interception_id: Box<InterceptionId>,
    pub request: Box<Request>,
    pub frame_id: Box<crate::page::FrameId>,
    pub resource_type: Box<ResourceType>,
    pub is_navigation_request: bool,
    pub is_download: bool,
    pub redirect_url: String,
    pub auth_challenge: Box<AuthChallenge>,
    pub response_error_reason: Box<ErrorReason>,
    pub response_status_code: i64,
    pub response_headers: Box<Headers>,
    pub request_id: Box<NetworkRequestId>,
}
/// Fired if request ended up loading from cache.
pub struct NetworkRequestServedFromCacheEvent {
    pub request_id: Box<NetworkRequestId>,
}
/// Fired when page is about to send HTTP request.
pub struct NetworkRequestWillBeSentEvent {
    pub request_id: Box<NetworkRequestId>,
    pub loader_id: Box<LoaderId>,
    pub document_url: String,
    pub request: Box<Request>,
    pub timestamp: Box<MonotonicTime>,
    pub wall_time: Box<NetworkTimeSinceEpoch>,
    pub initiator: Box<Initiator>,
    pub redirect_has_extra_info: bool,
    pub redirect_response: Box<Response>,
    pub _type: Box<ResourceType>,
    pub frame_id: Box<crate::page::FrameId>,
    pub has_user_gesture: bool,
}
/// ⚠️ Experimental
/// Fired when resource loading priority is changed
pub struct NetworkResourceChangedPriorityEvent {
    pub request_id: Box<NetworkRequestId>,
    pub new_priority: Box<ResourcePriority>,
    pub timestamp: Box<MonotonicTime>,
}
/// ⚠️ Experimental
/// Fired when a signed exchange was received over the network
pub struct NetworkSignedExchangeReceivedEvent {
    pub request_id: Box<NetworkRequestId>,
    pub info: Box<SignedExchangeInfo>,
}
/// Fired when HTTP response is available.
pub struct NetworkResponseReceivedEvent {
    pub request_id: Box<NetworkRequestId>,
    pub loader_id: Box<LoaderId>,
    pub timestamp: Box<MonotonicTime>,
    pub _type: Box<ResourceType>,
    pub response: Box<Response>,
    pub has_extra_info: bool,
    pub frame_id: Box<crate::page::FrameId>,
}
/// Fired when WebSocket is closed.
pub struct NetworkWebSocketClosedEvent {
    pub request_id: Box<NetworkRequestId>,
    pub timestamp: Box<MonotonicTime>,
}
/// Fired upon WebSocket creation.
pub struct NetworkWebSocketCreatedEvent {
    pub request_id: Box<NetworkRequestId>,
    pub url: String,
    pub initiator: Box<Initiator>,
}
/// Fired when WebSocket message error occurs.
pub struct NetworkWebSocketFrameErrorEvent {
    pub request_id: Box<NetworkRequestId>,
    pub timestamp: Box<MonotonicTime>,
    pub error_message: String,
}
/// Fired when WebSocket message is received.
pub struct NetworkWebSocketFrameReceivedEvent {
    pub request_id: Box<NetworkRequestId>,
    pub timestamp: Box<MonotonicTime>,
    pub response: Box<WebSocketFrame>,
}
/// Fired when WebSocket message is sent.
pub struct NetworkWebSocketFrameSentEvent {
    pub request_id: Box<NetworkRequestId>,
    pub timestamp: Box<MonotonicTime>,
    pub response: Box<WebSocketFrame>,
}
/// Fired when WebSocket handshake response becomes available.
pub struct NetworkWebSocketHandshakeResponseReceivedEvent {
    pub request_id: Box<NetworkRequestId>,
    pub timestamp: Box<MonotonicTime>,
    pub response: Box<WebSocketResponse>,
}
/// Fired when WebSocket is about to initiate handshake.
pub struct NetworkWebSocketWillSendHandshakeRequestEvent {
    pub request_id: Box<NetworkRequestId>,
    pub timestamp: Box<MonotonicTime>,
    pub wall_time: Box<NetworkTimeSinceEpoch>,
    pub request: Box<WebSocketRequest>,
}
/// Fired upon WebTransport creation.
pub struct NetworkWebTransportCreatedEvent {
    pub transport_id: Box<NetworkRequestId>,
    pub url: String,
    pub timestamp: Box<MonotonicTime>,
    pub initiator: Box<Initiator>,
}
/// Fired when WebTransport handshake is finished.
pub struct NetworkWebTransportConnectionEstablishedEvent {
    pub transport_id: Box<NetworkRequestId>,
    pub timestamp: Box<MonotonicTime>,
}
/// Fired when WebTransport is disposed.
pub struct NetworkWebTransportClosedEvent {
    pub transport_id: Box<NetworkRequestId>,
    pub timestamp: Box<MonotonicTime>,
}
/// ⚠️ Experimental
/// Fired upon direct_socket.TCPSocket creation.
pub struct NetworkDirectTcpSocketCreatedEvent {
    pub identifier: Box<NetworkRequestId>,
    pub remote_addr: String,
    pub remote_port: i64,
    pub options: Box<DirectTcpSocketOptions>,
    pub timestamp: Box<MonotonicTime>,
    pub initiator: Box<Initiator>,
}
/// ⚠️ Experimental
/// Fired when direct_socket.TCPSocket connection is opened.
pub struct NetworkDirectTcpSocketOpenedEvent {
    pub identifier: Box<NetworkRequestId>,
    pub remote_addr: String,
    pub remote_port: i64,
    pub timestamp: Box<MonotonicTime>,
    pub local_addr: String,
    pub local_port: i64,
}
/// ⚠️ Experimental
/// Fired when direct_socket.TCPSocket is aborted.
pub struct NetworkDirectTcpSocketAbortedEvent {
    pub identifier: Box<NetworkRequestId>,
    pub error_message: String,
    pub timestamp: Box<MonotonicTime>,
}
/// ⚠️ Experimental
/// Fired when direct_socket.TCPSocket is closed.
pub struct NetworkDirectTcpSocketClosedEvent {
    pub identifier: Box<NetworkRequestId>,
    pub timestamp: Box<MonotonicTime>,
}
/// ⚠️ Experimental
/// Fired when data is sent to tcp direct socket stream.
pub struct NetworkDirectTcpSocketChunkSentEvent {
    pub identifier: Box<NetworkRequestId>,
    pub data: String,
    pub timestamp: Box<MonotonicTime>,
}
/// ⚠️ Experimental
/// Fired when data is received from tcp direct socket stream.
pub struct NetworkDirectTcpSocketChunkReceivedEvent {
    pub identifier: Box<NetworkRequestId>,
    pub data: String,
    pub timestamp: Box<MonotonicTime>,
}
/// ⚠️ Experimental
/** Fired when there is an error
when writing to tcp direct socket stream.
For example, if user writes illegal type like string
instead of ArrayBuffer or ArrayBufferView.
There's no reporting for reading, because
we cannot know errors on the other side.*/
pub struct NetworkDirectTcpSocketChunkErrorEvent {
    pub identifier: Box<NetworkRequestId>,
    pub error_message: String,
    pub timestamp: Box<MonotonicTime>,
}
/// ⚠️ Experimental
/** Fired when additional information about a requestWillBeSent event is available from the
network stack. Not every requestWillBeSent event will have an additional
requestWillBeSentExtraInfo fired for it, and there is no guarantee whether requestWillBeSent
or requestWillBeSentExtraInfo will be fired first for the same request.*/
pub struct NetworkRequestWillBeSentExtraInfoEvent {
    pub request_id: Box<NetworkRequestId>,
    pub associated_cookies: Vec<AssociatedCookie>,
    pub headers: Box<Headers>,
    pub connect_timing: Box<ConnectTiming>,
    pub client_security_state: Box<ClientSecurityState>,
    pub site_has_cookie_in_other_partition: bool,
}
/// ⚠️ Experimental
/** Fired when additional information about a responseReceived event is available from the network
stack. Not every responseReceived event will have an additional responseReceivedExtraInfo for
it, and responseReceivedExtraInfo may be fired before or after responseReceived.*/
pub struct NetworkResponseReceivedExtraInfoEvent {
    pub request_id: Box<NetworkRequestId>,
    pub blocked_cookies: Vec<BlockedSetCookieWithReason>,
    pub headers: Box<Headers>,
    pub resource_ip_address_space: Box<IpAddressSpace>,
    pub status_code: i64,
    pub headers_text: String,
    pub cookie_partition_key: Box<CookiePartitionKey>,
    pub cookie_partition_key_opaque: bool,
    pub exempted_cookies: Vec<ExemptedSetCookieWithReason>,
}
/// ⚠️ Experimental
/** Fired when 103 Early Hints headers is received in addition to the common response.
Not every responseReceived event will have an responseReceivedEarlyHints fired.
Only one responseReceivedEarlyHints may be fired for eached responseReceived event.*/
pub struct NetworkResponseReceivedEarlyHintsEvent {
    pub request_id: Box<NetworkRequestId>,
    pub headers: Box<Headers>,
}
/// ⚠️ Experimental
/** Fired exactly once for each Trust Token operation. Depending on
the type of the operation and whether the operation succeeded or
failed, the event is fired before the corresponding request was sent
or after the response was received.*/
pub struct NetworkTrustTokenOperationDoneEvent {
    pub status: String,
    pub _type: Box<TrustTokenOperationType>,
    pub request_id: Box<NetworkRequestId>,
    pub top_level_origin: String,
    pub issuer_origin: String,
    pub issued_token_count: i64,
}
/// ⚠️ Experimental
/// Fired once security policy has been updated.
pub type NetworkPolicyUpdatedEvent = String;
/// ⚠️ Experimental
/** Fired once when parsing the .wbn file has succeeded.
The event contains the information about the web bundle contents.*/
pub struct NetworkSubresourceWebBundleMetadataReceivedEvent {
    pub request_id: Box<NetworkRequestId>,
    pub urls: Vec<String>,
}
/// ⚠️ Experimental
/// Fired once when parsing the .wbn file has failed.
pub struct NetworkSubresourceWebBundleMetadataErrorEvent {
    pub request_id: Box<NetworkRequestId>,
    pub error_message: String,
}
/// ⚠️ Experimental
/** Fired when handling requests for resources within a .wbn file.
Note: this will only be fired for resources that are requested by the webpage.*/
pub struct NetworkSubresourceWebBundleInnerResponseParsedEvent {
    pub inner_request_id: Box<NetworkRequestId>,
    pub inner_request_url: String,
    pub bundle_request_id: Box<NetworkRequestId>,
}
/// ⚠️ Experimental
/// Fired when request for resources within a .wbn file failed.
pub struct NetworkSubresourceWebBundleInnerResponseErrorEvent {
    pub inner_request_id: Box<NetworkRequestId>,
    pub inner_request_url: String,
    pub error_message: String,
    pub bundle_request_id: Box<NetworkRequestId>,
}
/// ⚠️ Experimental
/** Is sent whenever a new report is added.
And after 'enableReportingApi' for all existing reports.*/
pub struct NetworkReportingApiReportAddedEvent {
    pub report: Box<ReportingApiReport>,
}
/// ⚠️ Experimental
pub struct NetworkReportingApiReportUpdatedEvent {
    pub report: Box<ReportingApiReport>,
}
/// ⚠️ Experimental
pub struct NetworkReportingApiEndpointsChangedForOriginEvent {
    pub origin: String,
    pub endpoints: Vec<ReportingApiEndpoint>,
}
