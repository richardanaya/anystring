extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;
use std::str::FromStr;

#[proc_macro_hack]
pub fn anystring(input: TokenStream) -> TokenStream {
    let f1 = input.to_string();
    let f2 = f1.replace("\\","\\\\");
    let totally_unescaped = f2.replace("\"","\\\"");
    TokenStream::from_str(&format!("\"{}\"", totally_unescaped)).unwrap()
}
