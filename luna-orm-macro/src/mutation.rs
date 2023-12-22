use proc_macro::{self, TokenStream};
use quote::quote;
use quote::quote_spanned;

use crate::utils::*;
use proc_macro2::{Ident, Span};
use syn::Attribute;
use syn::Field;
use syn::{
    parse_macro_input, token, Data, DataEnum, DataStruct, DataUnion, DeriveInput, Error, Fields,
    FieldsNamed, LitStr, Path, Result,
};

pub fn impl_mutation_macro(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    let fields = extract_fields(&data).unwrap();
    //let args_push_clause = build_args_push_clause(&fields);
    let args_push_ref_clause = build_args_add_option_ref_clause(&fields);
    let fields_name = extract_fields_name(&fields);

    let output = quote! {
        impl Mutation for #ident {
            fn get_fields_name(&self) -> Vec<String> {
                vec![
                    #(#fields_name, )*
                ]
            }

            fn any_arguments(&self) -> AnyArguments<'_> {
                let mut arguments = AnyArguments::default();
                #(#args_push_ref_clause ;)*
                return arguments;
            }
        }
    };

    output.into()
}
