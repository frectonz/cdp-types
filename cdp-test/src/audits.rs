use crate::common::*;
use crate::network::*;
use crate::page::*;
use crate::dom::*;
/// Information about a cookie that is affected by an inspector issue.
pub struct AffectedCookie {
    pub name: String,
    pub path: String,
    pub domain: String,
}
/// Information about a request that is affected by an inspector issue.
pub struct AffectedRequest {
    pub request_id: Box<NetworkRequestId>,
    pub url: String,
}
/// Information about the frame affected by an inspector issue.
pub struct AffectedFrame {
    pub frame_id: Box<crate::page::FrameId>,
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
    pub _type: Box<InsightType>,
    pub table_entry_url: String,
}
/** This information is currently necessary, as the front-end has a difficult
time finding a specific cookie. With this, we can convey specific error
information without the cookie.*/
pub struct CookieIssueDetails {
    pub cookie: Box<AffectedCookie>,
    pub raw_cookie_line: String,
    pub cookie_warning_reasons: Vec<CookieWarningReason>,
    pub cookie_exclusion_reasons: Vec<CookieExclusionReason>,
    pub operation: Box<CookieOperation>,
    pub site_for_cookies: String,
    pub cookie_url: String,
    pub request: Box<AffectedRequest>,
    pub insight: Box<CookieIssueInsight>,
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
    pub resource_type: Box<MixedContentResourceType>,
    pub resolution_status: Box<MixedContentResolutionStatus>,
    pub insecure_url: String,
    pub main_resource_url: String,
    pub request: Box<AffectedRequest>,
    pub frame: Box<AffectedFrame>,
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
    pub request: Box<AffectedRequest>,
    pub parent_frame: Box<AffectedFrame>,
    pub blocked_frame: Box<AffectedFrame>,
    pub reason: Box<BlockedByResponseReason>,
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
    pub resolution: Box<HeavyAdResolutionStatus>,
    pub reason: Box<HeavyAdReason>,
    pub frame: Box<AffectedFrame>,
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
    pub script_id: Box<()>,
    pub url: String,
    pub line_number: i64,
    pub column_number: i64,
}
pub struct ContentSecurityPolicyIssueDetails {
    pub blocked_url: String,
    pub violated_directive: String,
    pub is_report_only: bool,
    pub content_security_policy_violation_type: Box<ContentSecurityPolicyViolationType>,
    pub frame_ancestor: Box<AffectedFrame>,
    pub source_code_location: Box<SourceCodeLocation>,
    pub violating_node_id: Box<BackendNodeId>,
}
pub enum SharedArrayBufferIssueType {
    TransferIssue,
    CreationIssue,
}
/** Details for a issue arising from an SAB being instantiated in, or
transferred to a context that is not cross-origin isolated.*/
pub struct SharedArrayBufferIssueDetails {
    pub source_code_location: Box<SourceCodeLocation>,
    pub is_warning: bool,
    pub _type: Box<SharedArrayBufferIssueType>,
}
pub struct LowTextContrastIssueDetails {
    pub violating_node_id: Box<BackendNodeId>,
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
    pub cors_error_status: Box<CorsErrorStatus>,
    pub is_warning: bool,
    pub request: Box<AffectedRequest>,
    pub location: Box<SourceCodeLocation>,
    pub initiator_origin: String,
    pub resource_ip_address_space: Box<IpAddressSpace>,
    pub client_security_state: Box<ClientSecurityState>,
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
    pub violation_type: Box<AttributionReportingIssueType>,
    pub request: Box<AffectedRequest>,
    pub violating_node_id: Box<BackendNodeId>,
    pub invalid_parameter: String,
}
/** Details for issues about documents in Quirks Mode
or Limited Quirks Mode that affects page layouting.*/
pub struct QuirksModeIssueDetails {
    pub is_limited_quirks_mode: bool,
    pub document_node_id: Box<BackendNodeId>,
    pub url: String,
    pub frame_id: Box<crate::page::FrameId>,
    pub loader_id: Box<LoaderId>,
}
#[deprecated]
pub struct NavigatorUserAgentIssueDetails {
    pub url: String,
    pub location: Box<SourceCodeLocation>,
}
pub struct SharedDictionaryIssueDetails {
    pub shared_dictionary_error: Box<SharedDictionaryError>,
    pub request: Box<AffectedRequest>,
}
pub struct SriMessageSignatureIssueDetails {
    pub error: Box<SriMessageSignatureError>,
    pub signature_base: String,
    pub integrity_assertions: Vec<String>,
    pub request: Box<AffectedRequest>,
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
    pub error_type: Box<GenericIssueErrorType>,
    pub frame_id: Box<crate::page::FrameId>,
    pub violating_node_id: Box<BackendNodeId>,
    pub violating_node_attribute: String,
    pub request: Box<AffectedRequest>,
}
/** This issue tracks information needed to print a deprecation message.
https://source.chromium.org/chromium/chromium/src/+/main:third_party/blink/renderer/core/frame/third_party/blink/renderer/core/frame/deprecation/README.md*/
pub struct DeprecationIssueDetails {
    pub affected_frame: Box<AffectedFrame>,
    pub source_code_location: Box<SourceCodeLocation>,
    pub _type: String,
}
/** This issue warns about sites in the redirect chain of a finished navigation
that may be flagged as trackers and have their state cleared if they don't
receive a user interaction. Note that in this context 'site' means eTLD+1.
For example, if the URL `https://example.test:80/bounce` was in the
redirect chain, the site reported would be `example.test`.*/
pub struct BounceTrackingIssueDetails {
    pub tracking_sites: Vec<String>,
}
/** This issue warns about third-party sites that are accessing cookies on the
current page, and have been permitted due to having a global metadata grant.
Note that in this context 'site' means eTLD+1. For example, if the URL
`https://example.test:80/web_page` was accessing cookies, the site reported
would be `example.test`.*/
pub struct CookieDeprecationMetadataIssueDetails {
    pub allowed_sites: Vec<String>,
    pub opt_out_percentage: u64,
    pub is_opt_out_top_level: bool,
    pub operation: Box<CookieOperation>,
}
pub enum ClientHintIssueReason {
    MetaTagAllowListInvalidOrigin,
    MetaTagModifiedHtml,
}
pub struct FederatedAuthRequestIssueDetails {
    pub federated_auth_request_issue_reason: Box<FederatedAuthRequestIssueReason>,
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
    pub federated_auth_user_info_request_issue_reason: Box<
        FederatedAuthUserInfoRequestIssueReason,
    >,
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
    pub source_code_location: Box<SourceCodeLocation>,
    pub client_hint_issue_reason: Box<ClientHintIssueReason>,
}
pub struct FailedRequestInfo {
    pub url: String,
    pub failure_message: String,
    pub request_id: Box<NetworkRequestId>,
}
pub enum PartitioningBlobUrlInfo {
    BlockedCrossPartitionFetching,
    EnforceNoopenerForNavigation,
}
pub struct PartitioningBlobUrlIssueDetails {
    pub url: String,
    pub partitioning_blob_url_info: Box<PartitioningBlobUrlInfo>,
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
    pub node_id: Box<BackendNodeId>,
    pub select_element_accessibility_issue_reason: Box<
        SelectElementAccessibilityIssueReason,
    >,
    pub has_disallowed_attributes: bool,
}
pub enum StyleSheetLoadingIssueReason {
    LateImportRule,
    RequestFailed,
}
/// This issue warns when a referenced stylesheet couldn't be loaded.
pub struct StylesheetLoadingIssueDetails {
    pub source_code_location: Box<SourceCodeLocation>,
    pub style_sheet_loading_issue_reason: Box<StyleSheetLoadingIssueReason>,
    pub failed_request_info: Box<FailedRequestInfo>,
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
    pub source_code_location: Box<SourceCodeLocation>,
    pub property_rule_issue_reason: Box<PropertyRuleIssueReason>,
    pub property_value: String,
}
pub enum UserReidentificationIssueType {
    BlockedFrameNavigation,
    BlockedSubresource,
}
/** This issue warns about uses of APIs that may be considered misuse to
re-identify users.*/
pub struct UserReidentificationIssueDetails {
    pub _type: Box<UserReidentificationIssueType>,
    pub request: Box<AffectedRequest>,
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
    pub cookie_issue_details: Box<CookieIssueDetails>,
    pub mixed_content_issue_details: Box<MixedContentIssueDetails>,
    pub blocked_by_response_issue_details: Box<BlockedByResponseIssueDetails>,
    pub heavy_ad_issue_details: Box<HeavyAdIssueDetails>,
    pub content_security_policy_issue_details: Box<ContentSecurityPolicyIssueDetails>,
    pub shared_array_buffer_issue_details: Box<SharedArrayBufferIssueDetails>,
    pub low_text_contrast_issue_details: Box<LowTextContrastIssueDetails>,
    pub cors_issue_details: Box<CorsIssueDetails>,
    pub attribution_reporting_issue_details: Box<AttributionReportingIssueDetails>,
    pub quirks_mode_issue_details: Box<QuirksModeIssueDetails>,
    pub partitioning_blob_url_issue_details: Box<PartitioningBlobUrlIssueDetails>,
    pub navigator_user_agent_issue_details: Box<NavigatorUserAgentIssueDetails>,
    pub generic_issue_details: Box<GenericIssueDetails>,
    pub deprecation_issue_details: Box<DeprecationIssueDetails>,
    pub client_hint_issue_details: Box<ClientHintIssueDetails>,
    pub federated_auth_request_issue_details: Box<FederatedAuthRequestIssueDetails>,
    pub bounce_tracking_issue_details: Box<BounceTrackingIssueDetails>,
    pub cookie_deprecation_metadata_issue_details: Box<
        CookieDeprecationMetadataIssueDetails,
    >,
    pub stylesheet_loading_issue_details: Box<StylesheetLoadingIssueDetails>,
    pub property_rule_issue_details: Box<PropertyRuleIssueDetails>,
    pub federated_auth_user_info_request_issue_details: Box<
        FederatedAuthUserInfoRequestIssueDetails,
    >,
    pub shared_dictionary_issue_details: Box<SharedDictionaryIssueDetails>,
    pub select_element_accessibility_issue_details: Box<
        SelectElementAccessibilityIssueDetails,
    >,
    pub sri_message_signature_issue_details: Box<SriMessageSignatureIssueDetails>,
    pub user_reidentification_issue_details: Box<UserReidentificationIssueDetails>,
}
/** A unique id for a DevTools inspector issue. Allows other entities (e.g.
exceptions, CDP message, console messages, etc.) to reference an issue.*/
pub struct IssueId(String);
/// An inspector issue reported from the back-end.
pub struct InspectorIssue {
    pub code: Box<InspectorIssueCode>,
    pub details: Box<InspectorIssueDetails>,
    pub issue_id: Box<IssueId>,
}
/** Returns the response body and size if it were re-encoded with the specified settings. Only
applies to images.*/
pub struct AuditsGetEncodedResponseParams {
    pub request_id: Box<NetworkRequestId>,
    pub encoding: String,
    pub quality: u64,
    pub size_only: bool,
}
/** Returns the response body and size if it were re-encoded with the specified settings. Only
applies to images.*/
pub struct AuditsGetEncodedResponseParams {
    pub body: String,
    pub original_size: i64,
    pub encoded_size: i64,
}
/// Disables issues domain, prevents further issues from being reported to the client.
pub type AuditsDisableParams = ();
/// Disables issues domain, prevents further issues from being reported to the client.
pub type AuditsDisableReturns = ();
/** Enables issues domain, sends the issues collected so far to the client by means of the
`issueAdded` event.*/
pub type AuditsEnableParams = ();
/** Enables issues domain, sends the issues collected so far to the client by means of the
`issueAdded` event.*/
pub type AuditsEnableReturns = ();
/** Runs the contrast check for the target page. Found issues are reported
using Audits.issueAdded event.*/
pub struct AuditsCheckContrastParams {
    pub report_aaa: bool,
}
/** Runs the contrast check for the target page. Found issues are reported
using Audits.issueAdded event.*/
pub type AuditsCheckContrastReturns = ();
/** Runs the form issues check for the target page. Found issues are reported
using Audits.issueAdded event.*/
pub type AuditsCheckFormsIssuesParams = ();
/** Runs the form issues check for the target page. Found issues are reported
using Audits.issueAdded event.*/
pub struct AuditsCheckFormsIssuesParams {
    pub form_issues: Vec<GenericIssueDetails>,
}
pub struct AuditsIssueAddedEvent {
    pub issue: Box<InspectorIssue>,
}
