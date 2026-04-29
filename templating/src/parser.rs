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
        todo!()
    }
}
