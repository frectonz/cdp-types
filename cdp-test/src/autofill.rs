use crate::common::*;
use crate::page::*;
use crate::dom::*;
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
    pub fields: Vec<AddressField>,
}
pub struct Address {
    pub fields: Vec<AddressField>,
}
/** Defines how an address can be displayed like in chrome://settings/addresses.
Address UI is a two dimensional array, each inner array is an "address information line", and when rendered in a UI surface should be displayed as such.
The following address UI for instance:
[[{name: "GIVE_NAME", value: "Jon"}, {name: "FAMILY_NAME", value: "Doe"}], [{name: "CITY", value: "Munich"}, {name: "ZIP", value: "81456"}]]
should allow the receiver to render:
Jon Doe
Munich 81456*/
pub struct AddressUi {
    pub address_fields: Vec<AddressFields>,
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
    pub filling_strategy: Box<FillingStrategy>,
    pub frame_id: Box<crate::page::FrameId>,
    pub field_id: Box<BackendNodeId>,
}
/** Trigger autofill on a form identified by the fieldId.
If the field and related form cannot be autofilled, returns an error.*/
pub type AutofillTriggerParams = ();
/** Trigger autofill on a form identified by the fieldId.
If the field and related form cannot be autofilled, returns an error.*/
pub type AutofillTriggerReturns = ();
/// Set addresses so that developers can verify their forms implementation.
pub type AutofillSetAddressesParams = ();
/// Set addresses so that developers can verify their forms implementation.
pub type AutofillSetAddressesReturns = ();
/// Disables autofill domain notifications.
pub type AutofillDisableParams = ();
/// Disables autofill domain notifications.
pub type AutofillDisableReturns = ();
/// Enables autofill domain notifications.
pub type AutofillEnableParams = ();
/// Enables autofill domain notifications.
pub type AutofillEnableReturns = ();
