use std::collections::HashMap;

use color_eyre::eyre::Result;
use heck::{ToPascalCase, ToSnakeCase};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use serde::Deserialize;
use syn::Ident;

type Str = Box<str>;
type List<T> = Box<[T]>;

#[derive(Debug, Deserialize)]
pub struct BrowserProtocol {
    pub version: Version,
    pub domains: List<Domain>,
}

#[derive(Debug)]
struct ProtocolData {
    common_type_ids: Vec<Str>,
    common_types: Vec<(Str, Type)>,
    common_type_name_map: HashMap<Str, Str>,
}

impl ProtocolData {
    fn is_common_type(&self, typ: &str) -> bool {
        self.common_type_ids
            .iter()
            .find(|x| x.as_ref() == typ)
            .is_some()
    }

    fn get_common_type(&self, real_type_name: &str) -> Option<&str> {
        self.common_type_name_map
            .get(real_type_name)
            .map(|x| x.as_ref())
    }
}

impl BrowserProtocol {
    fn protocol_data(&self) -> ProtocolData {
        let common_type_ids = self.common_type_ids();
        let mut common_types = Vec::with_capacity(common_type_ids.len());
        let mut common_type_name_map = HashMap::with_capacity(common_type_ids.len());

        for domain in self.domains.iter() {
            for typ in domain.types.iter() {
                if common_type_ids.contains(&typ.id) {
                    let mut typ = typ.to_owned();
                    let domain = domain.domain.to_owned();

                    let id = typ.id.as_ref();
                    let id = if id.starts_with(domain.as_ref()) {
                        id.to_pascal_case()
                    } else {
                        let domain = domain.to_pascal_case();
                        format!("{domain}{id}").to_pascal_case()
                    };

                    let original = typ.id.as_ref();
                    let resolved = id.into_boxed_str();
                    let real_type_name = format!("{domain}.{original}").into_boxed_str();

                    typ.id = resolved.clone();

                    common_types.push((domain, typ));
                    common_type_name_map.insert(real_type_name, resolved);
                }
            }
        }

        ProtocolData {
            common_type_ids,
            common_types,
            common_type_name_map,
        }
    }

    fn common_type_ids(&self) -> Vec<Str> {
        let mut type_to_count: HashMap<Str, u8> = HashMap::new();

        for domain in self.domains.iter() {
            for typ in domain.types.iter() {
                type_to_count
                    .entry(typ.id.clone())
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
        }

        type_to_count
            .into_iter()
            .filter(|(_, c)| *c > 1)
            .map(|(t, _)| t)
            .collect()
    }
}

#[derive(Debug, Deserialize)]
pub struct Version {
    pub major: Str,
    pub minor: Str,
}

#[derive(Debug, Deserialize)]
pub struct Domain {
    pub domain: Str,
    pub description: Option<Str>,

    #[serde(default)]
    pub dependencies: List<Str>,

    #[serde(default)]
    pub types: List<Type>,
    #[serde(default)]
    pub events: List<Event>,
    #[serde(default)]
    pub commands: List<Command>,

    #[serde(default)]
    pub experimental: bool,
}

#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Eq)]
pub struct Items {
    pub r#type: Option<Str>,
    #[serde(rename = "$ref")]
    pub r#ref: Option<Str>,
}

#[derive(Debug, Deserialize)]
pub struct Parameter {
    pub name: Str,
    pub items: Option<Items>,
    pub description: Option<Str>,

    pub r#type: Option<Str>,
    pub r#enum: Option<List<Str>>,

    #[serde(rename = "$ref")]
    pub r#ref: Option<Str>,

    #[serde(default)]
    pub optional: bool,
    #[serde(default)]
    pub deprecated: bool,
    #[serde(default)]
    pub experimental: bool,
}

#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Eq)]
pub struct Type {
    id: Str,
    items: Option<Items>,
    description: Option<Str>,
    properties: Option<List<Property>>,

    r#type: Str,
    r#enum: Option<List<Str>>,

    #[serde(default)]
    deprecated: bool,
    #[serde(default)]
    experimental: bool,
}

#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Eq)]
pub struct Property {
    pub name: Str,
    pub items: Option<Items>,
    pub description: Option<Str>,

    pub r#type: Option<Str>,
    pub r#enum: Option<List<Str>>,

    #[serde(rename = "$ref")]
    pub r#ref: Option<Str>,

    #[serde(default)]
    pub optional: bool,
    #[serde(default)]
    pub deprecated: bool,
    #[serde(default)]
    pub experimental: bool,
}

#[derive(Debug, Deserialize)]
pub struct Command {
    pub name: Str,
    pub redirect: Option<Str>,
    pub description: Option<Str>,
    pub returns: Option<List<Return>>,
    pub parameters: Option<List<Parameter>>,

    #[serde(default)]
    pub deprecated: bool,
    #[serde(default)]
    pub experimental: bool,
}

#[derive(Debug, Deserialize)]
pub struct Return {
    pub name: Str,
    pub items: Option<Items>,
    pub description: Option<Str>,

    pub r#type: Option<Str>,

    #[serde(rename = "$ref")]
    pub r#ref: Option<Str>,

    #[serde(default)]
    pub optional: bool,
    #[serde(default)]
    pub deprecated: bool,
    #[serde(default)]
    pub experimental: bool,
}

#[derive(Debug, Deserialize)]
pub struct Event {
    pub name: Str,
    pub description: Option<Str>,
    pub parameters: Option<List<Parameter>>,

    #[serde(default)]
    pub deprecated: bool,
    #[serde(default)]
    pub experimental: bool,
}

fn get_rust_type(typ: &str) -> Option<TokenStream> {
    match typ {
        "integer" => Some(quote! { i64 }),
        "number" => Some(quote! { u64 }),
        "string" => Some(quote! { String }),
        "object" => Some(quote! {
            serde_json::Map<String, serde_json::Value>
        }),

        _ => None,
    }
}

impl Property {
    fn to_rust(&self, protocol_data: &ProtocolData) -> TokenStream {
        let name = if self.name.as_ref() == "type" {
            "_type".to_owned()
        } else {
            self.name.to_snake_case()
        };

        let name = Ident::new(&name, Span::call_site());

        let basic_type = self.r#type.as_ref().and_then(|typ| get_rust_type(typ));
        let ref_typ = self.r#ref.as_ref().map(|ref_typ| {
            if ref_typ.as_ref() == "WindowState" {
                if self
                    .description
                    .as_ref()
                    .map(|desc| desc.to_lowercase().contains("target"))
                    .unwrap_or_default()
                {
                    return quote! {
                        TargetWindowState
                    };
                } else {
                    return quote! {
                        BrowserWindowState
                    };
                }
            }

            if ref_typ.as_ref() == "Page.FrameId" {
                return quote! {
                    crate::page::FrameId
                };
            }

            if ref_typ.as_ref().starts_with("Runtime.") {
                return quote! {
                    ()
                };
            }

            let common_type = protocol_data.get_common_type(&ref_typ);

            let ref_type = match common_type {
                Some(ref_typ) => Ident::new(ref_typ, Span::call_site()),
                None => {
                    let (_, ref_typ) = ref_typ.split_once('.').unwrap_or(("", ref_typ.as_ref()));
                    let ref_type = ref_typ.to_pascal_case();
                    Ident::new(&ref_type, Span::call_site())
                }
            };

            quote! { #ref_type }
        });

        let property_type = basic_type.or(ref_typ);

        if let Some(typ) = property_type {
            return quote! {
                pub #name: Box<#typ>
            };
        }

        quote! {
            pub #name: ()
        }
    }
}

impl Type {
    fn id_ident(&self) -> Ident {
        let id = self.id.to_pascal_case();
        Ident::new(&id, Span::call_site())
    }

    fn enum_variants(&self) -> Option<TokenStream> {
        self.r#enum.as_ref().map(|enums| {
            // All enums have the type `string`.
            assert_eq!(self.r#type.as_ref(), "string");

            let enum_defs = enums.into_iter().map(|e| {
                let enum_name = if e.as_ref() == "Self" {
                    "SELF".to_owned()
                } else {
                    e.to_pascal_case()
                };

                let enum_name = Ident::new(&enum_name, Span::call_site());
                quote! {
                    #enum_name
                }
            });

            quote! {
                #(#enum_defs),*
            }
        })
    }

    fn properties(&self, protocol_data: &ProtocolData) -> Option<TokenStream> {
        self.properties.as_ref().map(|props| {
            // All objects have the type `object`.
            assert_eq!(self.r#type.as_ref(), "object");

            let property_defs = props.into_iter().map(|p| p.to_rust(protocol_data));

            quote! {
                #(#property_defs),*
            }
        })
    }

    fn items(&self) -> Option<TokenStream> {
        self.items.as_ref().and_then(|items| {
            // All arrays have the type `array`.
            assert_eq!(self.r#type.as_ref(), "array");

            let items_typ = items.r#type.as_ref().and_then(|typ| get_rust_type(typ));
            let items_ref = items.r#ref.as_ref().and_then(|typ| {
                let typ_ident = typ.to_pascal_case();
                let typ_ident = Ident::new(&typ_ident, Span::call_site());

                Some(quote! { #typ_ident })
            });

            items_typ.or(items_ref)
        })
    }

    fn deprecated_flag(&self) -> TokenStream {
        if self.deprecated {
            quote! { #[deprecated] }
        } else {
            quote! {}
        }
    }

    fn experimental_flag(&self) -> TokenStream {
        if self.experimental {
            quote! { #[doc = " ⚠️ Experimental"] }
        } else {
            quote! {}
        }
    }

    fn description(&self) -> TokenStream {
        let description = self
            .description
            .as_ref()
            .map(|desc| {
                let desc = format!(" {desc}");
                quote! { #[doc = #desc] }
            })
            .unwrap_or_default();

        quote! {
            #description
        }
    }

    fn to_rust(&self, protocol_data: &ProtocolData) -> TokenStream {
        let id = self.id_ident();
        let items = self.items();
        let properties = self.properties(protocol_data);
        let enum_variants = self.enum_variants();

        let description = self.description();
        let deprecated = self.deprecated_flag();
        let experimental = self.experimental_flag();

        let attrs = quote! {
            #deprecated
            #experimental
            #description
        };

        if let Some(enum_variants) = enum_variants {
            return quote! {
                #attrs
                pub enum #id { #enum_variants }
            };
        }

        if let Some(properties) = properties {
            return quote! {
                #attrs
                pub struct #id { #properties }
            };
        }

        if let Some(typ) = get_rust_type(&self.r#type) {
            return quote! {
                #attrs
                pub struct #id(#typ);
            };
        }

        if let Some(items) = items {
            return quote! {
                #attrs
                pub struct #id(Vec<#items>);
            };
        }

        unreachable!("Couldn't convert '{}' to a Rust type.", self.id);
    }
}

impl Domain {
    fn module_name(&self) -> String {
        self.domain.to_snake_case()
    }

    fn dependency_names(&self) -> impl Iterator<Item = TokenStream> {
        self.dependencies
            .as_ref()
            .iter()
            .filter(|x| x.as_ref() != "Runtime")
            .filter(|x| x.as_ref() != "Debugger")
            .map(|dep| {
                let dep = dep.to_snake_case();
                let dep = Ident::new(&dep, Span::call_site());

                quote! {
                    use crate::#dep::*;
                }
            })
    }

    fn to_rust(&self, protocol: &ProtocolData) -> RustFile {
        let name = self.module_name();

        let dependecies = self.dependency_names();
        let types = self
            .types
            .iter()
            .filter(|x| !protocol.is_common_type(&x.id))
            .map(|t| t.to_rust(protocol));

        let content = quote! {
            use crate::common::*;
            #(#dependecies)*
            #(#types)*
        };

        RustFile { name, content }
    }
}

impl BrowserProtocol {
    fn generate(self) -> Result<()> {
        let protocol_data = self.protocol_data();

        let common_types = protocol_data.common_types.iter().map(|(domain, typ)| {
            let common_type = typ.to_rust(&protocol_data);
            let domain = domain.to_snake_case();
            let domain = Ident::new(&domain, Span::call_site());

            quote! {
                use crate::#domain::*;

                #common_type
            }
        });

        let common_rs = RustFile {
            name: "common".to_owned(),
            content: quote! {
                #(#common_types)*
            },
        };

        common_rs.write()?;

        let mut modules = Vec::with_capacity(self.domains.len());
        for domain in self.domains {
            let ident = domain.to_rust(&protocol_data).write()?;
            modules.push(ident);
        }

        let modules = modules.into_iter().map(|module| {
            quote! {
                pub mod #module;
            }
        });

        let lib_rs = RustFile {
            name: "lib".to_owned(),
            content: quote! {
                #![allow(unused_imports)]
                pub mod common;
                #(#modules)*
            },
        };

        lib_rs.write()?;

        Ok(())
    }
}

struct RustFile {
    name: String,
    content: TokenStream,
}

impl RustFile {
    fn write(self) -> Result<Ident> {
        let file: syn::File = syn::parse2(self.content)?;
        let file = prettyplease::unparse(&file);
        std::fs::write(format!("cdp-test/src/{}.rs", self.name), file)?;

        let ident = Ident::new(&self.name, Span::call_site());
        Ok(ident)
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let data = std::fs::read_to_string("data/browser_protocol.json")?;
    let protocol: BrowserProtocol = serde_json::from_str(&data)?;

    println!(
        "Generating types for CDP version {}.{}",
        protocol.version.major, protocol.version.minor
    );

    protocol.generate()?;

    Ok(())
}
