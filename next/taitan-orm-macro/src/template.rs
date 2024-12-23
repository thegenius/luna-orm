use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use crate::expands::generate_template_struct_and_impl;
use crate::util::{extract_fields, extract_generic_lifetimes};


pub fn impl_template_macro(input: TokenStream) -> TokenStream {
    let DeriveInput {
        attrs, ident, data, generics, ..
    } = parse_macro_input!(input);

    let lifetimes = extract_generic_lifetimes(generics);
    // panic!("{:?}", lifetimes);

    let fields = extract_fields(&data).unwrap();
    let output = generate_template_struct_and_impl(&ident, &attrs, &fields, &lifetimes);
    // panic!("{}", output);
    output.into()
}