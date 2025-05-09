use crate::dom::*;
use crate::page::*;
/// <https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-StyleSheetId>
pub struct CssStyleSheetId(String);
/** Stylesheet type: "injected" for stylesheets injected via extension, "user-agent" for user-agent
stylesheets, "inspector" for stylesheets created by the inspector (i.e. those holding the "via
inspector" rules), "regular" for regular stylesheets.*/
/// <https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-StyleSheetOrigin>
pub enum CssStyleSheetOrigin {
    Injected,
    UserAgent,
    Inspector,
    Regular,
}
/// CSS rule collection for a single pseudo style.
/// <https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-PseudoElementMatches>
pub struct CssPseudoElementMatches {
    pub pseudo_type: (),
    pub pseudo_identifier: String,
    pub matches: (),
}
/// CSS style coming from animations with the name of the animation.
/// <https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-CSSAnimationStyle>
pub struct CssAnimationStyle {
    pub name: String,
    pub style: (),
}
/// Inherited CSS rule collection from ancestor node.
/// <https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-InheritedStyleEntry>
pub struct CssInheritedStyleEntry {
    pub inline_style: (),
    pub matched_css_rules: (),
}
/// Inherited CSS style collection for animated styles from ancestor node.
/// <https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-InheritedAnimatedStyleEntry>
pub struct CssInheritedAnimatedStyleEntry {
    pub animation_styles: (),
    pub transitions_style: (),
}
/// Inherited pseudo element matches from pseudos of an ancestor node.
/// <https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-InheritedPseudoElementMatches>
pub struct CssInheritedPseudoElementMatches {
    pub pseudo_elements: (),
}
/// Match data for a CSS rule.
/// <https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-RuleMatch>
pub struct CssRuleMatch {
    pub rule: (),
    pub matching_selectors: (),
}
/// Data for a simple selector (these are delimited by commas in a selector list).
/// <https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-Value>
pub struct CssValue {
    pub text: String,
    pub range: (),
    pub specificity: (),
}
/// ⚠️ Experimental
/** Specificity:
https://drafts.csswg.org/selectors/#specificity-rules*/
/// <https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-Specificity>
pub struct CssSpecificity {
    pub a: i64,
    pub b: i64,
    pub c: i64,
}
/// Selector list data.
/// <https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-SelectorList>
pub struct CssSelectorList {
    pub selectors: (),
    pub text: String,
}
/// CSS stylesheet metainformation.
/// <https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-CSSStyleSheetHeader>
pub struct CssStyleSheetHeader {
    pub style_sheet_id: (),
    pub frame_id: (),
    pub source_url: String,
    pub source_map_url: String,
    pub origin: (),
    pub title: String,
    pub owner_node: (),
    pub disabled: (),
    pub has_source_url: (),
    pub is_inline: (),
    pub is_mutable: (),
    pub is_constructed: (),
    pub start_line: u64,
    pub start_column: u64,
    pub length: u64,
    pub end_line: u64,
    pub end_column: u64,
    pub loading_failed: (),
}
/// CSS rule representation.
/// <https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-CSSRule>
pub struct CssRule {
    pub style_sheet_id: (),
    pub selector_list: (),
    pub nesting_selectors: (),
    pub origin: (),
    pub style: (),
    pub media: (),
    pub container_queries: (),
    pub supports: (),
    pub layers: (),
    pub scopes: (),
    pub rule_types: (),
    pub starting_styles: (),
}
/// ⚠️ Experimental
/** Enum indicating the type of a CSS rule, used to represent the order of a style rule's ancestors.
This list only contains rule types that are collected during the ancestor rule collection.*/
/// <https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-CSSRuleType>
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
/// <https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-RuleUsage>
pub struct CssRuleUsage {
    pub style_sheet_id: (),
    pub start_offset: u64,
    pub end_offset: u64,
    pub used: (),
}
/// Text range within a resource. All numbers are zero-based.
/// <https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-SourceRange>
pub struct CssSourceRange {
    pub start_line: i64,
    pub start_column: i64,
    pub end_line: i64,
    pub end_column: i64,
}
/// <https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-ShorthandEntry>
pub struct CssShorthandEntry {
    pub name: String,
    pub value: String,
    pub important: (),
}
/// <https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-CSSComputedStyleProperty>
pub struct CssComputedStyleProperty {
    pub name: String,
    pub value: String,
}
/// CSS style representation.
/// <https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-CSSStyle>
pub struct CssStyle {
    pub style_sheet_id: (),
    pub css_properties: (),
    pub shorthand_entries: (),
    pub css_text: String,
    pub range: (),
}
/// CSS property declaration data.
/// <https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-CSSProperty>
pub struct CssProperty {
    pub name: String,
    pub value: String,
    pub important: (),
    pub implicit: (),
    pub text: String,
    pub parsed_ok: (),
    pub disabled: (),
    pub range: (),
    pub longhand_properties: (),
}
/// CSS media rule descriptor.
/// <https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-CSSMedia>
pub struct CssMedia {
    pub text: String,
    pub source: String,
    pub source_url: String,
    pub range: (),
    pub style_sheet_id: (),
    pub media_list: (),
}
/// Media query descriptor.
/// <https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-MediaQuery>
pub struct CssMediaQuery {
    pub expressions: (),
    pub active: (),
}
/// Media query expression descriptor.
/// <https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-MediaQueryExpression>
pub struct CssMediaQueryExpression {
    pub value: u64,
    pub unit: String,
    pub feature: String,
    pub value_range: (),
    pub computed_length: u64,
}
/// ⚠️ Experimental
/// CSS container query rule descriptor.
/// <https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-CSSContainerQuery>
pub struct CssContainerQuery {
    pub text: String,
    pub range: (),
    pub style_sheet_id: (),
    pub name: String,
    pub physical_axes: (),
    pub logical_axes: (),
    pub queries_scroll_state: (),
}
/// ⚠️ Experimental
/// CSS Supports at-rule descriptor.
/// <https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-CSSSupports>
pub struct CssSupports {
    pub text: String,
    pub active: (),
    pub range: (),
    pub style_sheet_id: (),
}
/// ⚠️ Experimental
/// CSS Scope at-rule descriptor.
/// <https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-CSSScope>
pub struct CssScope {
    pub text: String,
    pub range: (),
    pub style_sheet_id: (),
}
/// ⚠️ Experimental
/// CSS Layer at-rule descriptor.
/// <https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-CSSLayer>
pub struct CssLayer {
    pub text: String,
    pub range: (),
    pub style_sheet_id: (),
}
/// ⚠️ Experimental
/// CSS Starting Style at-rule descriptor.
/// <https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-CSSStartingStyle>
pub struct CssStartingStyle {
    pub range: (),
    pub style_sheet_id: (),
}
/// ⚠️ Experimental
/// CSS Layer data.
/// <https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-CSSLayerData>
pub struct CssLayerData {
    pub name: String,
    pub sub_layers: (),
    pub order: u64,
}
/// Information about amount of glyphs that were rendered with given font.
/// <https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-PlatformFontUsage>
pub struct CssPlatformFontUsage {
    pub family_name: String,
    pub post_script_name: String,
    pub is_custom_font: (),
    pub glyph_count: u64,
}
/// Information about font variation axes for variable fonts
/// <https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-FontVariationAxis>
pub struct CssFontVariationAxis {
    pub tag: String,
    pub name: String,
    pub min_value: u64,
    pub max_value: u64,
    pub default_value: u64,
}
/** Properties of a web font: https://www.w3.org/TR/2008/REC-CSS2-20080411/fonts.html#font-descriptions
and additional information such as platformFontFamily and fontVariationAxes.*/
/// <https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-FontFace>
pub struct CssFontFace {
    pub font_family: String,
    pub font_style: String,
    pub font_variant: String,
    pub font_weight: String,
    pub font_stretch: String,
    pub font_display: String,
    pub unicode_range: String,
    pub src: String,
    pub platform_font_family: String,
    pub font_variation_axes: (),
}
/// CSS try rule representation.
/// <https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-CSSTryRule>
pub struct CssTryRule {
    pub style_sheet_id: (),
    pub origin: (),
    pub style: (),
}
/// CSS @position-try rule representation.
/// <https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-CSSPositionTryRule>
pub struct CssPositionTryRule {
    pub name: (),
    pub style_sheet_id: (),
    pub origin: (),
    pub style: (),
    pub active: (),
}
/// CSS keyframes rule representation.
/// <https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-CSSKeyframesRule>
pub struct CssKeyframesRule {
    pub animation_name: (),
    pub keyframes: (),
}
/// Representation of a custom property registration through CSS.registerProperty
/// <https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-CSSPropertyRegistration>
pub struct CssPropertyRegistration {
    pub property_name: String,
    pub initial_value: (),
    pub inherits: (),
    pub syntax: String,
}
/// CSS font-palette-values rule representation.
/// <https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-CSSFontPaletteValuesRule>
pub struct CssFontPaletteValuesRule {
    pub style_sheet_id: (),
    pub origin: (),
    pub font_palette_name: (),
    pub style: (),
}
/// CSS property at-rule representation.
/// <https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-CSSPropertyRule>
pub struct CssPropertyRule {
    pub style_sheet_id: (),
    pub origin: (),
    pub property_name: (),
    pub style: (),
}
/// CSS function argument representation.
/// <https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-CSSFunctionParameter>
pub struct CssFunctionParameter {
    pub name: String,
    pub _type: String,
}
/// CSS function conditional block representation.
/// <https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-CSSFunctionConditionNode>
pub struct CssFunctionConditionNode {
    pub media: (),
    pub container_queries: (),
    pub supports: (),
    pub children: (),
    pub condition_text: String,
}
/// Section of the body of a CSS function rule.
/// <https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-CSSFunctionNode>
pub struct CssFunctionNode {
    pub condition: (),
    pub style: (),
}
/// CSS function at-rule representation.
/// <https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-CSSFunctionRule>
pub struct CssFunctionRule {
    pub name: (),
    pub style_sheet_id: (),
    pub origin: (),
    pub parameters: (),
    pub children: (),
}
/// CSS keyframe rule representation.
/// <https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-CSSKeyframeRule>
pub struct CssKeyframeRule {
    pub style_sheet_id: (),
    pub origin: (),
    pub key_text: (),
    pub style: (),
}
/// A descriptor of operation to mutate style declaration text.
/// <https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-StyleDeclarationEdit>
pub struct CssStyleDeclarationEdit {
    pub style_sheet_id: (),
    pub range: (),
    pub text: String,
}
