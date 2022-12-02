use proc_macro::TokenStream;
use comrak;
use syn::{parse_macro_input, LitStr};
use quote::quote;

#[proc_macro]
pub fn md_to_html(input: TokenStream) -> TokenStream {
    let markdown = parse_macro_input!(input as LitStr).value();

    let out = comrak::markdown_to_html(&markdown, &comrak::ComrakOptions::default());

    (quote!{
        #out
    }).into()
}
