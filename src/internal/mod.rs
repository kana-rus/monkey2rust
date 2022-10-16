use proc_macro2::TokenStream;
use syn::parse2;

mod syntax; use syntax::Program;
mod parser;
mod builder;


pub(super) fn monkey(code: TokenStream) -> TokenStream {
    let monkey_program: Program = parse2(code).expect("can't parse monkey program");
    monkey_program.into()
}