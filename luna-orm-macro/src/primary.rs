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

pub fn impl_primary_macro(input: TokenStream) -> TokenStream {
    let DeriveInput {
        attrs, ident, data, ..
    } = parse_macro_input!(input);
    let fields = extract_fields(&data).unwrap();
    let args_push_clause = build_args_push_clause(&fields);
    let args_add_clause = build_args_add_ref_clause(&fields);
    let fields_name = extract_fields_name(&fields);
    let fields_name_str = extract_fields_name_str(&fields);
    let table_name = extract_table_name(&ident, &attrs);

    let output = quote! {

        impl Primary for #ident {

            fn get_table_name(&self) -> &'static str {
                #table_name
            }

            fn get_primary_field_names(&self) -> &'static [&'static str] {
                &[ #(#fields_name_str)* ]
            }

            fn name(&self) -> String {
                String::from(#table_name)
            }

            fn get_fields_name(&self) -> Vec<String> {
                vec![ #(#fields_name, )* ]
            }

            fn any_arguments(&self) -> sqlx::any::AnyArguments<'_> {
                let mut arguments = AnyArguments::default();
                #(#args_add_clause;)*
                return arguments;
            }
        }
    };

    //panic!("{}", output);
    output.into()
}
