//! Module documentation
//!
extern crate proc_macro;
use proc_macro::TokenStream;
mod naming;

/// Macro doc
#[proc_macro_attribute]
pub fn unit(attr: TokenStream, item: TokenStream) -> TokenStream {
    naming::impl_rename(attr, item, naming::NameType::Unit)
}

/// Macro doc
#[proc_macro_attribute]
pub fn integration(attr: TokenStream, item: TokenStream) -> TokenStream {
    naming::impl_rename(attr, item, naming::NameType::Integration)
}