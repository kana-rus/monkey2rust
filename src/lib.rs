#![doc(html_root_url = "https://docs.rs/monkey2rust/0.1.0")]

use proc_macro::TokenStream;
mod internal;

#[proc_macro]
pub fn monkey(code: TokenStream) -> TokenStream {
    internal::monkey(code.into()).into()
}
