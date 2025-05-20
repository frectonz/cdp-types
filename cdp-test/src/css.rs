use crate::common::*;
use crate::dom::*;
use crate::page::*;
pub struct StyleSheetId(String);
/** Stylesheet type: "injected" for stylesheets injected via extension, "user-agent" for user-agent
stylesheets, "inspector" for stylesheets created by the inspector (i.e. those holding the "via
inspector" rules), "regular" for regular stylesheets.*/
pub enum StyleSheetOrigin {
    Injected,
    UserAgent,
    Inspector,
    Regular,
}
/// CSS rule collection for a single pseudo style.
pub struct PseudoElementMatches {
    pub pseudo_type: Box<PseudoType>,
    pub pseudo_identifier: String,
    pub matches: Vec<RuleMatch>,
}
/// CSS style coming from animations with the name of the animation.
pub struct CssAnimationStyle {
    pub name: String,
    pub style: Box<CssStyle>,
}
/// Inherited CSS rule collection from ancestor node.
pub struct InheritedStyleEntry {
    pub inline_style: Box<CssStyle>,
    pub matched_css_rules: Vec<RuleMatch>,
}
/// Inherited CSS style collection for animated styles from ancestor node.
pub struct InheritedAnimatedStyleEntry {
    pub animation_styles: Vec<CssAnimationStyle>,
    pub transitions_style: Box<CssStyle>,
}
/// Inherited pseudo element matches from pseudos of an ancestor node.
pub struct InheritedPseudoElementMatches {
    pub pseudo_elements: Vec<PseudoElementMatches>,
}
/// Match data for a CSS rule.
pub struct RuleMatch {
    pub rule: Box<CssRule>,
    pub matching_selectors: Vec<i64>,
}
/// Data for a simple selector (these are delimited by commas in a selector list).
pub struct Value {
    pub text: String,
    pub range: Box<SourceRange>,
    pub specificity: Box<Specificity>,
}
/// ⚠️ Experimental
/** Specificity:
https://drafts.csswg.org/selectors/#specificity-rules*/
pub struct Specificity {
    pub a: i64,
    pub b: i64,
    pub c: i64,
}
/// Selector list data.
pub struct SelectorList {
    pub selectors: Vec<Value>,
    pub text: String,
}
/// CSS stylesheet metainformation.
pub struct CssStyleSheetHeader {
    pub style_sheet_id: Box<StyleSheetId>,
    pub frame_id: Box<crate::page::FrameId>,
    pub source_url: String,
    pub source_map_url: String,
    pub origin: Box<StyleSheetOrigin>,
    pub title: String,
    pub owner_node: Box<BackendNodeId>,
    pub disabled: bool,
    pub has_source_url: bool,
    pub is_inline: bool,
    pub is_mutable: bool,
    pub is_constructed: bool,
    pub start_line: u64,
    pub start_column: u64,
    pub length: u64,
    pub end_line: u64,
    pub end_column: u64,
    pub loading_failed: bool,
}
/// CSS rule representation.
pub struct CssRule {
    pub style_sheet_id: Box<StyleSheetId>,
    pub selector_list: Box<SelectorList>,
    pub nesting_selectors: Vec<String>,
    pub origin: Box<StyleSheetOrigin>,
    pub style: Box<CssStyle>,
    pub media: Vec<CssMedia>,
    pub container_queries: Vec<CssContainerQuery>,
    pub supports: Vec<CssSupports>,
    pub layers: Vec<CssLayer>,
    pub scopes: Vec<CssScope>,
    pub rule_types: Vec<CssRuleType>,
    pub starting_styles: Vec<CssStartingStyle>,
}
/// ⚠️ Experimental
/** Enum indicating the type of a CSS rule, used to represent the order of a style rule's ancestors.
This list only contains rule types that are collected during the ancestor rule collection.*/
pub enum CssRuleType {
    MediaRule,
    SupportsRule,
    ContainerRule,
    LayerRule,
    ScopeRule,
    StyleRule,
    StartingStyleRule,
}
/// CSS coverage information.
pub struct RuleUsage {
    pub style_sheet_id: Box<StyleSheetId>,
    pub start_offset: u64,
    pub end_offset: u64,
    pub used: bool,
}
/// Text range within a resource. All numbers are zero-based.
pub struct SourceRange {
    pub start_line: i64,
    pub start_column: i64,
    pub end_line: i64,
    pub end_column: i64,
}
pub struct ShorthandEntry {
    pub name: String,
    pub value: String,
    pub important: bool,
}
/// CSS style representation.
pub struct CssStyle {
    pub style_sheet_id: Box<StyleSheetId>,
    pub css_properties: Vec<CssProperty>,
    pub shorthand_entries: Vec<ShorthandEntry>,
    pub css_text: String,
    pub range: Box<SourceRange>,
}
/// CSS property declaration data.
pub struct CssProperty {
    pub name: String,
    pub value: String,
    pub important: bool,
    pub implicit: bool,
    pub text: String,
    pub parsed_ok: bool,
    pub disabled: bool,
    pub range: Box<SourceRange>,
    pub longhand_properties: Vec<CssProperty>,
}
/// CSS media rule descriptor.
pub struct CssMedia {
    pub text: String,
    pub source: String,
    pub source_url: String,
    pub range: Box<SourceRange>,
    pub style_sheet_id: Box<StyleSheetId>,
    pub media_list: Vec<MediaQuery>,
}
/// Media query descriptor.
pub struct MediaQuery {
    pub expressions: Vec<MediaQueryExpression>,
    pub active: bool,
}
/// Media query expression descriptor.
pub struct MediaQueryExpression {
    pub value: u64,
    pub unit: String,
    pub feature: String,
    pub value_range: Box<SourceRange>,
    pub computed_length: u64,
}
/// ⚠️ Experimental
/// CSS container query rule descriptor.
pub struct CssContainerQuery {
    pub text: String,
    pub range: Box<SourceRange>,
    pub style_sheet_id: Box<StyleSheetId>,
    pub name: String,
    pub physical_axes: Box<PhysicalAxes>,
    pub logical_axes: Box<LogicalAxes>,
    pub queries_scroll_state: bool,
}
/// ⚠️ Experimental
/// CSS Supports at-rule descriptor.
pub struct CssSupports {
    pub text: String,
    pub active: bool,
    pub range: Box<SourceRange>,
    pub style_sheet_id: Box<StyleSheetId>,
}
/// ⚠️ Experimental
/// CSS Scope at-rule descriptor.
pub struct CssScope {
    pub text: String,
    pub range: Box<SourceRange>,
    pub style_sheet_id: Box<StyleSheetId>,
}
/// ⚠️ Experimental
/// CSS Layer at-rule descriptor.
pub struct CssLayer {
    pub text: String,
    pub range: Box<SourceRange>,
    pub style_sheet_id: Box<StyleSheetId>,
}
/// ⚠️ Experimental
/// CSS Starting Style at-rule descriptor.
pub struct CssStartingStyle {
    pub range: Box<SourceRange>,
    pub style_sheet_id: Box<StyleSheetId>,
}
/// ⚠️ Experimental
/// CSS Layer data.
pub struct CssLayerData {
    pub name: String,
    pub sub_layers: Vec<CssLayerData>,
    pub order: u64,
}
/// Information about amount of glyphs that were rendered with given font.
pub struct PlatformFontUsage {
    pub family_name: String,
    pub post_script_name: String,
    pub is_custom_font: bool,
    pub glyph_count: u64,
}
/// Information about font variation axes for variable fonts
pub struct FontVariationAxis {
    pub tag: String,
    pub name: String,
    pub min_value: u64,
    pub max_value: u64,
    pub default_value: u64,
}
/** Properties of a web font: https://www.w3.org/TR/2008/REC-CSS2-20080411/fonts.html#font-descriptions
and additional information such as platformFontFamily and fontVariationAxes.*/
pub struct FontFace {
    pub font_family: String,
    pub font_style: String,
    pub font_variant: String,
    pub font_weight: String,
    pub font_stretch: String,
    pub font_display: String,
    pub unicode_range: String,
    pub src: String,
    pub platform_font_family: String,
    pub font_variation_axes: Vec<FontVariationAxis>,
}
/// CSS try rule representation.
pub struct CssTryRule {
    pub style_sheet_id: Box<StyleSheetId>,
    pub origin: Box<StyleSheetOrigin>,
    pub style: Box<CssStyle>,
}
/// CSS @position-try rule representation.
pub struct CssPositionTryRule {
    pub name: Box<Value>,
    pub style_sheet_id: Box<StyleSheetId>,
    pub origin: Box<StyleSheetOrigin>,
    pub style: Box<CssStyle>,
    pub active: bool,
}
/// CSS keyframes rule representation.
pub struct CssKeyframesRule {
    pub animation_name: Box<Value>,
    pub keyframes: Vec<CssKeyframeRule>,
}
/// Representation of a custom property registration through CSS.registerProperty
pub struct CssPropertyRegistration {
    pub property_name: String,
    pub initial_value: Box<Value>,
    pub inherits: bool,
    pub syntax: String,
}
/// CSS font-palette-values rule representation.
pub struct CssFontPaletteValuesRule {
    pub style_sheet_id: Box<StyleSheetId>,
    pub origin: Box<StyleSheetOrigin>,
    pub font_palette_name: Box<Value>,
    pub style: Box<CssStyle>,
}
/// CSS property at-rule representation.
pub struct CssPropertyRule {
    pub style_sheet_id: Box<StyleSheetId>,
    pub origin: Box<StyleSheetOrigin>,
    pub property_name: Box<Value>,
    pub style: Box<CssStyle>,
}
/// CSS function argument representation.
pub struct CssFunctionParameter {
    pub name: String,
    pub _type: String,
}
/// CSS function conditional block representation.
pub struct CssFunctionConditionNode {
    pub media: Box<CssMedia>,
    pub container_queries: Box<CssContainerQuery>,
    pub supports: Box<CssSupports>,
    pub children: Vec<CssFunctionNode>,
    pub condition_text: String,
}
/// Section of the body of a CSS function rule.
pub struct CssFunctionNode {
    pub condition: Box<CssFunctionConditionNode>,
    pub style: Box<CssStyle>,
}
/// CSS function at-rule representation.
pub struct CssFunctionRule {
    pub name: Box<Value>,
    pub style_sheet_id: Box<StyleSheetId>,
    pub origin: Box<StyleSheetOrigin>,
    pub parameters: Vec<CssFunctionParameter>,
    pub children: Vec<CssFunctionNode>,
}
/// CSS keyframe rule representation.
pub struct CssKeyframeRule {
    pub style_sheet_id: Box<StyleSheetId>,
    pub origin: Box<StyleSheetOrigin>,
    pub key_text: Box<Value>,
    pub style: Box<CssStyle>,
}
/// A descriptor of operation to mutate style declaration text.
pub struct StyleDeclarationEdit {
    pub style_sheet_id: Box<StyleSheetId>,
    pub range: Box<SourceRange>,
    pub text: String,
}
pub type CssAddRule = ();
pub type CssCollectClassNames = ();
pub type CssCreateStyleSheet = ();
pub type CssDisable = ();
pub type CssEnable = ();
pub type CssForcePseudoState = ();
pub type CssForceStartingStyle = ();
pub type CssGetBackgroundColors = ();
pub type CssGetComputedStyleForNode = ();
pub type CssResolveValues = ();
pub type CssGetLonghandProperties = ();
pub type CssGetInlineStylesForNode = ();
pub type CssGetAnimatedStylesForNode = ();
pub type CssGetMatchedStylesForNode = ();
pub type CssGetMediaQueries = ();
pub type CssGetPlatformFontsForNode = ();
pub type CssGetStyleSheetText = ();
pub type CssGetLayersForNode = ();
pub type CssGetLocationForSelector = ();
pub type CssTrackComputedStyleUpdatesForNode = ();
pub type CssTrackComputedStyleUpdates = ();
pub type CssTakeComputedStyleUpdates = ();
pub type CssSetEffectivePropertyValueForNode = ();
pub type CssSetPropertyRulePropertyName = ();
pub type CssSetKeyframeKey = ();
pub type CssSetMediaText = ();
pub type CssSetContainerQueryText = ();
pub type CssSetSupportsText = ();
pub type CssSetScopeText = ();
pub type CssSetRuleSelector = ();
pub type CssSetStyleSheetText = ();
pub type CssSetStyleTexts = ();
pub type CssStartRuleUsageTracking = ();
pub type CssStopRuleUsageTracking = ();
pub type CssTakeCoverageDelta = ();
pub type CssSetLocalFontsEnabled = ();
