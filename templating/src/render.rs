use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::LitStr;

use crate::Node;

pub fn render(nodes: Vec<Node>) -> TokenStream {
    let statements = nodes.iter().map(render_node);
    quote! {
        {
            let mut output = String::new();
            #(#statements)*
            output
        }
    }
}

fn render_node(node: &Node) -> TokenStream {
    match node {
        Node::Text(text) => {
            let lit = LitStr::new(text, Span::call_site());
            quote! {
                output.push_str(#lit);
            }
        }
        Node::Element {
            name: name,
            children: children,
        } => {
            let name_lit = LitStr::new(name, Span::call_site());
            let child_statements = children.iter().map(render_node);

            quote! {
                output.push_str("<");
                output.push_str(#name_lit);
                output.push_str(">");
                #(#child_statements)*
                output.push_str("<");
                output.push_str("/");
                output.push_str(#name_lit);
                output.push_str(">");
            }
        }
    }
}
