use color_eyre::eyre::Result;
use heck::ToSnakeCase;
use proc_macro2::TokenStream;
use quote::quote;
use serde::Deserialize;

type Str = Box<str>;
type List<T> = Box<[T]>;

#[derive(Debug, Deserialize)]
pub struct BrowserProtocol {
    pub version: Version,
    pub domains: List<Domain>,
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
    pub id: Str,
    pub r#type: Str,
    pub items: Option<Items>,
    pub description: Option<Str>,
    pub r#enum: Option<List<Str>>,
    pub properties: Option<List<Property>>,

    #[serde(default)]
    pub deprecated: bool,
    #[serde(default)]
    pub experimental: bool,
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

impl Domain {
    fn module_name(&self) -> String {
        self.domain.to_snake_case()
    }

    fn to_rust(&self) -> RustFile {
        let content = quote! {};

        RustFile {
            name: self.module_name(),
            content,
        }
    }
}

impl BrowserProtocol {
    fn generate(self) -> Result<()> {
        for domain in self.domains {
            domain.to_rust().write()?;
        }

        Ok(())
    }
}

struct RustFile {
    name: String,
    content: TokenStream,
}

impl RustFile {
    fn write(self) -> Result<()> {
        let file: syn::File = syn::parse2(self.content)?;
        let file = prettyplease::unparse(&file);
        std::fs::write(format!("out/{}.rs", self.name), file)?;

        Ok(())
    }
}

fn main() -> Result<()> {
    let data = std::fs::read_to_string("data/browser_protocol.json")?;
    let protocol: BrowserProtocol = serde_json::from_str(&data)?;

    println!(
        "Generating types for CDP version {}.{}",
        protocol.version.major, protocol.version.minor
    );

    protocol.generate()?;

    Ok(())
}
