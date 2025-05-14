pub use crate::common::*;
pub struct CreditCard {
    pub number: String,
    pub name: String,
    pub expiry_month: String,
    pub expiry_year: String,
    pub cvc: String,
}
pub struct AddressField {
    pub name: String,
    pub value: String,
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
    pub html_type: String,
    pub id: String,
    pub name: String,
    pub value: String,
    pub autofill_type: String,
    pub filling_strategy: (),
    pub frame_id: (),
    pub field_id: (),
}
