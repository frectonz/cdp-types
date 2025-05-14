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
    pub pseudo_identifier: Box<String>,
    pub matches: (),
}
/// CSS style coming from animations with the name of the animation.
pub struct CssAnimationStyle {
    pub name: Box<String>,
    pub style: Box<CssStyle>,
}
/// Inherited CSS rule collection from ancestor node.
pub struct InheritedStyleEntry {
    pub inline_style: Box<CssStyle>,
    pub matched_css_rules: (),
}
/// Inherited CSS style collection for animated styles from ancestor node.
pub struct InheritedAnimatedStyleEntry {
    pub animation_styles: (),
    pub transitions_style: Box<CssStyle>,
}
/// Inherited pseudo element matches from pseudos of an ancestor node.
pub struct InheritedPseudoElementMatches {
    pub pseudo_elements: (),
}
/// Match data for a CSS rule.
pub struct RuleMatch {
    pub rule: Box<CssRule>,
    pub matching_selectors: (),
}
/// Data for a simple selector (these are delimited by commas in a selector list).
pub struct Value {
    pub text: Box<String>,
    pub range: Box<SourceRange>,
    pub specificity: Box<Specificity>,
}
/// ⚠️ Experimental
/** Specificity:
https://drafts.csswg.org/selectors/#specificity-rules*/
pub struct Specificity {
    pub a: Box<i64>,
    pub b: Box<i64>,
    pub c: Box<i64>,
}
/// Selector list data.
pub struct SelectorList {
    pub selectors: (),
    pub text: Box<String>,
}
/// CSS stylesheet metainformation.
pub struct CssStyleSheetHeader {
    pub style_sheet_id: Box<StyleSheetId>,
    pub frame_id: Box<FrameId>,
    pub source_url: Box<String>,
    pub source_map_url: Box<String>,
    pub origin: Box<StyleSheetOrigin>,
    pub title: Box<String>,
    pub owner_node: Box<BackendNodeId>,
    pub disabled: (),
    pub has_source_url: (),
    pub is_inline: (),
    pub is_mutable: (),
    pub is_constructed: (),
    pub start_line: Box<u64>,
    pub start_column: Box<u64>,
    pub length: Box<u64>,
    pub end_line: Box<u64>,
    pub end_column: Box<u64>,
    pub loading_failed: (),
}
/// CSS rule representation.
pub struct CssRule {
    pub style_sheet_id: Box<StyleSheetId>,
    pub selector_list: Box<SelectorList>,
    pub nesting_selectors: (),
    pub origin: Box<StyleSheetOrigin>,
    pub style: Box<CssStyle>,
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
    pub style_sheet_id: Box<StyleSheetId>,
    pub start_offset: Box<u64>,
    pub end_offset: Box<u64>,
    pub used: (),
}
/// Text range within a resource. All numbers are zero-based.
pub struct SourceRange {
    pub start_line: Box<i64>,
    pub start_column: Box<i64>,
    pub end_line: Box<i64>,
    pub end_column: Box<i64>,
}
pub struct ShorthandEntry {
    pub name: Box<String>,
    pub value: Box<String>,
    pub important: (),
}
/// CSS style representation.
pub struct CssStyle {
    pub style_sheet_id: Box<StyleSheetId>,
    pub css_properties: (),
    pub shorthand_entries: (),
    pub css_text: Box<String>,
    pub range: Box<SourceRange>,
}
/// CSS property declaration data.
pub struct CssProperty {
    pub name: Box<String>,
    pub value: Box<String>,
    pub important: (),
    pub implicit: (),
    pub text: Box<String>,
    pub parsed_ok: (),
    pub disabled: (),
    pub range: Box<SourceRange>,
    pub longhand_properties: (),
}
/// CSS media rule descriptor.
pub struct CssMedia {
    pub text: Box<String>,
    pub source: Box<String>,
    pub source_url: Box<String>,
    pub range: Box<SourceRange>,
    pub style_sheet_id: Box<StyleSheetId>,
    pub media_list: (),
}
/// Media query descriptor.
pub struct MediaQuery {
    pub expressions: (),
    pub active: (),
}
/// Media query expression descriptor.
pub struct MediaQueryExpression {
    pub value: Box<u64>,
    pub unit: Box<String>,
    pub feature: Box<String>,
    pub value_range: Box<SourceRange>,
    pub computed_length: Box<u64>,
}
/// ⚠️ Experimental
/// CSS container query rule descriptor.
pub struct CssContainerQuery {
    pub text: Box<String>,
    pub range: Box<SourceRange>,
    pub style_sheet_id: Box<StyleSheetId>,
    pub name: Box<String>,
    pub physical_axes: Box<PhysicalAxes>,
    pub logical_axes: Box<LogicalAxes>,
    pub queries_scroll_state: (),
}
/// ⚠️ Experimental
/// CSS Supports at-rule descriptor.
pub struct CssSupports {
    pub text: Box<String>,
    pub active: (),
    pub range: Box<SourceRange>,
    pub style_sheet_id: Box<StyleSheetId>,
}
/// ⚠️ Experimental
/// CSS Scope at-rule descriptor.
pub struct CssScope {
    pub text: Box<String>,
    pub range: Box<SourceRange>,
    pub style_sheet_id: Box<StyleSheetId>,
}
/// ⚠️ Experimental
/// CSS Layer at-rule descriptor.
pub struct CssLayer {
    pub text: Box<String>,
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
    pub name: Box<String>,
    pub sub_layers: (),
    pub order: Box<u64>,
}
/// Information about amount of glyphs that were rendered with given font.
pub struct PlatformFontUsage {
    pub family_name: Box<String>,
    pub post_script_name: Box<String>,
    pub is_custom_font: (),
    pub glyph_count: Box<u64>,
}
/// Information about font variation axes for variable fonts
pub struct FontVariationAxis {
    pub tag: Box<String>,
    pub name: Box<String>,
    pub min_value: Box<u64>,
    pub max_value: Box<u64>,
    pub default_value: Box<u64>,
}
/** Properties of a web font: https://www.w3.org/TR/2008/REC-CSS2-20080411/fonts.html#font-descriptions
and additional information such as platformFontFamily and fontVariationAxes.*/
pub struct FontFace {
    pub font_family: Box<String>,
    pub font_style: Box<String>,
    pub font_variant: Box<String>,
    pub font_weight: Box<String>,
    pub font_stretch: Box<String>,
    pub font_display: Box<String>,
    pub unicode_range: Box<String>,
    pub src: Box<String>,
    pub platform_font_family: Box<String>,
    pub font_variation_axes: (),
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
    pub active: (),
}
/// CSS keyframes rule representation.
pub struct CssKeyframesRule {
    pub animation_name: Box<Value>,
    pub keyframes: (),
}
/// Representation of a custom property registration through CSS.registerProperty
pub struct CssPropertyRegistration {
    pub property_name: Box<String>,
    pub initial_value: Box<Value>,
    pub inherits: (),
    pub syntax: Box<String>,
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
    pub name: Box<String>,
    pub _type: Box<String>,
}
/// CSS function conditional block representation.
pub struct CssFunctionConditionNode {
    pub media: Box<CssMedia>,
    pub container_queries: Box<CssContainerQuery>,
    pub supports: Box<CssSupports>,
    pub children: (),
    pub condition_text: Box<String>,
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
    pub parameters: (),
    pub children: (),
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
    pub text: Box<String>,
}
