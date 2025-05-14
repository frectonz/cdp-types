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
