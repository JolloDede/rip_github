use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::LitStr;

use crate::{Attribute, AttributeValue, Node, consts::SPECIAL_ELEMENTS};

pub fn render(nodes: Vec<Node>) -> TokenStream {
    render_string(nodes)
}

pub fn render_string(nodes: Vec<Node>) -> TokenStream {
    let output = Ident::new("output", Span::call_site());
    let statements = nodes.iter().map(|node| render_node(node, &output));
    quote! {
        {
            let mut output = String::new();
            #(#statements)*
            output
        }
    }
}

fn render_node(node: &Node, output: &Ident) -> TokenStream {
    match node {
        Node::Text(text) => {
            let lit = LitStr::new(text, Span::call_site());
            quote! {
                #output.push_str(#lit);
            }
        }
        Node::Element {
            name,
            attributes,
            children,
        } => {
            let name_lit = LitStr::new(name, Span::call_site());
            let attributes = attributes
                .iter()
                .map(|attribute| render_attribute(attribute, output));
            let child_statements = children.iter().map(|node| render_node(node, output));

            if SPECIAL_ELEMENTS.contains(&name.as_str()) {
                if child_statements.count() > 0 {
                    panic!(
                        "Elements in this list: {} dont have child elements",
                        SPECIAL_ELEMENTS.join(",")
                    );
                }
                quote! {
                    #output.push_str("<");
                    #output.push_str(#name_lit);
                    #(#attributes)*
                    #output.push_str(">");
                }
            } else {
                quote! {
                    #output.push_str("<");
                    #output.push_str(#name_lit);
                    #(#attributes)*
                    #output.push_str(">");
                    #(#child_statements)*
                    #output.push_str("</");
                    #output.push_str(#name_lit);
                    #output.push_str(">");
                }
            }
        }
        Node::Component { name, children } => {
            let component_name = syn::Ident::new(name, Span::call_site());
            let component_output = Ident::new("__component_children", Span::call_site());
            let child_statements = children
                .iter()
                .map(|node| render_node(node, &component_output));

            quote! {
                {
                    let mut #component_output = String::new();
                    #(#child_statements)*
                    #output.push_str(&#component_name(#component_output));
                }
            }
        }
        Node::Variable(expression) => {
            quote! {
                #output.push_str(&::std::string::ToString::to_string(&(#expression)));
            }
        }
        Node::Loop {
            local,
            list,
            children,
        } => {
            let child_statements = children
                .iter()
                .map(|node| render_node(node, output));

            quote! {
                for #local in #list {
                    #(#child_statements)*
                }
            }
        }
        Node::If {
            condition,
            then_branch,
            else_branch,
        } => {
            let then_statements = then_branch
                .iter()
                .map(|node| render_node(node, output));

            let else_statements = else_branch.as_ref().map(|nodes| {
                nodes.iter().map(|node| render_node(node, output))
            });

            if let Some(else_statements) = else_statements {
                quote! {
                    if #condition {
                        #(#then_statements)*
                    } else {
                        #(#else_statements)*
                    }
                }
            } else {
                quote! {
                    if #condition {
                        #(#then_statements)*
                    }
                }
            }
        }
    }
}

fn render_attribute(attribute: &Attribute, output: &Ident) -> TokenStream {
    let attribute_name: &mut String = &mut attribute.name.replace("_", "-").as_str().into();
    if attribute_name.contains("#") {
        attribute_name.replace_range(0..2, "");
    }
    let name_lit = LitStr::new(attribute_name, Span::call_site());

    let value = match &attribute.value {
        AttributeValue::Literal(value) => {
            let value_lit = LitStr::new(value, Span::call_site());
            quote! {
                #output.push_str(#value_lit);
            }
        }
        AttributeValue::Expression(expression) => {
            quote! {
                #output.push_str(&::std::string::ToString::to_string(&(#expression)));
            }
        }
    };

    quote! {
        #output.push_str(" ");
        #output.push_str(#name_lit);
        #output.push_str("=\"");
        #value
        #output.push_str("\"");
    }
}
