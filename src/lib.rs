//! Module documentation
//!
extern crate proc_macro;
use proc_macro::TokenStream;

/// Macro doc
#[proc_macro_attribute]
pub fn unit(attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Macro doc
#[proc_macro_attribute]
pub fn integration(attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}