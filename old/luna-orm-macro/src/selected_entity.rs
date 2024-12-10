use proc_macro::{self, TokenStream};
use quote::quote;

use crate::fields_parser::FieldsParser;
use crate::utils::*;
use case::CaseExt;
use proc_macro2::{Ident, Span};
use syn::Field;
use syn::{parse_macro_input, DeriveInput};

pub fn impl_selected_entity_macro(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    let fields = extract_fields(&data).unwrap();
    let row_construct = FieldsParser::from_named(&fields).get_row_construct();

    let output = quote! {
        impl SelectedEntity for #ident {
            fn from_any_row(row: AnyRow) -> Result<Self, SqlxError> where Self: Sized {
                #row_construct
            }
        }
    };
    // panic!("{}", output);
    output.into()
}

pub fn generate_selected_entity(table_name: &str, fields: &Vec<Field>) -> proc_macro2::TokenStream {
    let selected_name = format!("{}SelectedEntity", table_name.to_camel());
    let selected_ident = Ident::new(&selected_name, Span::call_site());

    let parser = FieldsParser::from_vec(fields);
    let option_fields = parser.get_option_fields();
    let row_construct = parser.get_row_construct();

    let output = quote!(
        #[derive(Default, Debug, Clone, PartialEq, Eq)]
        pub struct #selected_ident {
            #option_fields
        }

        impl SelectedEntity for #selected_ident {
            fn from_any_row(row: AnyRow) -> Result<Self, SqlxError> where Self: Sized {
                #row_construct
            }
        }
    );

    //panic!("{}", output);
    output
}
