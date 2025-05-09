use std::collections::HashMap;

use color_eyre::eyre::Result;
use heck::{ToPascalCase, ToSnakeCase};
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use serde::Deserialize;
use syn::Ident;

type Str = Box<str>;
type List<T> = Box<[T]>;

#[derive(Debug, Deserialize)]
pub struct BrowserProtocol {
    pub version: Version,
    pub domains: List<Domain>,
}

impl BrowserProtocol {
    fn type_to_domain_map(&self) -> HashMap<Str, Str> {
        type Type = Str;
        type Domain = Str;
        let mut type_to_domain: HashMap<Type, Domain> = HashMap::new();

        for domain in self.domains.iter() {
            for typ in domain.types.iter() {
                type_to_domain.insert(typ.id.clone(), domain.domain.clone());
            }
        }

        type_to_domain
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

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
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
    fn to_rust(&self) -> TokenStream {
        let name = if self.name.as_ref() == "type" {
            "_type".to_owned()
        } else {
            self.name.to_snake_case()
        };

        let name = Ident::new(&name, Span::call_site());

        quote! {
            pub #name: ()
        }
    }
}

impl Type {
    fn id_ident(&self, domain: &str) -> Ident {
        if self.id.starts_with(domain) {
            let id = self.id.to_pascal_case();
            Ident::new(&id, Span::call_site())
        } else {
            let id = self.id.as_ref();
            let id = format!("{domain}{id}");
            let id = id.to_pascal_case();
            Ident::new(&id, Span::call_site())
        }
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

    fn properties(&self) -> Option<TokenStream> {
        self.properties.as_ref().map(|props| {
            // All objects have the type `object`.
            assert_eq!(self.r#type.as_ref(), "object");

            let property_defs = props.into_iter().map(|p| p.to_rust());

            quote! {
                #(#property_defs),*
            }
        })
    }

    fn items(&self, type_to_domain: &HashMap<Str, Str>) -> Option<TokenStream> {
        self.items.as_ref().and_then(|items| {
            // All arrays have the type `array`.
            assert_eq!(self.r#type.as_ref(), "array");

            let items_typ = items.r#type.as_ref().and_then(|typ| get_rust_type(typ));
            let items_ref = items.r#ref.as_ref().and_then(|typ| {
                let domain = type_to_domain.get(typ)?.to_pascal_case();
                let typ_ident = format_ident!("{domain}{typ}");

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

    fn description(&self, domain: &str) -> TokenStream {
        let typ = self.id.as_ref();
        let desc = format!(
            " <https://chromedevtools.github.io/devtools-protocol/tot/{domain}/#type-{typ}>"
        );

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
            #[doc = #desc]
        }
    }

    fn to_rust(&self, domain: &str, type_to_domain: &HashMap<Str, Str>) -> TokenStream {
        let id = self.id_ident(domain);
        let items = self.items(type_to_domain);
        let properties = self.properties();
        let enum_variants = self.enum_variants();

        let description = self.description(domain);
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

    fn to_rust(&self, type_to_domain: &HashMap<Str, Str>) -> RustFile {
        let name = self.module_name();

        let dependecies = self.dependency_names();
        let types = self
            .types
            .iter()
            .map(|t| t.to_rust(&self.domain, type_to_domain));

        let content = quote! {
            #(#dependecies)*
            #(#types)*
        };

        RustFile { name, content }
    }
}

impl BrowserProtocol {
    fn generate(self) -> Result<()> {
        let type_to_domain = self.type_to_domain_map();

        let mut modules = Vec::with_capacity(self.domains.len());
        for domain in self.domains {
            let ident = domain.to_rust(&type_to_domain).write()?;
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
