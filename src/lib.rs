extern crate proc_macro;
use proc_macro2::TokenStream;
mod lexer;
mod parser;
use parser::parse;

pub fn preprocess_and_generate_rust_code(content: &str) -> TokenStream {
    let rust_code = parse(content);
    rust_code.parse().unwrap()

}
