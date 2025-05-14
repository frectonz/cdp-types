pub use crate::common::*;
use crate::network::*;
/// Information about a cookie that is affected by an inspector issue.
pub struct AffectedCookie {
    pub name: String,
    pub path: String,
    pub domain: String,
}
/// Information about a request that is affected by an inspector issue.
pub struct AffectedRequest {
    pub request_id: (),
    pub url: String,
}
/// Information about the frame affected by an inspector issue.
pub struct AffectedFrame {
    pub frame_id: (),
}
pub enum CookieExclusionReason {
    ExcludeSameSiteUnspecifiedTreatedAsLax,
    ExcludeSameSiteNoneInsecure,
    ExcludeSameSiteLax,
    ExcludeSameSiteStrict,
    ExcludeInvalidSameParty,
    ExcludeSamePartyCrossPartyContext,
    ExcludeDomainNonAscii,
    ExcludeThirdPartyCookieBlockedInFirstPartySet,
    ExcludeThirdPartyPhaseout,
    ExcludePortMismatch,
    ExcludeSchemeMismatch,
}
pub enum CookieWarningReason {
    WarnSameSiteUnspecifiedCrossSiteContext,
    WarnSameSiteNoneInsecure,
    WarnSameSiteUnspecifiedLaxAllowUnsafe,
    WarnSameSiteStrictLaxDowngradeStrict,
    WarnSameSiteStrictCrossDowngradeStrict,
    WarnSameSiteStrictCrossDowngradeLax,
    WarnSameSiteLaxCrossDowngradeStrict,
    WarnSameSiteLaxCrossDowngradeLax,
    WarnAttributeValueExceedsMaxSize,
    WarnDomainNonAscii,
    WarnThirdPartyPhaseout,
    WarnCrossSiteRedirectDowngradeChangesInclusion,
    WarnDeprecationTrialMetadata,
    WarnThirdPartyCookieHeuristic,
}
pub enum CookieOperation {
    SetCookie,
    ReadCookie,
}
/// Represents the category of insight that a cookie issue falls under.
pub enum InsightType {
    GitHubResource,
    GracePeriod,
    Heuristics,
}
/// Information about the suggested solution to a cookie issue.
pub struct CookieIssueInsight {
    pub _type: (),
    pub table_entry_url: String,
}
/** This information is currently necessary, as the front-end has a difficult
time finding a specific cookie. With this, we can convey specific error
information without the cookie.*/
pub struct CookieIssueDetails {
    pub cookie: (),
    pub raw_cookie_line: String,
    pub cookie_warning_reasons: (),
    pub cookie_exclusion_reasons: (),
    pub operation: (),
    pub site_for_cookies: String,
    pub cookie_url: String,
    pub request: (),
    pub insight: (),
}
pub enum MixedContentResolutionStatus {
    MixedContentBlocked,
    MixedContentAutomaticallyUpgraded,
    MixedContentWarning,
}
pub enum MixedContentResourceType {
    AttributionSrc,
    Audio,
    Beacon,
    CspReport,
    Download,
    EventSource,
    Favicon,
    Font,
    Form,
    Frame,
    Image,
    Import,
    Json,
    Manifest,
    Ping,
    PluginData,
    PluginResource,
    Prefetch,
    Resource,
    Script,
    ServiceWorker,
    SharedWorker,
    SpeculationRules,
    Stylesheet,
    Track,
    Video,
    Worker,
    XmlHttpRequest,
    Xslt,
}
pub struct MixedContentIssueDetails {
    pub resource_type: (),
    pub resolution_status: (),
    pub insecure_url: String,
    pub main_resource_url: String,
    pub request: (),
    pub frame: (),
}
/** Enum indicating the reason a response has been blocked. These reasons are
refinements of the net error BLOCKED_BY_RESPONSE.*/
pub enum BlockedByResponseReason {
    CoepFrameResourceNeedsCoepHeader,
    CoopSandboxedIFrameCannotNavigateToCoopPage,
    CorpNotSameOrigin,
    CorpNotSameOriginAfterDefaultedToSameOriginByCoep,
    CorpNotSameOriginAfterDefaultedToSameOriginByDip,
    CorpNotSameOriginAfterDefaultedToSameOriginByCoepAndDip,
    CorpNotSameSite,
    SriMessageSignatureMismatch,
}
/** Details for a request that has been blocked with the BLOCKED_BY_RESPONSE
code. Currently only used for COEP/COOP, but may be extended to include
some CSP errors in the future.*/
pub struct BlockedByResponseIssueDetails {
    pub request: (),
    pub parent_frame: (),
    pub blocked_frame: (),
    pub reason: (),
}
pub enum HeavyAdResolutionStatus {
    HeavyAdBlocked,
    HeavyAdWarning,
}
pub enum HeavyAdReason {
    NetworkTotalLimit,
    CpuTotalLimit,
    CpuPeakLimit,
}
pub struct HeavyAdIssueDetails {
    pub resolution: (),
    pub reason: (),
    pub frame: (),
}
pub enum ContentSecurityPolicyViolationType {
    KInlineViolation,
    KEvalViolation,
    KUrlViolation,
    KSriViolation,
    KTrustedTypesSinkViolation,
    KTrustedTypesPolicyViolation,
    KWasmEvalViolation,
}
pub struct SourceCodeLocation {
    pub script_id: (),
    pub url: String,
    pub line_number: i64,
    pub column_number: i64,
}
pub struct ContentSecurityPolicyIssueDetails {
    pub blocked_url: String,
    pub violated_directive: String,
    pub is_report_only: (),
    pub content_security_policy_violation_type: (),
    pub frame_ancestor: (),
    pub source_code_location: (),
    pub violating_node_id: (),
}
pub enum SharedArrayBufferIssueType {
    TransferIssue,
    CreationIssue,
}
/** Details for a issue arising from an SAB being instantiated in, or
transferred to a context that is not cross-origin isolated.*/
pub struct SharedArrayBufferIssueDetails {
    pub source_code_location: (),
    pub is_warning: (),
    pub _type: (),
}
pub struct LowTextContrastIssueDetails {
    pub violating_node_id: (),
    pub violating_node_selector: String,
    pub contrast_ratio: u64,
    pub threshold_aa: u64,
    pub threshold_aaa: u64,
    pub font_size: String,
    pub font_weight: String,
}
/** Details for a CORS related issue, e.g. a warning or error related to
CORS RFC1918 enforcement.*/
pub struct CorsIssueDetails {
    pub cors_error_status: (),
    pub is_warning: (),
    pub request: (),
    pub location: (),
    pub initiator_origin: String,
    pub resource_ip_address_space: (),
    pub client_security_state: (),
}
pub enum AttributionReportingIssueType {
    PermissionPolicyDisabled,
    UntrustworthyReportingOrigin,
    InsecureContext,
    InvalidHeader,
    InvalidRegisterTriggerHeader,
    SourceAndTriggerHeaders,
    SourceIgnored,
    TriggerIgnored,
    OsSourceIgnored,
    OsTriggerIgnored,
    InvalidRegisterOsSourceHeader,
    InvalidRegisterOsTriggerHeader,
    WebAndOsHeaders,
    NoWebOrOsSupport,
    NavigationRegistrationWithoutTransientUserActivation,
    InvalidInfoHeader,
    NoRegisterSourceHeader,
    NoRegisterTriggerHeader,
    NoRegisterOsSourceHeader,
    NoRegisterOsTriggerHeader,
    NavigationRegistrationUniqueScopeAlreadySet,
}
pub enum SharedDictionaryError {
    UseErrorCrossOriginNoCorsRequest,
    UseErrorDictionaryLoadFailure,
    UseErrorMatchingDictionaryNotUsed,
    UseErrorUnexpectedContentDictionaryHeader,
    WriteErrorCossOriginNoCorsRequest,
    WriteErrorDisallowedBySettings,
    WriteErrorExpiredResponse,
    WriteErrorFeatureDisabled,
    WriteErrorInsufficientResources,
    WriteErrorInvalidMatchField,
    WriteErrorInvalidStructuredHeader,
    WriteErrorNavigationRequest,
    WriteErrorNoMatchField,
    WriteErrorNonListMatchDestField,
    WriteErrorNonSecureContext,
    WriteErrorNonStringIdField,
    WriteErrorNonStringInMatchDestList,
    WriteErrorNonStringMatchField,
    WriteErrorNonTokenTypeField,
    WriteErrorRequestAborted,
    WriteErrorShuttingDown,
    WriteErrorTooLongIdField,
    WriteErrorUnsupportedType,
}
pub enum SriMessageSignatureError {
    MissingSignatureHeader,
    MissingSignatureInputHeader,
    InvalidSignatureHeader,
    InvalidSignatureInputHeader,
    SignatureHeaderValueIsNotByteSequence,
    SignatureHeaderValueIsParameterized,
    SignatureHeaderValueIsIncorrectLength,
    SignatureInputHeaderMissingLabel,
    SignatureInputHeaderValueNotInnerList,
    SignatureInputHeaderValueMissingComponents,
    SignatureInputHeaderInvalidComponentType,
    SignatureInputHeaderInvalidComponentName,
    SignatureInputHeaderInvalidHeaderComponentParameter,
    SignatureInputHeaderInvalidDerivedComponentParameter,
    SignatureInputHeaderKeyIdLength,
    SignatureInputHeaderInvalidParameter,
    SignatureInputHeaderMissingRequiredParameters,
    ValidationFailedSignatureExpired,
    ValidationFailedInvalidLength,
    ValidationFailedSignatureMismatch,
    ValidationFailedIntegrityMismatch,
}
/** Details for issues around "Attribution Reporting API" usage.
Explainer: https://github.com/WICG/attribution-reporting-api*/
pub struct AttributionReportingIssueDetails {
    pub violation_type: (),
    pub request: (),
    pub violating_node_id: (),
    pub invalid_parameter: String,
}
/** Details for issues about documents in Quirks Mode
or Limited Quirks Mode that affects page layouting.*/
pub struct QuirksModeIssueDetails {
    pub is_limited_quirks_mode: (),
    pub document_node_id: (),
    pub url: String,
    pub frame_id: (),
    pub loader_id: (),
}
#[deprecated]
pub struct NavigatorUserAgentIssueDetails {
    pub url: String,
    pub location: (),
}
pub struct SharedDictionaryIssueDetails {
    pub shared_dictionary_error: (),
    pub request: (),
}
pub struct SriMessageSignatureIssueDetails {
    pub error: (),
    pub signature_base: String,
    pub integrity_assertions: (),
    pub request: (),
}
pub enum GenericIssueErrorType {
    FormLabelForNameError,
    FormDuplicateIdForInputError,
    FormInputWithNoLabelError,
    FormAutocompleteAttributeEmptyError,
    FormEmptyIdAndNameAttributesForInputError,
    FormAriaLabelledByToNonExistingId,
    FormInputAssignedAutocompleteValueToIdOrNameAttributeError,
    FormLabelHasNeitherForNorNestedInput,
    FormLabelForMatchesNonExistingIdError,
    FormInputHasWrongButWellIntendedAutocompleteValueError,
    ResponseWasBlockedByOrb,
}
/// Depending on the concrete errorType, different properties are set.
pub struct GenericIssueDetails {
    pub error_type: (),
    pub frame_id: (),
    pub violating_node_id: (),
    pub violating_node_attribute: String,
    pub request: (),
}
/** This issue tracks information needed to print a deprecation message.
https://source.chromium.org/chromium/chromium/src/+/main:third_party/blink/renderer/core/frame/third_party/blink/renderer/core/frame/deprecation/README.md*/
pub struct DeprecationIssueDetails {
    pub affected_frame: (),
    pub source_code_location: (),
    pub _type: String,
}
/** This issue warns about sites in the redirect chain of a finished navigation
that may be flagged as trackers and have their state cleared if they don't
receive a user interaction. Note that in this context 'site' means eTLD+1.
For example, if the URL `https://example.test:80/bounce` was in the
redirect chain, the site reported would be `example.test`.*/
pub struct BounceTrackingIssueDetails {
    pub tracking_sites: (),
}
/** This issue warns about third-party sites that are accessing cookies on the
current page, and have been permitted due to having a global metadata grant.
Note that in this context 'site' means eTLD+1. For example, if the URL
`https://example.test:80/web_page` was accessing cookies, the site reported
would be `example.test`.*/
pub struct CookieDeprecationMetadataIssueDetails {
    pub allowed_sites: (),
    pub opt_out_percentage: u64,
    pub is_opt_out_top_level: (),
    pub operation: (),
}
pub enum ClientHintIssueReason {
    MetaTagAllowListInvalidOrigin,
    MetaTagModifiedHtml,
}
pub struct FederatedAuthRequestIssueDetails {
    pub federated_auth_request_issue_reason: (),
}
/** Represents the failure reason when a federated authentication reason fails.
Should be updated alongside RequestIdTokenStatus in
third_party/blink/public/mojom/devtools/inspector_issue.mojom to include
all cases except for success.*/
pub enum FederatedAuthRequestIssueReason {
    ShouldEmbargo,
    TooManyRequests,
    WellKnownHttpNotFound,
    WellKnownNoResponse,
    WellKnownInvalidResponse,
    WellKnownListEmpty,
    WellKnownInvalidContentType,
    ConfigNotInWellKnown,
    WellKnownTooBig,
    ConfigHttpNotFound,
    ConfigNoResponse,
    ConfigInvalidResponse,
    ConfigInvalidContentType,
    ClientMetadataHttpNotFound,
    ClientMetadataNoResponse,
    ClientMetadataInvalidResponse,
    ClientMetadataInvalidContentType,
    IdpNotPotentiallyTrustworthy,
    DisabledInSettings,
    DisabledInFlags,
    ErrorFetchingSignin,
    InvalidSigninResponse,
    AccountsHttpNotFound,
    AccountsNoResponse,
    AccountsInvalidResponse,
    AccountsListEmpty,
    AccountsInvalidContentType,
    IdTokenHttpNotFound,
    IdTokenNoResponse,
    IdTokenInvalidResponse,
    IdTokenIdpErrorResponse,
    IdTokenCrossSiteIdpErrorResponse,
    IdTokenInvalidRequest,
    IdTokenInvalidContentType,
    ErrorIdToken,
    Canceled,
    RpPageNotVisible,
    SilentMediationFailure,
    ThirdPartyCookiesBlocked,
    NotSignedInWithIdp,
    MissingTransientUserActivation,
    ReplacedByActiveMode,
    InvalidFieldsSpecified,
    RelyingPartyOriginIsOpaque,
    TypeNotMatching,
    UiDismissedNoEmbargo,
    CorsError,
    SuppressedBySegmentationPlatform,
}
pub struct FederatedAuthUserInfoRequestIssueDetails {
    pub federated_auth_user_info_request_issue_reason: (),
}
/** Represents the failure reason when a getUserInfo() call fails.
Should be updated alongside FederatedAuthUserInfoRequestResult in
third_party/blink/public/mojom/devtools/inspector_issue.mojom.*/
pub enum FederatedAuthUserInfoRequestIssueReason {
    NotSameOrigin,
    NotIframe,
    NotPotentiallyTrustworthy,
    NoApiPermission,
    NotSignedInWithIdp,
    NoAccountSharingPermission,
    InvalidConfigOrWellKnown,
    InvalidAccountsResponse,
    NoReturningUserFromFetchedAccounts,
}
/** This issue tracks client hints related issues. It's used to deprecate old
features, encourage the use of new ones, and provide general guidance.*/
pub struct ClientHintIssueDetails {
    pub source_code_location: (),
    pub client_hint_issue_reason: (),
}
pub struct FailedRequestInfo {
    pub url: String,
    pub failure_message: String,
    pub request_id: (),
}
pub enum PartitioningBlobUrlInfo {
    BlockedCrossPartitionFetching,
    EnforceNoopenerForNavigation,
}
pub struct PartitioningBlobUrlIssueDetails {
    pub url: String,
    pub partitioning_blob_url_info: (),
}
pub enum SelectElementAccessibilityIssueReason {
    DisallowedSelectChild,
    DisallowedOptGroupChild,
    NonPhrasingContentOptionChild,
    InteractiveContentOptionChild,
    InteractiveContentLegendChild,
}
/// This issue warns about errors in the select element content model.
pub struct SelectElementAccessibilityIssueDetails {
    pub node_id: (),
    pub select_element_accessibility_issue_reason: (),
    pub has_disallowed_attributes: (),
}
pub enum StyleSheetLoadingIssueReason {
    LateImportRule,
    RequestFailed,
}
/// This issue warns when a referenced stylesheet couldn't be loaded.
pub struct StylesheetLoadingIssueDetails {
    pub source_code_location: (),
    pub style_sheet_loading_issue_reason: (),
    pub failed_request_info: (),
}
pub enum PropertyRuleIssueReason {
    InvalidSyntax,
    InvalidInitialValue,
    InvalidInherits,
    InvalidName,
}
/** This issue warns about errors in property rules that lead to property
registrations being ignored.*/
pub struct PropertyRuleIssueDetails {
    pub source_code_location: (),
    pub property_rule_issue_reason: (),
    pub property_value: String,
}
pub enum UserReidentificationIssueType {
    BlockedFrameNavigation,
    BlockedSubresource,
}
/** This issue warns about uses of APIs that may be considered misuse to
re-identify users.*/
pub struct UserReidentificationIssueDetails {
    pub _type: (),
    pub request: (),
}
/** A unique identifier for the type of issue. Each type may use one of the
optional fields in InspectorIssueDetails to convey more specific
information about the kind of issue.*/
pub enum InspectorIssueCode {
    CookieIssue,
    MixedContentIssue,
    BlockedByResponseIssue,
    HeavyAdIssue,
    ContentSecurityPolicyIssue,
    SharedArrayBufferIssue,
    LowTextContrastIssue,
    CorsIssue,
    AttributionReportingIssue,
    QuirksModeIssue,
    PartitioningBlobUrlIssue,
    NavigatorUserAgentIssue,
    GenericIssue,
    DeprecationIssue,
    ClientHintIssue,
    FederatedAuthRequestIssue,
    BounceTrackingIssue,
    CookieDeprecationMetadataIssue,
    StylesheetLoadingIssue,
    FederatedAuthUserInfoRequestIssue,
    PropertyRuleIssue,
    SharedDictionaryIssue,
    SelectElementAccessibilityIssue,
    SriMessageSignatureIssue,
    UserReidentificationIssue,
}
/** This struct holds a list of optional fields with additional information
specific to the kind of issue. When adding a new issue code, please also
add a new optional field to this type.*/
pub struct InspectorIssueDetails {
    pub cookie_issue_details: (),
    pub mixed_content_issue_details: (),
    pub blocked_by_response_issue_details: (),
    pub heavy_ad_issue_details: (),
    pub content_security_policy_issue_details: (),
    pub shared_array_buffer_issue_details: (),
    pub low_text_contrast_issue_details: (),
    pub cors_issue_details: (),
    pub attribution_reporting_issue_details: (),
    pub quirks_mode_issue_details: (),
    pub partitioning_blob_url_issue_details: (),
    pub navigator_user_agent_issue_details: (),
    pub generic_issue_details: (),
    pub deprecation_issue_details: (),
    pub client_hint_issue_details: (),
    pub federated_auth_request_issue_details: (),
    pub bounce_tracking_issue_details: (),
    pub cookie_deprecation_metadata_issue_details: (),
    pub stylesheet_loading_issue_details: (),
    pub property_rule_issue_details: (),
    pub federated_auth_user_info_request_issue_details: (),
    pub shared_dictionary_issue_details: (),
    pub select_element_accessibility_issue_details: (),
    pub sri_message_signature_issue_details: (),
    pub user_reidentification_issue_details: (),
}
/** A unique id for a DevTools inspector issue. Allows other entities (e.g.
exceptions, CDP message, console messages, etc.) to reference an issue.*/
pub struct IssueId(String);
/// An inspector issue reported from the back-end.
pub struct InspectorIssue {
    pub code: (),
    pub details: (),
    pub issue_id: (),
}
