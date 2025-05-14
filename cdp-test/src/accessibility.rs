use crate::common::*;
use crate::dom::*;
use crate::page::*;
/// Unique accessibility node identifier.
pub struct AxNodeId(String);
/// Enum of possible property types.
pub enum AxValueType {
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
pub enum AxValueSourceType {
    Attribute,
    Implicit,
    Style,
    Contents,
    Placeholder,
    RelatedElement,
}
/// Enum of possible native property sources (as a subtype of a particular AXValueSourceType).
pub enum AxValueNativeSourceType {
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
pub struct AxValueSource {
    pub _type: Box<AxValueSourceType>,
    pub value: Box<AxValue>,
    pub attribute: String,
    pub attribute_value: Box<AxValue>,
    pub superseded: bool,
    pub native_source: Box<AxValueNativeSourceType>,
    pub native_source_value: Box<AxValue>,
    pub invalid: bool,
    pub invalid_reason: String,
}
pub struct AxRelatedNode {
    pub backend_dom_node_id: Box<BackendNodeId>,
    pub idref: String,
    pub text: String,
}
pub struct AxProperty {
    pub name: Box<AxPropertyName>,
    pub value: Box<AxValue>,
}
/// A single computed AX property.
pub struct AxValue {
    pub _type: Box<AxValueType>,
    pub value: serde_json::Value,
    pub related_nodes: Vec<AxRelatedNode>,
    pub sources: Vec<AxValueSource>,
}
/** Values of AXProperty name:
- from 'busy' to 'roledescription': states which apply to every AX node
- from 'live' to 'root': attributes which apply to nodes in live regions
- from 'autocomplete' to 'valuetext': attributes which apply to widgets
- from 'checked' to 'selected': states which apply to widgets
- from 'activedescendant' to 'owns' - relationships between elements other than parent/child/sibling.*/
pub enum AxPropertyName {
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
pub struct AxNode {
    pub node_id: Box<AxNodeId>,
    pub ignored: bool,
    pub ignored_reasons: Vec<AxProperty>,
    pub role: Box<AxValue>,
    pub chrome_role: Box<AxValue>,
    pub name: Box<AxValue>,
    pub description: Box<AxValue>,
    pub value: Box<AxValue>,
    pub properties: Vec<AxProperty>,
    pub parent_id: Box<AxNodeId>,
    pub child_ids: Vec<AxNodeId>,
    pub backend_dom_node_id: Box<BackendNodeId>,
    pub frame_id: Box<crate::page::FrameId>,
}
