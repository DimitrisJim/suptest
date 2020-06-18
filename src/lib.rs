//! Module documentation
//!
extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse, NestedMeta, ItemFn};

fn implement_unit(_attributes: &NestedMeta, _function: &ItemFn) -> TokenStream {
    // ok. return silliness for now.
    let gen = quote! {
        #[test]
        fn a_very_silly_name__for_now() {
            assert_eq!(1, 1);
        }
    };
    gen.into()
}


fn implement_integration(_attributes: &NestedMeta, _function: &ItemFn) -> TokenStream {
    // ok. return silliness for now.
    let gen = quote! {
        #[test]
        fn an_equally_silly_name__for_now(){
            assert_eq!(2, 2)
        }
    };
    gen.into()
}


/// Macro doc
#[proc_macro_attribute]
pub fn unit(attr: TokenStream, item: TokenStream) -> TokenStream {
    let function = parse(item).expect("Unable to parse function.");
    // may very well be empty.
    let attributes = match parse(attr){
        Ok(it) => it,
        // substitue with empty parents (I'd guess.)
        Err(_error) => panic!("Temporarily.")
    };
    implement_unit(&attributes, &function)
}


/// Macro doc
#[proc_macro_attribute]
pub fn integration(attr: TokenStream, item: TokenStream) -> TokenStream {
    let function = parse(item).expect("Unable to parse function.");
    // may very well be empty.
    let attributes = match parse(attr){
        Ok(it) => it,
        // substitue with empty parents (I'd guess.)
        Err(_error) => panic!("Temporarily.")
    };

    implement_integration(&attributes, &function)
}