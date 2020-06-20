//! Small macros to support writing tests.
//!
//! TODO: What else can I really write?
extern crate proc_macro;
use proc_macro::TokenStream;
mod naming;

/// Mark a test as a unit test.
///
/// The `unit` procedural macro simply renames the test function from
/// `fname` to `fname_unit`. This way, all unit tests can be run
/// independently of where they have been defined by invoking `cargo test`
/// with `unit` as so:
///
/// ```bash
/// cargo test unit
/// ```
///
/// `unit` can be applied to a single `fn` as well as a `mod`. In the latter, it
/// traverses all `fn`s and applies `unit` to them.
///
/// ```rust
/// # #[macro_use] extern crate suptest;
/// #[unit]
/// fn another_complex_test_case(){
///     assert_eq!(42, 42);
/// }
///
/// #[unit]
/// mod tests{
///     fn running_out_of_complex_test_cases(){
///          assert_eq!("42", "42");
///     }
///
///     fn the_sheer_complexity_of_this_test_case(){
///         assert_eq(vec![42], vec![42]);
///     }
/// }
/// ```
///
/// `#[unit]` applied to a function applies the `#[test]` attribute to it and
/// does not strip it of any other attributes applied. Thats is, adding
/// `#[ignore]` and/or `#[should_panic]` continues
/// working without issue:
///
/// ```rust
/// # #[macro_use] extern crate suptest;
/// #[unit]
/// #[ignore]
/// fn super_important_but_ignore(){
///     assert_ne!("Is this", "super important?");
/// }
///
/// #[unit]
/// #[should_panic]
/// fn ultra_super_test(){
///     panic!("I can't handle how super I am");
/// }
/// ```
///
/// Similarly, `#[unit]` for a `mod` applies `#[cfg(test)]` to it in order
/// for the testing facility to pick it up.
///
#[proc_macro_attribute]
pub fn unit(attr: TokenStream, item: TokenStream) -> TokenStream {
    naming::impl_rename(attr, item, naming::NameType::Unit)
}

/// Mark a test as an integration test.
///
/// The `integration` procedural macro simply renames the test function from
/// `fname` to `fname_integration`. This way, all integration tests can be run
/// independently of where they have been defined by invoking `cargo test`
/// with `integration` as so:
///
/// ```bash
/// cargo test unit
/// ```
///
/// `integration` can be applied to a single `fn` as well as a `mod`. In the latter, it
/// traverses all `fn`s and applies `unit` to them. For example:
///
/// ```rust
/// # #[macro_use] extern crate suptest;
/// #[integration]
/// fn my_complex_test_case(){
///     assert_eq!(42, 42);
/// }
///
/// #[integration]
/// mod tests{
///     fn my_other_complex_test(){
///          assert_eq!("42", "42");
///     }
///
///     fn getting_more_complex(){
///         assert_eq(vec![42], vec![42]);
///     }
/// }
/// ```
///
/// `#[unit]` applied to a function applies the `#[test]` attribute to it and
/// does not strip it of any other attributes applied. Thats is, adding
/// `#[ignore]` and/or `#[should_panic]` continues
/// working without issue:
///
/// ```rust
/// # #[macro_use] extern crate suptest;
/// #[integration]
/// #[ignore]
/// fn not_at_all_super(){
///     assert_ne!("One day", "I'll be super.");
/// }
///
/// #[integration]
/// #[should_panic]
/// fn minus_infinity_of_being_super(){
///     panic!("Nothing else left but to panic.");
/// }
/// ```
///
/// Similarly, `#[unit]` for a `mod` applies `#[cfg(test)]` to it in order
/// for the testing facility to pick it up.
///
#[proc_macro_attribute]
pub fn integration(attr: TokenStream, item: TokenStream) -> TokenStream {
    naming::impl_rename(attr, item, naming::NameType::Integration)
}