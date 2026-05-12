use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::LitStr;

use crate::{Attribute, Node, consts::SPECIAL_ELEMENTS};

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
            name,
            attributes,
            children,
        } => {
            let name_lit = LitStr::new(name, Span::call_site());
            let attributes = attributes.iter().map(render_attribute);
            let child_statements = children.iter().map(render_node);

            if SPECIAL_ELEMENTS.contains(&name.as_str()) {
                if child_statements.count() > 0 {
                    panic!(
                        "Elements in this list: {} dont have child elements",
                        SPECIAL_ELEMENTS.join(",")
                    );
                }
                quote! {
                    output.push_str("<");
                    output.push_str(#name_lit);
                    #(#attributes)*
                    output.push_str(">");
                }
            } else {
                quote! {
                    output.push_str("<");
                    output.push_str(#name_lit);
                    #(#attributes)*
                    output.push_str(">");
                    #(#child_statements)*
                    output.push_str("</");
                    output.push_str(#name_lit);
                    output.push_str(">");
                }
            }
        }
    }
}

fn render_attribute(attribute: &Attribute) -> TokenStream {
    let name_lit = LitStr::new(&attribute.name.replace("_", "-"), Span::call_site());
    let value_lit = LitStr::new(&attribute.value, Span::call_site());

    quote! {
        output.push_str(" ");
        output.push_str(#name_lit);
        output.push_str("=\"");
        output.push_str(#value_lit);
        output.push_str("\"");
    }
}
