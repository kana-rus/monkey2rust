use proc_macro2::TokenStream;
use quote::quote;

use super::syntax::Program;

impl Into<TokenStream> for Program {
    fn into(self) -> TokenStream {
        let mut main_process = TokenStream::new();

        quote!(
            fn main() {
                #main_process
            }
        )
    }
}