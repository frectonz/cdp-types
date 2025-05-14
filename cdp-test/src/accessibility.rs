pub use crate::common::*;
use crate::dom::*;
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
    pub _type: (),
    pub value: (),
    pub attribute: String,
    pub attribute_value: (),
    pub superseded: (),
    pub native_source: (),
    pub native_source_value: (),
    pub invalid: (),
    pub invalid_reason: String,
}
pub struct AxRelatedNode {
    pub backend_dom_node_id: (),
    pub idref: String,
    pub text: String,
}
pub struct AxProperty {
    pub name: (),
    pub value: (),
}
/// A single computed AX property.
pub struct AxValue {
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
