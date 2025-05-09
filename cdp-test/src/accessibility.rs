use crate::dom::*;
/// Unique accessibility node identifier.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Accessibility/#type-AXNodeId>
pub struct AccessibilityAxNodeId(String);
/// Enum of possible property types.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Accessibility/#type-AXValueType>
pub enum AccessibilityAxValueType {
    Boolean,
    Tristate,
    BooleanOrUndefined,
    Idref,
    IdrefList,
    Integer,
    Node,
    NodeList,
    Number,
    String,
    ComputedString,
    Token,
    TokenList,
    DomRelation,
    Role,
    InternalRole,
    ValueUndefined,
}
/// Enum of possible property sources.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Accessibility/#type-AXValueSourceType>
pub enum AccessibilityAxValueSourceType {
    Attribute,
    Implicit,
    Style,
    Contents,
    Placeholder,
    RelatedElement,
}
/// Enum of possible native property sources (as a subtype of a particular AXValueSourceType).
/// <https://chromedevtools.github.io/devtools-protocol/tot/Accessibility/#type-AXValueNativeSourceType>
pub enum AccessibilityAxValueNativeSourceType {
    Description,
    Figcaption,
    Label,
    Labelfor,
    Labelwrapped,
    Legend,
    Rubyannotation,
    Tablecaption,
    Title,
    Other,
}
/// A single source for a computed AX property.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Accessibility/#type-AXValueSource>
pub struct AccessibilityAxValueSource {
    pub _type: (),
    pub value: (),
    pub attribute: (),
    pub attribute_value: (),
    pub superseded: (),
    pub native_source: (),
    pub native_source_value: (),
    pub invalid: (),
    pub invalid_reason: (),
}
/// <https://chromedevtools.github.io/devtools-protocol/tot/Accessibility/#type-AXRelatedNode>
pub struct AccessibilityAxRelatedNode {
    pub backend_dom_node_id: (),
    pub idref: (),
    pub text: (),
}
/// <https://chromedevtools.github.io/devtools-protocol/tot/Accessibility/#type-AXProperty>
pub struct AccessibilityAxProperty {
    pub name: (),
    pub value: (),
}
/// A single computed AX property.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Accessibility/#type-AXValue>
pub struct AccessibilityAxValue {
    pub _type: (),
    pub value: (),
    pub related_nodes: (),
    pub sources: (),
}
/** Values of AXProperty name:
- from 'busy' to 'roledescription': states which apply to every AX node
- from 'live' to 'root': attributes which apply to nodes in live regions
- from 'autocomplete' to 'valuetext': attributes which apply to widgets
- from 'checked' to 'selected': states which apply to widgets
- from 'activedescendant' to 'owns' - relationships between elements other than parent/child/sibling.*/
/// <https://chromedevtools.github.io/devtools-protocol/tot/Accessibility/#type-AXPropertyName>
pub enum AccessibilityAxPropertyName {
    Actions,
    Busy,
    Disabled,
    Editable,
    Focusable,
    Focused,
    Hidden,
    HiddenRoot,
    Invalid,
    Keyshortcuts,
    Settable,
    Roledescription,
    Live,
    Atomic,
    Relevant,
    Root,
    Autocomplete,
    HasPopup,
    Level,
    Multiselectable,
    Orientation,
    Multiline,
    Readonly,
    Required,
    Valuemin,
    Valuemax,
    Valuetext,
    Checked,
    Expanded,
    Modal,
    Pressed,
    Selected,
    Activedescendant,
    Controls,
    Describedby,
    Details,
    Errormessage,
    Flowto,
    Labelledby,
    Owns,
    Url,
}
/// A node in the accessibility tree.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Accessibility/#type-AXNode>
pub struct AccessibilityAxNode {
    pub node_id: (),
    pub ignored: (),
    pub ignored_reasons: (),
    pub role: (),
    pub chrome_role: (),
    pub name: (),
    pub description: (),
    pub value: (),
    pub properties: (),
    pub parent_id: (),
    pub child_ids: (),
    pub backend_dom_node_id: (),
    pub frame_id: (),
}
