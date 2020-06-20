extern crate proc_macro;
extern crate alloc;
use alloc::string::ToString;
use proc_macro::{TokenStream};
use quote::{quote, format_ident};
use syn::{
    parse, parse::{Parser}, parse_macro_input,
    AttributeArgs, Attribute, Signature,
    Item, ItemFn, ItemMod
};


// For my own ease.
type TS = TokenStream;

// Handle cases - Unit and Integration.
#[derive(Clone, Copy)]
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


/// Make the attribute, seems like there should be a cleaner way to do this.
fn make_attr(name: &str) -> Vec<Attribute> {
    let id = format_ident!("{}", name);
    // Make into TokenStream with quote and parse it again.
    let attr = quote! {
        #[#id]
    }.into();

    Attribute::parse_outer.parse(attr).expect("The unexpected")
}

/// Apply rename_fn to all the fn's found in the module. Apply #[cfg(test)] to module.
fn handle_mod(attrs: AttributeArgs, module: ItemMod, mode: NameType) -> TS {
    // Match for the content (i.e anything defined in module).
    // empty content => module declaration.
    let (brace, content) = match module.content {
        // Grab the items
        Some(content) => content,
        None => {
            panic!("Cannot apply macro to mod declaration.");
        }
    };
    // Go through Items and build result. Non fn's should be ignored.
    let mut contents: Vec<Item> = Vec::new();
    for it in content {
        match it {
            // Call case to handle fn
            Item::Fn(it) => {
                let res = rename_fn(attrs.clone(), it, mode);
                contents.push(syn::Item::from(res));
            }
            // Others, just push them on contents.
            other => {
                contents.push(other);
            }
        }
    }
    // Create new module, add #[cfg(test)] and return.
    let resmod = ItemMod{
        content: Some((brace, contents)), ..module
    };
    (quote!{
        #[cfg(test)]
        #resmod
    }).into()
}


/// Transform name from fn_name to fn_name_mode.to_string
fn rename_fn(_attrs: AttributeArgs, function: ItemFn, mode: NameType) -> ItemFn {
    // Create the new identifier.
    let ident = format_ident!("{}_{}", function.sig.ident, mode.to_string());

    // add the #[test] attribute and any other attributes added to the function.
    let mut function_attributes= make_attr("test");
    function_attributes.extend(function.attrs);

    // create the new signature
    let sig = Signature{
        ident, ..function.sig
    };
    // and return the new itemfn
    ItemFn{sig, attrs: function_attributes,  ..function}
}


/// Implement renaming. If a mod, we rename all fn's if fn, simply rename that.
pub(crate) fn impl_rename(attrs: TS, f: TS, mode: NameType) -> TS {
    // Grab it as an item.
    let it: Item = parse(f).expect("Expected an item");
    let attributes: AttributeArgs = parse_macro_input!(attrs as AttributeArgs);
    match it {
        // Handle fn application.
        Item::Fn(it) => {
            let res = rename_fn(attributes, it, mode);
            (quote!{
                #res
            }).into()
        }
        // Handle module application.
        Item::Mod(it) => {
            handle_mod(attributes, it, mode)
        }
        // Can't handle this.
        _ => {
            panic!("Item must be mod or fn.");
        }
    }
}