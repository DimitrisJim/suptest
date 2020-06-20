extern crate proc_macro;
extern crate alloc;
use alloc::string::ToString;
use proc_macro::TokenStream;
use quote::quote;
use syn;

type TS = TokenStream;

// Handle cases - Unit and Integration.
pub(crate) enum NameType{
    Unit,
    Integration,
}

impl ToString for NameType {
    fn to_string(&self) -> String {
        match self {
            NameType::Unit => String::from("unit"),
            NameType::Integration => String::from("integration"),
        }
    }
}

pub(crate) fn impl_rename(attrs: TS, f: TS, mode: NameType) -> TS {
    // Eventually might need to support fn and mod
    let function: syn::ItemFn = syn::parse(f)
        .expect("Unable to parse function.");

    // may very well be empty
    let attributes = attrs.clone();
    println!("{}", attributes);
    println!("{}", mode.to_string());
    /*
    let attributes: syn::Ident = match syn::parse(attributes){
        Ok(it) => it,
        // substitute with empty parents (I'd guess.)
        Err(_error) => panic!("Temporarily.")
    };
    */
    let fid = quote::format_ident!("{}_{}", function.sig.ident, mode.to_string());

    let gen = quote! {
        #[test]
        fn #fid(){
            assert_eq!(2, 2)
        }
    };
    gen.into()
}