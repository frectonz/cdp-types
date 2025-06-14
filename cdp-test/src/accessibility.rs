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
/// Disables the accessibility domain.
pub type AccessibilityDisableParams = ();
/// Disables the accessibility domain.
pub type AccessibilityDisableReturns = ();
/** Enables the accessibility domain which causes `AXNodeId`s to remain consistent between method calls.
This turns on accessibility for the page, which can impact performance until accessibility is disabled.*/
pub type AccessibilityEnableParams = ();
/** Enables the accessibility domain which causes `AXNodeId`s to remain consistent between method calls.
This turns on accessibility for the page, which can impact performance until accessibility is disabled.*/
pub type AccessibilityEnableReturns = ();
/// ⚠️ Experimental
/// Fetches the accessibility node and partial accessibility tree for this DOM node, if it exists.
pub struct AccessibilityGetPartialAxTreeParams {
    pub node_id: Box<NodeId>,
    pub backend_node_id: Box<BackendNodeId>,
    pub object_id: Box<()>,
    pub fetch_relatives: bool,
}
/// ⚠️ Experimental
/// Fetches the accessibility node and partial accessibility tree for this DOM node, if it exists.
pub struct AccessibilityGetPartialAxTreeParams {
    pub nodes: Vec<AxNode>,
}
/// ⚠️ Experimental
/// Fetches the entire accessibility tree for the root Document
pub struct AccessibilityGetFullAxTreeParams {
    pub depth: i64,
    pub frame_id: Box<crate::page::FrameId>,
}
/// ⚠️ Experimental
/// Fetches the entire accessibility tree for the root Document
pub struct AccessibilityGetFullAxTreeParams {
    pub nodes: Vec<AxNode>,
}
/// ⚠️ Experimental
/** Fetches the root node.
Requires `enable()` to have been called previously.*/
pub struct AccessibilityGetRootAxNodeParams {
    pub frame_id: Box<crate::page::FrameId>,
}
/// ⚠️ Experimental
/** Fetches the root node.
Requires `enable()` to have been called previously.*/
pub struct AccessibilityGetRootAxNodeParams {
    pub node: Box<AxNode>,
}
/// ⚠️ Experimental
/** Fetches a node and all ancestors up to and including the root.
Requires `enable()` to have been called previously.*/
pub struct AccessibilityGetAxNodeAndAncestorsParams {
    pub node_id: Box<NodeId>,
    pub backend_node_id: Box<BackendNodeId>,
    pub object_id: Box<()>,
}
/// ⚠️ Experimental
/** Fetches a node and all ancestors up to and including the root.
Requires `enable()` to have been called previously.*/
pub struct AccessibilityGetAxNodeAndAncestorsParams {
    pub nodes: Vec<AxNode>,
}
/// ⚠️ Experimental
/** Fetches a particular accessibility node by AXNodeId.
Requires `enable()` to have been called previously.*/
pub struct AccessibilityGetChildAxNodesParams {
    pub id: Box<AxNodeId>,
    pub frame_id: Box<crate::page::FrameId>,
}
/// ⚠️ Experimental
/** Fetches a particular accessibility node by AXNodeId.
Requires `enable()` to have been called previously.*/
pub struct AccessibilityGetChildAxNodesParams {
    pub nodes: Vec<AxNode>,
}
/// ⚠️ Experimental
/** Query a DOM node's accessibility subtree for accessible name and role.
This command computes the name and role for all nodes in the subtree, including those that are
ignored for accessibility, and returns those that match the specified name and role. If no DOM
node is specified, or the DOM node does not exist, the command returns an error. If neither
`accessibleName` or `role` is specified, it returns all the accessibility nodes in the subtree.*/
pub struct AccessibilityQueryAxTreeParams {
    pub node_id: Box<NodeId>,
    pub backend_node_id: Box<BackendNodeId>,
    pub object_id: Box<()>,
    pub accessible_name: String,
    pub role: String,
}
/// ⚠️ Experimental
/** Query a DOM node's accessibility subtree for accessible name and role.
This command computes the name and role for all nodes in the subtree, including those that are
ignored for accessibility, and returns those that match the specified name and role. If no DOM
node is specified, or the DOM node does not exist, the command returns an error. If neither
`accessibleName` or `role` is specified, it returns all the accessibility nodes in the subtree.*/
pub struct AccessibilityQueryAxTreeParams {
    pub nodes: Vec<AxNode>,
}
/// ⚠️ Experimental
/** The loadComplete event mirrors the load complete event sent by the browser to assistive
technology when the web page has finished loading.*/
pub struct AccessibilityLoadCompleteEvent {
    pub root: Box<AxNode>,
}
/// ⚠️ Experimental
/// The nodesUpdated event is sent every time a previously requested node has changed the in tree.
pub struct AccessibilityNodesUpdatedEvent {
    pub nodes: Vec<AxNode>,
}
