extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;
use std::str::FromStr;

#[proc_macro_hack]
pub fn anystring(input: TokenStream) -> TokenStream {
    TokenStream::from_str(&format!("r#\"{}\"#", input.to_string())).unwrap()
}
