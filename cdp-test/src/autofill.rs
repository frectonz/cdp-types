/// <https://chromedevtools.github.io/devtools-protocol/tot/Autofill/#type-CreditCard>
pub struct AutofillCreditCard {
    pub number: (),
    pub name: (),
    pub expiry_month: (),
    pub expiry_year: (),
    pub cvc: (),
}
/// <https://chromedevtools.github.io/devtools-protocol/tot/Autofill/#type-AddressField>
pub struct AutofillAddressField {
    pub name: (),
    pub value: (),
}
/// A list of address fields.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Autofill/#type-AddressFields>
pub struct AutofillAddressFields {
    pub fields: (),
}
/// <https://chromedevtools.github.io/devtools-protocol/tot/Autofill/#type-Address>
pub struct AutofillAddress {
    pub fields: (),
}
/** Defines how an address can be displayed like in chrome://settings/addresses.
Address UI is a two dimensional array, each inner array is an "address information line", and when rendered in a UI surface should be displayed as such.
The following address UI for instance:
[[{name: "GIVE_NAME", value: "Jon"}, {name: "FAMILY_NAME", value: "Doe"}], [{name: "CITY", value: "Munich"}, {name: "ZIP", value: "81456"}]]
should allow the receiver to render:
Jon Doe
Munich 81456*/
/// <https://chromedevtools.github.io/devtools-protocol/tot/Autofill/#type-AddressUI>
pub struct AutofillAddressUi {
    pub address_fields: (),
}
/// Specified whether a filled field was done so by using the html autocomplete attribute or autofill heuristics.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Autofill/#type-FillingStrategy>
pub enum AutofillFillingStrategy {
    AutocompleteAttribute,
    AutofillInferred,
}
/// <https://chromedevtools.github.io/devtools-protocol/tot/Autofill/#type-FilledField>
pub struct AutofillFilledField {
    pub html_type: (),
    pub id: (),
    pub name: (),
    pub value: (),
    pub autofill_type: (),
    pub filling_strategy: (),
    pub frame_id: (),
    pub field_id: (),
}
