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
/** Inserts a new rule with the given `ruleText` in a stylesheet with given `styleSheetId`, at the
position specified by `location`.*/
pub struct CssAddRuleParams {
    pub style_sheet_id: Box<StyleSheetId>,
    pub rule_text: String,
    pub location: Box<SourceRange>,
    pub node_for_property_syntax_validation: Box<NodeId>,
}
/** Inserts a new rule with the given `ruleText` in a stylesheet with given `styleSheetId`, at the
position specified by `location`.*/
pub type CssAddRuleReturns = ();
/// Returns all class names from specified stylesheet.
pub struct CssCollectClassNamesParams {
    pub style_sheet_id: Box<StyleSheetId>,
}
/// Returns all class names from specified stylesheet.
pub type CssCollectClassNamesReturns = ();
/// Creates a new special "via-inspector" stylesheet in the frame with given `frameId`.
pub struct CssCreateStyleSheetParams {
    pub frame_id: Box<crate::page::FrameId>,
    pub force: bool,
}
/// Creates a new special "via-inspector" stylesheet in the frame with given `frameId`.
pub type CssCreateStyleSheetReturns = ();
/// Disables the CSS agent for the given page.
pub type CssDisableParams = ();
/// Disables the CSS agent for the given page.
pub type CssDisableReturns = ();
/** Enables the CSS agent for the given page. Clients should not assume that the CSS agent has been
enabled until the result of this command is received.*/
pub type CssEnableParams = ();
/** Enables the CSS agent for the given page. Clients should not assume that the CSS agent has been
enabled until the result of this command is received.*/
pub type CssEnableReturns = ();
/** Ensures that the given node will have specified pseudo-classes whenever its style is computed by
the browser.*/
pub struct CssForcePseudoStateParams {
    pub node_id: Box<NodeId>,
    pub forced_pseudo_classes: Vec<String>,
}
/** Ensures that the given node will have specified pseudo-classes whenever its style is computed by
the browser.*/
pub type CssForcePseudoStateReturns = ();
/// Ensures that the given node is in its starting-style state.
pub struct CssForceStartingStyleParams {
    pub node_id: Box<NodeId>,
    pub forced: bool,
}
/// Ensures that the given node is in its starting-style state.
pub type CssForceStartingStyleReturns = ();
pub struct CssGetBackgroundColorsParams {
    pub node_id: Box<NodeId>,
}
pub type CssGetBackgroundColorsReturns = ();
/// Returns the computed style for a DOM node identified by `nodeId`.
pub struct CssGetComputedStyleForNodeParams {
    pub node_id: Box<NodeId>,
}
/// Returns the computed style for a DOM node identified by `nodeId`.
pub type CssGetComputedStyleForNodeReturns = ();
/// ⚠️ Experimental
/** Resolve the specified values in the context of the provided element.
For example, a value of '1em' is evaluated according to the computed
'font-size' of the element and a value 'calc(1px + 2px)' will be
resolved to '3px'.
If the `propertyName` was specified the `values` are resolved as if
they were property's declaration. If a value cannot be parsed according
to the provided property syntax, the value is parsed using combined
syntax as if null `propertyName` was provided. If the value cannot be
resolved even then, return the provided value without any changes.*/
pub struct CssResolveValuesParams {
    pub values: Vec<String>,
    pub node_id: Box<NodeId>,
    pub property_name: String,
    pub pseudo_type: Box<PseudoType>,
    pub pseudo_identifier: String,
}
/// ⚠️ Experimental
/** Resolve the specified values in the context of the provided element.
For example, a value of '1em' is evaluated according to the computed
'font-size' of the element and a value 'calc(1px + 2px)' will be
resolved to '3px'.
If the `propertyName` was specified the `values` are resolved as if
they were property's declaration. If a value cannot be parsed according
to the provided property syntax, the value is parsed using combined
syntax as if null `propertyName` was provided. If the value cannot be
resolved even then, return the provided value without any changes.*/
pub type CssResolveValuesReturns = ();
/// ⚠️ Experimental
pub struct CssGetLonghandPropertiesParams {
    pub shorthand_name: String,
    pub value: String,
}
/// ⚠️ Experimental
pub type CssGetLonghandPropertiesReturns = ();
/** Returns the styles defined inline (explicitly in the "style" attribute and implicitly, using DOM
attributes) for a DOM node identified by `nodeId`.*/
pub struct CssGetInlineStylesForNodeParams {
    pub node_id: Box<NodeId>,
}
/** Returns the styles defined inline (explicitly in the "style" attribute and implicitly, using DOM
attributes) for a DOM node identified by `nodeId`.*/
pub type CssGetInlineStylesForNodeReturns = ();
/// ⚠️ Experimental
/** Returns the styles coming from animations & transitions
including the animation & transition styles coming from inheritance chain.*/
pub struct CssGetAnimatedStylesForNodeParams {
    pub node_id: Box<NodeId>,
}
/// ⚠️ Experimental
/** Returns the styles coming from animations & transitions
including the animation & transition styles coming from inheritance chain.*/
pub type CssGetAnimatedStylesForNodeReturns = ();
/// Returns requested styles for a DOM node identified by `nodeId`.
pub struct CssGetMatchedStylesForNodeParams {
    pub node_id: Box<NodeId>,
}
/// Returns requested styles for a DOM node identified by `nodeId`.
pub type CssGetMatchedStylesForNodeReturns = ();
/// Returns all media queries parsed by the rendering engine.
pub type CssGetMediaQueriesParams = ();
/// Returns all media queries parsed by the rendering engine.
pub type CssGetMediaQueriesReturns = ();
/** Requests information about platform fonts which we used to render child TextNodes in the given
node.*/
pub struct CssGetPlatformFontsForNodeParams {
    pub node_id: Box<NodeId>,
}
/** Requests information about platform fonts which we used to render child TextNodes in the given
node.*/
pub type CssGetPlatformFontsForNodeReturns = ();
/// Returns the current textual content for a stylesheet.
pub struct CssGetStyleSheetTextParams {
    pub style_sheet_id: Box<StyleSheetId>,
}
/// Returns the current textual content for a stylesheet.
pub type CssGetStyleSheetTextReturns = ();
/// ⚠️ Experimental
/** Returns all layers parsed by the rendering engine for the tree scope of a node.
Given a DOM element identified by nodeId, getLayersForNode returns the root
layer for the nearest ancestor document or shadow root. The layer root contains
the full layer tree for the tree scope and their ordering.*/
pub struct CssGetLayersForNodeParams {
    pub node_id: Box<NodeId>,
}
/// ⚠️ Experimental
/** Returns all layers parsed by the rendering engine for the tree scope of a node.
Given a DOM element identified by nodeId, getLayersForNode returns the root
layer for the nearest ancestor document or shadow root. The layer root contains
the full layer tree for the tree scope and their ordering.*/
pub type CssGetLayersForNodeReturns = ();
/// ⚠️ Experimental
/** Given a CSS selector text and a style sheet ID, getLocationForSelector
returns an array of locations of the CSS selector in the style sheet.*/
pub struct CssGetLocationForSelectorParams {
    pub style_sheet_id: Box<StyleSheetId>,
    pub selector_text: String,
}
/// ⚠️ Experimental
/** Given a CSS selector text and a style sheet ID, getLocationForSelector
returns an array of locations of the CSS selector in the style sheet.*/
pub type CssGetLocationForSelectorReturns = ();
/// ⚠️ Experimental
/** Starts tracking the given node for the computed style updates
and whenever the computed style is updated for node, it queues
a `computedStyleUpdated` event with throttling.
There can only be 1 node tracked for computed style updates
so passing a new node id removes tracking from the previous node.
Pass `undefined` to disable tracking.*/
pub struct CssTrackComputedStyleUpdatesForNodeParams {
    pub node_id: Box<NodeId>,
}
/// ⚠️ Experimental
/** Starts tracking the given node for the computed style updates
and whenever the computed style is updated for node, it queues
a `computedStyleUpdated` event with throttling.
There can only be 1 node tracked for computed style updates
so passing a new node id removes tracking from the previous node.
Pass `undefined` to disable tracking.*/
pub type CssTrackComputedStyleUpdatesForNodeReturns = ();
/// ⚠️ Experimental
/** Starts tracking the given computed styles for updates. The specified array of properties
replaces the one previously specified. Pass empty array to disable tracking.
Use takeComputedStyleUpdates to retrieve the list of nodes that had properties modified.
The changes to computed style properties are only tracked for nodes pushed to the front-end
by the DOM agent. If no changes to the tracked properties occur after the node has been pushed
to the front-end, no updates will be issued for the node.*/
pub struct CssTrackComputedStyleUpdatesParams {
    pub properties_to_track: Vec<CssComputedStyleProperty>,
}
/// ⚠️ Experimental
/** Starts tracking the given computed styles for updates. The specified array of properties
replaces the one previously specified. Pass empty array to disable tracking.
Use takeComputedStyleUpdates to retrieve the list of nodes that had properties modified.
The changes to computed style properties are only tracked for nodes pushed to the front-end
by the DOM agent. If no changes to the tracked properties occur after the node has been pushed
to the front-end, no updates will be issued for the node.*/
pub type CssTrackComputedStyleUpdatesReturns = ();
/// ⚠️ Experimental
/// Polls the next batch of computed style updates.
pub type CssTakeComputedStyleUpdatesParams = ();
/// ⚠️ Experimental
/// Polls the next batch of computed style updates.
pub type CssTakeComputedStyleUpdatesReturns = ();
/** Find a rule with the given active property for the given node and set the new value for this
property*/
pub struct CssSetEffectivePropertyValueForNodeParams {
    pub node_id: Box<NodeId>,
    pub property_name: String,
    pub value: String,
}
/** Find a rule with the given active property for the given node and set the new value for this
property*/
pub type CssSetEffectivePropertyValueForNodeReturns = ();
/// Modifies the property rule property name.
pub struct CssSetPropertyRulePropertyNameParams {
    pub style_sheet_id: Box<StyleSheetId>,
    pub range: Box<SourceRange>,
    pub property_name: String,
}
/// Modifies the property rule property name.
pub type CssSetPropertyRulePropertyNameReturns = ();
/// Modifies the keyframe rule key text.
pub struct CssSetKeyframeKeyParams {
    pub style_sheet_id: Box<StyleSheetId>,
    pub range: Box<SourceRange>,
    pub key_text: String,
}
/// Modifies the keyframe rule key text.
pub type CssSetKeyframeKeyReturns = ();
/// Modifies the rule selector.
pub struct CssSetMediaTextParams {
    pub style_sheet_id: Box<StyleSheetId>,
    pub range: Box<SourceRange>,
    pub text: String,
}
/// Modifies the rule selector.
pub type CssSetMediaTextReturns = ();
/// ⚠️ Experimental
/// Modifies the expression of a container query.
pub struct CssSetContainerQueryTextParams {
    pub style_sheet_id: Box<StyleSheetId>,
    pub range: Box<SourceRange>,
    pub text: String,
}
/// ⚠️ Experimental
/// Modifies the expression of a container query.
pub type CssSetContainerQueryTextReturns = ();
/// ⚠️ Experimental
/// Modifies the expression of a supports at-rule.
pub struct CssSetSupportsTextParams {
    pub style_sheet_id: Box<StyleSheetId>,
    pub range: Box<SourceRange>,
    pub text: String,
}
/// ⚠️ Experimental
/// Modifies the expression of a supports at-rule.
pub type CssSetSupportsTextReturns = ();
/// ⚠️ Experimental
/// Modifies the expression of a scope at-rule.
pub struct CssSetScopeTextParams {
    pub style_sheet_id: Box<StyleSheetId>,
    pub range: Box<SourceRange>,
    pub text: String,
}
/// ⚠️ Experimental
/// Modifies the expression of a scope at-rule.
pub type CssSetScopeTextReturns = ();
/// Modifies the rule selector.
pub struct CssSetRuleSelectorParams {
    pub style_sheet_id: Box<StyleSheetId>,
    pub range: Box<SourceRange>,
    pub selector: String,
}
/// Modifies the rule selector.
pub type CssSetRuleSelectorReturns = ();
/// Sets the new stylesheet text.
pub struct CssSetStyleSheetTextParams {
    pub style_sheet_id: Box<StyleSheetId>,
    pub text: String,
}
/// Sets the new stylesheet text.
pub type CssSetStyleSheetTextReturns = ();
/// Applies specified style edits one after another in the given order.
pub struct CssSetStyleTextsParams {
    pub edits: Vec<StyleDeclarationEdit>,
    pub node_for_property_syntax_validation: Box<NodeId>,
}
/// Applies specified style edits one after another in the given order.
pub type CssSetStyleTextsReturns = ();
/// Enables the selector recording.
pub type CssStartRuleUsageTrackingParams = ();
/// Enables the selector recording.
pub type CssStartRuleUsageTrackingReturns = ();
/** Stop tracking rule usage and return the list of rules that were used since last call to
`takeCoverageDelta` (or since start of coverage instrumentation).*/
pub type CssStopRuleUsageTrackingParams = ();
/** Stop tracking rule usage and return the list of rules that were used since last call to
`takeCoverageDelta` (or since start of coverage instrumentation).*/
pub type CssStopRuleUsageTrackingReturns = ();
/** Obtain list of rules that became used since last call to this method (or since start of coverage
instrumentation).*/
pub type CssTakeCoverageDeltaParams = ();
/** Obtain list of rules that became used since last call to this method (or since start of coverage
instrumentation).*/
pub type CssTakeCoverageDeltaReturns = ();
/// ⚠️ Experimental
/// Enables/disables rendering of local CSS fonts (enabled by default).
pub struct CssSetLocalFontsEnabledParams {
    pub enabled: bool,
}
/// ⚠️ Experimental
/// Enables/disables rendering of local CSS fonts (enabled by default).
pub type CssSetLocalFontsEnabledReturns = ();
