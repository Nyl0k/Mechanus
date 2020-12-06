extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn m_embed(input: TokenStream) -> TokenStream {
    format!("|m| {{m.set_embed({}); m}}", input).parse().unwrap()
}