use comrak;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

#[proc_macro]
pub fn md_to_html(input: TokenStream) -> TokenStream {
    let markdown = parse_macro_input!(input as LitStr).value();

    let mut options = comrak::ComrakOptions::default();
    options.render.unsafe_ = true;

    let out = comrak::markdown_to_html(&markdown, &options);

    (quote! {
        #out
    })
    .into()
}
