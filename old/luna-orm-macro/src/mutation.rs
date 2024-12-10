use proc_macro::{self, TokenStream};
use quote::quote;
use quote::quote_spanned;

use crate::field_utils::map_field;
use crate::field_utils::map_field_vec;
use crate::field_utils::map_fields;
use crate::field_utils::FieldMapType;
use crate::fields_parser::FieldsParser;
use crate::type_check::field_is_option;
use crate::type_check::type_is_option;
use crate::utils::*;
use case::CaseExt;
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
    //let args_push_ref_clause = build_args_add_option_ref_clause(&fields);
    //let fields_name = extract_fields_name(&fields);
    let parser = FieldsParser::from_named(&fields);
    let fields_tokens = parser.get_option_fields();
    let fields_name = parser.get_option_name_vec();
    let option_args_stmt = parser.get_option_args();

    let output = quote! {
        impl Mutation for #ident {
            fn get_fields_name(&self) -> Vec<String> {
                #fields_name
            }

            fn any_arguments(&self) -> AnyArguments<'_> {
                #option_args_stmt
            }
        }
    };

    output.into()
}

pub fn generate_mutation(table_name: &str, fields: &Vec<Field>) -> proc_macro2::TokenStream {
    let mutation_name = format!("{}Mutation", table_name.to_camel());
    let mutation_ident = Ident::new(&mutation_name, Span::call_site());

    let parser = FieldsParser::from_vec(fields);
    let fields_tokens = parser.get_option_fields();
    let fields_name = parser.get_option_name_vec();
    let option_args_stmt = parser.get_option_args();

    let output = quote!(
        #[derive(Default, Debug, Clone, PartialEq, Eq)]
        pub struct #mutation_ident {
            #fields_tokens
        }

        impl Mutation for #mutation_ident {
            fn get_fields_name(&self) -> Vec<String> {
                #fields_name
            }

            fn any_arguments(&self) -> AnyArguments<'_> {
                #option_args_stmt
            }
        }
    );

    //panic!("{}", output);
    output
}
