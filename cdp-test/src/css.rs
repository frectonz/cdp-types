pub use crate::common::*;
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
    pub pseudo_type: (),
    pub pseudo_identifier: String,
    pub matches: (),
}
/// CSS style coming from animations with the name of the animation.
pub struct CssAnimationStyle {
    pub name: String,
    pub style: (),
}
/// Inherited CSS rule collection from ancestor node.
pub struct InheritedStyleEntry {
    pub inline_style: (),
    pub matched_css_rules: (),
}
/// Inherited CSS style collection for animated styles from ancestor node.
pub struct InheritedAnimatedStyleEntry {
    pub animation_styles: (),
    pub transitions_style: (),
}
/// Inherited pseudo element matches from pseudos of an ancestor node.
pub struct InheritedPseudoElementMatches {
    pub pseudo_elements: (),
}
/// Match data for a CSS rule.
pub struct RuleMatch {
    pub rule: (),
    pub matching_selectors: (),
}
/// Data for a simple selector (these are delimited by commas in a selector list).
pub struct Value {
    pub text: String,
    pub range: (),
    pub specificity: (),
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
    pub selectors: (),
    pub text: String,
}
/// CSS stylesheet metainformation.
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
    pub style_sheet_id: (),
    pub start_offset: u64,
    pub end_offset: u64,
    pub used: (),
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
    pub important: (),
}
/// CSS style representation.
pub struct CssStyle {
    pub style_sheet_id: (),
    pub css_properties: (),
    pub shorthand_entries: (),
    pub css_text: String,
    pub range: (),
}
/// CSS property declaration data.
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
pub struct CssMedia {
    pub text: String,
    pub source: String,
    pub source_url: String,
    pub range: (),
    pub style_sheet_id: (),
    pub media_list: (),
}
/// Media query descriptor.
pub struct MediaQuery {
    pub expressions: (),
    pub active: (),
}
/// Media query expression descriptor.
pub struct MediaQueryExpression {
    pub value: u64,
    pub unit: String,
    pub feature: String,
    pub value_range: (),
    pub computed_length: u64,
}
/// ⚠️ Experimental
/// CSS container query rule descriptor.
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
pub struct CssSupports {
    pub text: String,
    pub active: (),
    pub range: (),
    pub style_sheet_id: (),
}
/// ⚠️ Experimental
/// CSS Scope at-rule descriptor.
pub struct CssScope {
    pub text: String,
    pub range: (),
    pub style_sheet_id: (),
}
/// ⚠️ Experimental
/// CSS Layer at-rule descriptor.
pub struct CssLayer {
    pub text: String,
    pub range: (),
    pub style_sheet_id: (),
}
/// ⚠️ Experimental
/// CSS Starting Style at-rule descriptor.
pub struct CssStartingStyle {
    pub range: (),
    pub style_sheet_id: (),
}
/// ⚠️ Experimental
/// CSS Layer data.
pub struct CssLayerData {
    pub name: String,
    pub sub_layers: (),
    pub order: u64,
}
/// Information about amount of glyphs that were rendered with given font.
pub struct PlatformFontUsage {
    pub family_name: String,
    pub post_script_name: String,
    pub is_custom_font: (),
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
    pub font_variation_axes: (),
}
/// CSS try rule representation.
pub struct CssTryRule {
    pub style_sheet_id: (),
    pub origin: (),
    pub style: (),
}
/// CSS @position-try rule representation.
pub struct CssPositionTryRule {
    pub name: (),
    pub style_sheet_id: (),
    pub origin: (),
    pub style: (),
    pub active: (),
}
/// CSS keyframes rule representation.
pub struct CssKeyframesRule {
    pub animation_name: (),
    pub keyframes: (),
}
/// Representation of a custom property registration through CSS.registerProperty
pub struct CssPropertyRegistration {
    pub property_name: String,
    pub initial_value: (),
    pub inherits: (),
    pub syntax: String,
}
/// CSS font-palette-values rule representation.
pub struct CssFontPaletteValuesRule {
    pub style_sheet_id: (),
    pub origin: (),
    pub font_palette_name: (),
    pub style: (),
}
/// CSS property at-rule representation.
pub struct CssPropertyRule {
    pub style_sheet_id: (),
    pub origin: (),
    pub property_name: (),
    pub style: (),
}
/// CSS function argument representation.
pub struct CssFunctionParameter {
    pub name: String,
    pub _type: String,
}
/// CSS function conditional block representation.
pub struct CssFunctionConditionNode {
    pub media: (),
    pub container_queries: (),
    pub supports: (),
    pub children: (),
    pub condition_text: String,
}
/// Section of the body of a CSS function rule.
pub struct CssFunctionNode {
    pub condition: (),
    pub style: (),
}
/// CSS function at-rule representation.
pub struct CssFunctionRule {
    pub name: (),
    pub style_sheet_id: (),
    pub origin: (),
    pub parameters: (),
    pub children: (),
}
/// CSS keyframe rule representation.
pub struct CssKeyframeRule {
    pub style_sheet_id: (),
    pub origin: (),
    pub key_text: (),
    pub style: (),
}
/// A descriptor of operation to mutate style declaration text.
pub struct StyleDeclarationEdit {
    pub style_sheet_id: (),
    pub range: (),
    pub text: String,
}
