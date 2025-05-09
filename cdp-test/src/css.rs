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
    pub pseudo_identifier: (),
    pub matches: (),
}
/// CSS style coming from animations with the name of the animation.
/// <https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-CSSAnimationStyle>
pub struct CssAnimationStyle {
    pub name: (),
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
    pub text: (),
    pub range: (),
    pub specificity: (),
}
/// ⚠️ Experimental
/** Specificity:
https://drafts.csswg.org/selectors/#specificity-rules*/
/// <https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-Specificity>
pub struct CssSpecificity {
    pub a: (),
    pub b: (),
    pub c: (),
}
/// Selector list data.
/// <https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-SelectorList>
pub struct CssSelectorList {
    pub selectors: (),
    pub text: (),
}
/// CSS stylesheet metainformation.
/// <https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-CSSStyleSheetHeader>
pub struct CssStyleSheetHeader {
    pub style_sheet_id: (),
    pub frame_id: (),
    pub source_url: (),
    pub source_map_url: (),
    pub origin: (),
    pub title: (),
    pub owner_node: (),
    pub disabled: (),
    pub has_source_url: (),
    pub is_inline: (),
    pub is_mutable: (),
    pub is_constructed: (),
    pub start_line: (),
    pub start_column: (),
    pub length: (),
    pub end_line: (),
    pub end_column: (),
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
    pub start_offset: (),
    pub end_offset: (),
    pub used: (),
}
/// Text range within a resource. All numbers are zero-based.
/// <https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-SourceRange>
pub struct CssSourceRange {
    pub start_line: (),
    pub start_column: (),
    pub end_line: (),
    pub end_column: (),
}
/// <https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-ShorthandEntry>
pub struct CssShorthandEntry {
    pub name: (),
    pub value: (),
    pub important: (),
}
/// <https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-CSSComputedStyleProperty>
pub struct CssComputedStyleProperty {
    pub name: (),
    pub value: (),
}
/// CSS style representation.
/// <https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-CSSStyle>
pub struct CssStyle {
    pub style_sheet_id: (),
    pub css_properties: (),
    pub shorthand_entries: (),
    pub css_text: (),
    pub range: (),
}
/// CSS property declaration data.
/// <https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-CSSProperty>
pub struct CssProperty {
    pub name: (),
    pub value: (),
    pub important: (),
    pub implicit: (),
    pub text: (),
    pub parsed_ok: (),
    pub disabled: (),
    pub range: (),
    pub longhand_properties: (),
}
/// CSS media rule descriptor.
/// <https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-CSSMedia>
pub struct CssMedia {
    pub text: (),
    pub source: (),
    pub source_url: (),
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
    pub value: (),
    pub unit: (),
    pub feature: (),
    pub value_range: (),
    pub computed_length: (),
}
/// ⚠️ Experimental
/// CSS container query rule descriptor.
/// <https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-CSSContainerQuery>
pub struct CssContainerQuery {
    pub text: (),
    pub range: (),
    pub style_sheet_id: (),
    pub name: (),
    pub physical_axes: (),
    pub logical_axes: (),
    pub queries_scroll_state: (),
}
/// ⚠️ Experimental
/// CSS Supports at-rule descriptor.
/// <https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-CSSSupports>
pub struct CssSupports {
    pub text: (),
    pub active: (),
    pub range: (),
    pub style_sheet_id: (),
}
/// ⚠️ Experimental
/// CSS Scope at-rule descriptor.
/// <https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-CSSScope>
pub struct CssScope {
    pub text: (),
    pub range: (),
    pub style_sheet_id: (),
}
/// ⚠️ Experimental
/// CSS Layer at-rule descriptor.
/// <https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-CSSLayer>
pub struct CssLayer {
    pub text: (),
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
    pub name: (),
    pub sub_layers: (),
    pub order: (),
}
/// Information about amount of glyphs that were rendered with given font.
/// <https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-PlatformFontUsage>
pub struct CssPlatformFontUsage {
    pub family_name: (),
    pub post_script_name: (),
    pub is_custom_font: (),
    pub glyph_count: (),
}
/// Information about font variation axes for variable fonts
/// <https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-FontVariationAxis>
pub struct CssFontVariationAxis {
    pub tag: (),
    pub name: (),
    pub min_value: (),
    pub max_value: (),
    pub default_value: (),
}
/** Properties of a web font: https://www.w3.org/TR/2008/REC-CSS2-20080411/fonts.html#font-descriptions
and additional information such as platformFontFamily and fontVariationAxes.*/
/// <https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-FontFace>
pub struct CssFontFace {
    pub font_family: (),
    pub font_style: (),
    pub font_variant: (),
    pub font_weight: (),
    pub font_stretch: (),
    pub font_display: (),
    pub unicode_range: (),
    pub src: (),
    pub platform_font_family: (),
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
    pub property_name: (),
    pub initial_value: (),
    pub inherits: (),
    pub syntax: (),
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
    pub name: (),
    pub _type: (),
}
/// CSS function conditional block representation.
/// <https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-CSSFunctionConditionNode>
pub struct CssFunctionConditionNode {
    pub media: (),
    pub container_queries: (),
    pub supports: (),
    pub children: (),
    pub condition_text: (),
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
    pub text: (),
}
