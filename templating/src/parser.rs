use syn::Ident;
use syn::LitStr;
use syn::Result;
use syn::parse::{Parse, ParseStream};

pub struct HtmlInput {
    // pub tokens: proc_macro2::TokenStream,
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

pub enum Node {
    Element { name: String, children: Vec<Node> },
    Text(String),
}

impl Parse for Node {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Ident) {
            let ident: Ident = input.parse()?;
            let content;
            syn::braced!(content in input);
            let children = parse_children(&content)?;
            return Ok(Node::Element {
                name: ident.to_string(),
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
