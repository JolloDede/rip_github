use syn::Ident;
use syn::LitStr;
use syn::Result;
use syn::parse::{Parse, ParseStream};

use crate::consts::HTML_ATTRIBUTES;
use crate::consts::HTMX_ATTRIBUTES;
use crate::consts::SPECIAL_ELEMENTS;

pub struct HtmlInput {
    pub nodes: Vec<Node>,
}

impl Parse for HtmlInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut nodes = Vec::new();

        while !input.is_empty() {
            nodes.push(input.parse()?);
        }

        Ok(HtmlInput { nodes })
    }
}

pub struct Attribute {
    pub name: String,
    pub value: String,
}

pub enum Node {
    Element {
        name: String,
        children: Vec<Node>,
        attributes: Vec<Attribute>,
    },
    Text(String),
}

impl Parse for Node {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Ident) {
            let ident: Ident = input.parse()?;
            let content;
            syn::braced!(content in input);
            let attributes = parse_attributes(&content)?;
            let children = parse_children(&content)?;
            if SPECIAL_ELEMENTS.contains(&ident.to_string().as_str()) && !children.is_empty() {
                return Err(input.error(format!(
                    "Elements in this list: {} dont have child elements",
                    SPECIAL_ELEMENTS.join(",")
                )));
            }
            return Ok(Node::Element {
                name: ident.to_string(),
                attributes: attributes,
                children: children,
            });
        }

        if input.peek(LitStr) {
            let lit: LitStr = input.parse()?;
            return Ok(Node::Text(lit.value()));
        }

        Err(input.error("expected string literal or element"))
    }
}

fn parse_children(input: ParseStream) -> Result<Vec<Node>> {
    let mut nodes = Vec::new();
    while !input.is_empty() {
        if input.peek(syn::Token![,]) {
            let _: syn::Token![,] = input.parse()?;
            continue;
        }
        nodes.push(input.parse()?);
    }
    Ok(nodes)
}

fn parse_attributes(input: ParseStream) -> Result<Vec<Attribute>> {
    let mut attributes = Vec::new();
    while !input.is_empty() {
        if input.peek(Ident) && input.peek2(syn::Token![:]) {
            let name: Ident = input.parse()?;
            if !valid_attributes(name.to_string().as_str()) {
                return Err(syn::Error::new(name.span(), "invalid attribute"));
            }
            let _: syn::Token![:] = input.parse()?;
            let value: LitStr = input.parse()?;
            attributes.push(Attribute {
                name: name.to_string(),
                value: value.value(),
            });

            if input.peek(syn::Token![,]) {
                let _: syn::Token![,] = input.parse()?;
            }
        } else {
            break;
        }
    }
    Ok(attributes)
}

fn valid_attributes(attribute: &str) -> bool {
    return HTML_ATTRIBUTES.contains(&attribute) || HTMX_ATTRIBUTES.contains(&attribute);
}
