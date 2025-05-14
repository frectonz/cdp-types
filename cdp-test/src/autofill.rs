use crate::common::*;
pub struct CreditCard {
    pub number: Box<String>,
    pub name: Box<String>,
    pub expiry_month: Box<String>,
    pub expiry_year: Box<String>,
    pub cvc: Box<String>,
}
pub struct AddressField {
    pub name: Box<String>,
    pub value: Box<String>,
}
/// A list of address fields.
pub struct AddressFields {
    pub fields: (),
}
pub struct Address {
    pub fields: (),
}
/** Defines how an address can be displayed like in chrome://settings/addresses.
Address UI is a two dimensional array, each inner array is an "address information line", and when rendered in a UI surface should be displayed as such.
The following address UI for instance:
[[{name: "GIVE_NAME", value: "Jon"}, {name: "FAMILY_NAME", value: "Doe"}], [{name: "CITY", value: "Munich"}, {name: "ZIP", value: "81456"}]]
should allow the receiver to render:
Jon Doe
Munich 81456*/
pub struct AddressUi {
    pub address_fields: (),
}
/// Specified whether a filled field was done so by using the html autocomplete attribute or autofill heuristics.
pub enum FillingStrategy {
    AutocompleteAttribute,
    AutofillInferred,
}
pub struct FilledField {
    pub html_type: Box<String>,
    pub id: Box<String>,
    pub name: Box<String>,
    pub value: Box<String>,
    pub autofill_type: Box<String>,
    pub filling_strategy: Box<FillingStrategy>,
    pub frame_id: Box<FrameId>,
    pub field_id: Box<BackendNodeId>,
}
