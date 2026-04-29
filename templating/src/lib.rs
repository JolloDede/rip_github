extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{LitStr, parse_macro_input};
mod parser;
use parser::*;
mod render;
use render::*;

#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
    let HtmlInput { nodes } = parse_macro_input!(input as HtmlInput);

    let expanded = render(nodes);

    TokenStream::from(expanded)
}
