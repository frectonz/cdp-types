use crate::network::*;
/// Information about a cookie that is affected by an inspector issue.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-AffectedCookie>
pub struct AuditsAffectedCookie {
    pub name: String,
    pub path: String,
    pub domain: String,
}
/// Information about a request that is affected by an inspector issue.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-AffectedRequest>
pub struct AuditsAffectedRequest {
    pub request_id: (),
    pub url: String,
}
/// Information about the frame affected by an inspector issue.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-AffectedFrame>
pub struct AuditsAffectedFrame {
    pub frame_id: (),
}
/// <https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-CookieExclusionReason>
pub enum AuditsCookieExclusionReason {
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
/// <https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-CookieWarningReason>
pub enum AuditsCookieWarningReason {
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
/// <https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-CookieOperation>
pub enum AuditsCookieOperation {
    SetCookie,
    ReadCookie,
}
/// Represents the category of insight that a cookie issue falls under.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-InsightType>
pub enum AuditsInsightType {
    GitHubResource,
    GracePeriod,
    Heuristics,
}
/// Information about the suggested solution to a cookie issue.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-CookieIssueInsight>
pub struct AuditsCookieIssueInsight {
    pub _type: (),
    pub table_entry_url: String,
}
/** This information is currently necessary, as the front-end has a difficult
time finding a specific cookie. With this, we can convey specific error
information without the cookie.*/
/// <https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-CookieIssueDetails>
pub struct AuditsCookieIssueDetails {
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
/// <https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-MixedContentResolutionStatus>
pub enum AuditsMixedContentResolutionStatus {
    MixedContentBlocked,
    MixedContentAutomaticallyUpgraded,
    MixedContentWarning,
}
/// <https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-MixedContentResourceType>
pub enum AuditsMixedContentResourceType {
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
/// <https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-MixedContentIssueDetails>
pub struct AuditsMixedContentIssueDetails {
    pub resource_type: (),
    pub resolution_status: (),
    pub insecure_url: String,
    pub main_resource_url: String,
    pub request: (),
    pub frame: (),
}
/** Enum indicating the reason a response has been blocked. These reasons are
refinements of the net error BLOCKED_BY_RESPONSE.*/
/// <https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-BlockedByResponseReason>
pub enum AuditsBlockedByResponseReason {
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
/// <https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-BlockedByResponseIssueDetails>
pub struct AuditsBlockedByResponseIssueDetails {
    pub request: (),
    pub parent_frame: (),
    pub blocked_frame: (),
    pub reason: (),
}
/// <https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-HeavyAdResolutionStatus>
pub enum AuditsHeavyAdResolutionStatus {
    HeavyAdBlocked,
    HeavyAdWarning,
}
/// <https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-HeavyAdReason>
pub enum AuditsHeavyAdReason {
    NetworkTotalLimit,
    CpuTotalLimit,
    CpuPeakLimit,
}
/// <https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-HeavyAdIssueDetails>
pub struct AuditsHeavyAdIssueDetails {
    pub resolution: (),
    pub reason: (),
    pub frame: (),
}
/// <https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-ContentSecurityPolicyViolationType>
pub enum AuditsContentSecurityPolicyViolationType {
    KInlineViolation,
    KEvalViolation,
    KUrlViolation,
    KSriViolation,
    KTrustedTypesSinkViolation,
    KTrustedTypesPolicyViolation,
    KWasmEvalViolation,
}
/// <https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-SourceCodeLocation>
pub struct AuditsSourceCodeLocation {
    pub script_id: (),
    pub url: String,
    pub line_number: i64,
    pub column_number: i64,
}
/// <https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-ContentSecurityPolicyIssueDetails>
pub struct AuditsContentSecurityPolicyIssueDetails {
    pub blocked_url: String,
    pub violated_directive: String,
    pub is_report_only: (),
    pub content_security_policy_violation_type: (),
    pub frame_ancestor: (),
    pub source_code_location: (),
    pub violating_node_id: (),
}
/// <https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-SharedArrayBufferIssueType>
pub enum AuditsSharedArrayBufferIssueType {
    TransferIssue,
    CreationIssue,
}
/** Details for a issue arising from an SAB being instantiated in, or
transferred to a context that is not cross-origin isolated.*/
/// <https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-SharedArrayBufferIssueDetails>
pub struct AuditsSharedArrayBufferIssueDetails {
    pub source_code_location: (),
    pub is_warning: (),
    pub _type: (),
}
/// <https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-LowTextContrastIssueDetails>
pub struct AuditsLowTextContrastIssueDetails {
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
/// <https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-CorsIssueDetails>
pub struct AuditsCorsIssueDetails {
    pub cors_error_status: (),
    pub is_warning: (),
    pub request: (),
    pub location: (),
    pub initiator_origin: String,
    pub resource_ip_address_space: (),
    pub client_security_state: (),
}
/// <https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-AttributionReportingIssueType>
pub enum AuditsAttributionReportingIssueType {
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
/// <https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-SharedDictionaryError>
pub enum AuditsSharedDictionaryError {
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
/// <https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-SRIMessageSignatureError>
pub enum AuditsSriMessageSignatureError {
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
/// <https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-AttributionReportingIssueDetails>
pub struct AuditsAttributionReportingIssueDetails {
    pub violation_type: (),
    pub request: (),
    pub violating_node_id: (),
    pub invalid_parameter: String,
}
/** Details for issues about documents in Quirks Mode
or Limited Quirks Mode that affects page layouting.*/
/// <https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-QuirksModeIssueDetails>
pub struct AuditsQuirksModeIssueDetails {
    pub is_limited_quirks_mode: (),
    pub document_node_id: (),
    pub url: String,
    pub frame_id: (),
    pub loader_id: (),
}
#[deprecated]
/// <https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-NavigatorUserAgentIssueDetails>
pub struct AuditsNavigatorUserAgentIssueDetails {
    pub url: String,
    pub location: (),
}
/// <https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-SharedDictionaryIssueDetails>
pub struct AuditsSharedDictionaryIssueDetails {
    pub shared_dictionary_error: (),
    pub request: (),
}
/// <https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-SRIMessageSignatureIssueDetails>
pub struct AuditsSriMessageSignatureIssueDetails {
    pub error: (),
    pub signature_base: String,
    pub integrity_assertions: (),
    pub request: (),
}
/// <https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-GenericIssueErrorType>
pub enum AuditsGenericIssueErrorType {
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
/// <https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-GenericIssueDetails>
pub struct AuditsGenericIssueDetails {
    pub error_type: (),
    pub frame_id: (),
    pub violating_node_id: (),
    pub violating_node_attribute: String,
    pub request: (),
}
/** This issue tracks information needed to print a deprecation message.
https://source.chromium.org/chromium/chromium/src/+/main:third_party/blink/renderer/core/frame/third_party/blink/renderer/core/frame/deprecation/README.md*/
/// <https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-DeprecationIssueDetails>
pub struct AuditsDeprecationIssueDetails {
    pub affected_frame: (),
    pub source_code_location: (),
    pub _type: String,
}
/** This issue warns about sites in the redirect chain of a finished navigation
that may be flagged as trackers and have their state cleared if they don't
receive a user interaction. Note that in this context 'site' means eTLD+1.
For example, if the URL `https://example.test:80/bounce` was in the
redirect chain, the site reported would be `example.test`.*/
/// <https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-BounceTrackingIssueDetails>
pub struct AuditsBounceTrackingIssueDetails {
    pub tracking_sites: (),
}
/** This issue warns about third-party sites that are accessing cookies on the
current page, and have been permitted due to having a global metadata grant.
Note that in this context 'site' means eTLD+1. For example, if the URL
`https://example.test:80/web_page` was accessing cookies, the site reported
would be `example.test`.*/
/// <https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-CookieDeprecationMetadataIssueDetails>
pub struct AuditsCookieDeprecationMetadataIssueDetails {
    pub allowed_sites: (),
    pub opt_out_percentage: u64,
    pub is_opt_out_top_level: (),
    pub operation: (),
}
/// <https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-ClientHintIssueReason>
pub enum AuditsClientHintIssueReason {
    MetaTagAllowListInvalidOrigin,
    MetaTagModifiedHtml,
}
/// <https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-FederatedAuthRequestIssueDetails>
pub struct AuditsFederatedAuthRequestIssueDetails {
    pub federated_auth_request_issue_reason: (),
}
/** Represents the failure reason when a federated authentication reason fails.
Should be updated alongside RequestIdTokenStatus in
third_party/blink/public/mojom/devtools/inspector_issue.mojom to include
all cases except for success.*/
/// <https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-FederatedAuthRequestIssueReason>
pub enum AuditsFederatedAuthRequestIssueReason {
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
/// <https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-FederatedAuthUserInfoRequestIssueDetails>
pub struct AuditsFederatedAuthUserInfoRequestIssueDetails {
    pub federated_auth_user_info_request_issue_reason: (),
}
/** Represents the failure reason when a getUserInfo() call fails.
Should be updated alongside FederatedAuthUserInfoRequestResult in
third_party/blink/public/mojom/devtools/inspector_issue.mojom.*/
/// <https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-FederatedAuthUserInfoRequestIssueReason>
pub enum AuditsFederatedAuthUserInfoRequestIssueReason {
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
/// <https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-ClientHintIssueDetails>
pub struct AuditsClientHintIssueDetails {
    pub source_code_location: (),
    pub client_hint_issue_reason: (),
}
/// <https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-FailedRequestInfo>
pub struct AuditsFailedRequestInfo {
    pub url: String,
    pub failure_message: String,
    pub request_id: (),
}
/// <https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-PartitioningBlobURLInfo>
pub enum AuditsPartitioningBlobUrlInfo {
    BlockedCrossPartitionFetching,
    EnforceNoopenerForNavigation,
}
/// <https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-PartitioningBlobURLIssueDetails>
pub struct AuditsPartitioningBlobUrlIssueDetails {
    pub url: String,
    pub partitioning_blob_url_info: (),
}
/// <https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-SelectElementAccessibilityIssueReason>
pub enum AuditsSelectElementAccessibilityIssueReason {
    DisallowedSelectChild,
    DisallowedOptGroupChild,
    NonPhrasingContentOptionChild,
    InteractiveContentOptionChild,
    InteractiveContentLegendChild,
}
/// This issue warns about errors in the select element content model.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-SelectElementAccessibilityIssueDetails>
pub struct AuditsSelectElementAccessibilityIssueDetails {
    pub node_id: (),
    pub select_element_accessibility_issue_reason: (),
    pub has_disallowed_attributes: (),
}
/// <https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-StyleSheetLoadingIssueReason>
pub enum AuditsStyleSheetLoadingIssueReason {
    LateImportRule,
    RequestFailed,
}
/// This issue warns when a referenced stylesheet couldn't be loaded.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-StylesheetLoadingIssueDetails>
pub struct AuditsStylesheetLoadingIssueDetails {
    pub source_code_location: (),
    pub style_sheet_loading_issue_reason: (),
    pub failed_request_info: (),
}
/// <https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-PropertyRuleIssueReason>
pub enum AuditsPropertyRuleIssueReason {
    InvalidSyntax,
    InvalidInitialValue,
    InvalidInherits,
    InvalidName,
}
/** This issue warns about errors in property rules that lead to property
registrations being ignored.*/
/// <https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-PropertyRuleIssueDetails>
pub struct AuditsPropertyRuleIssueDetails {
    pub source_code_location: (),
    pub property_rule_issue_reason: (),
    pub property_value: String,
}
/// <https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-UserReidentificationIssueType>
pub enum AuditsUserReidentificationIssueType {
    BlockedFrameNavigation,
    BlockedSubresource,
}
/** This issue warns about uses of APIs that may be considered misuse to
re-identify users.*/
/// <https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-UserReidentificationIssueDetails>
pub struct AuditsUserReidentificationIssueDetails {
    pub _type: (),
    pub request: (),
}
/** A unique identifier for the type of issue. Each type may use one of the
optional fields in InspectorIssueDetails to convey more specific
information about the kind of issue.*/
/// <https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-InspectorIssueCode>
pub enum AuditsInspectorIssueCode {
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
/// <https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-InspectorIssueDetails>
pub struct AuditsInspectorIssueDetails {
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
/// <https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-IssueId>
pub struct AuditsIssueId(String);
/// An inspector issue reported from the back-end.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Audits/#type-InspectorIssue>
pub struct AuditsInspectorIssue {
    pub code: (),
    pub details: (),
    pub issue_id: (),
}
