extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Expr, ItemFn, Stmt, parse_macro_input, parse_quote};

mod parser;
use parser::*;
mod render;
use render::*;
mod consts;

#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
    let HtmlInput { nodes } = parse_macro_input!(input as HtmlInput);

    let expanded = render(nodes);

    TokenStream::from(expanded)
}

#[proc_macro]
pub fn component_html(input: TokenStream) -> TokenStream {
    let HtmlInput { nodes } = parse_macro_input!(input as HtmlInput);

    TokenStream::from(render_string(nodes))
}

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn Component(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut function = parse_macro_input!(input as ItemFn);

    for argument in &mut function.sig.inputs {
        if let syn::FnArg::Typed(argument) = argument {
            argument.ty = Box::new(parse_quote!(String));
        }
    }
    function.sig.output = parse_quote!(-> String);

    match function.block.stmts.last_mut() {
        Some(Stmt::Expr(expression, semicolon)) => {
            *semicolon = None;
            replace_html_macro(expression);
        }
        Some(Stmt::Macro(statement)) => {
            statement.semi_token = None;
            statement.mac.path = parse_quote!(::rip_templating_macro::component_html);
        }
        _ => {
            return syn::Error::new_spanned(
                function.sig.ident,
                "a component must end with an html! expression",
            )
            .to_compile_error()
            .into();
        }
    }

    quote!(#function).into()
}

fn replace_html_macro(expression: &mut Expr) {
    if let Expr::Macro(expression) = expression {
        expression.mac.path = parse_quote!(::templating_macro::component_html);
    }
}
