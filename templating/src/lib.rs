extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{LitStr, parse_macro_input};
mod parser;
use parser::*;

#[proc_macro]
pub fn rsx(input: TokenStream) -> TokenStream {
    // For now: only accept a string literal
    let HtmlInput { tokens } = parse_macro_input!(input as HtmlInput);

    // Very basic: just return the string as-is
    let expanded = quote! {
        {
            let mut output = String::new();
            output.push_str(#tokens);
            output
        }
    };

    TokenStream::from(expanded)
}
