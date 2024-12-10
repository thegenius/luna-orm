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
use crate::type_extract::get_option_inner_type;
use crate::utils::*;
use case::CaseExt;
use luna_orm_trait::array_str_equal;
use proc_macro2::{Ident, Span};
use syn::Attribute;
use syn::Field;
use syn::{
    parse_macro_input, token, Data, DataEnum, DataStruct, DataUnion, DeriveInput, Error, Fields,
    FieldsNamed, LitStr, Path, Result,
};

fn generate_impl(
    table_name: &str,
    fields: &Vec<Field>,
    attrs: &Vec<Attribute>,
) -> proc_macro2::TokenStream {
    let unique_indexes = extract_unique_index(attrs);
    let parser = FieldsParser::from_vec(fields);
    let build_args_stmt = parser.get_option_location_args();
    let where_stmt = parser.get_where_clause();
    let unique_index_check = parser.get_unique_index_check(&unique_indexes);
    let fields_name = parser.get_option_name_vec();

    let output = quote! {

            fn get_table_name(&self) -> &'static str {
                #table_name
            }


            fn get_fields_name(&self) -> Vec<String> {
                #fields_name
            }


            fn check_valid_order_by(&self, fields: &[&str]) -> bool {
                #unique_index_check
            }

            fn get_where_clause(&self, wrap_char: char, place_holder: char) -> String {
                #where_stmt
            }

            fn any_arguments(&self) -> AnyArguments<'_> {
                #build_args_stmt
            }
    };
    output
}

pub fn impl_location_macro(input: TokenStream) -> TokenStream {
    let DeriveInput {
        attrs, ident, data, ..
    } = parse_macro_input!(input);

    let fields = extract_fields(&data).unwrap();
    let fields = fields.named.into_iter().collect::<Vec<Field>>();
    let table_name = extract_table_name(&ident, &attrs);
    let generated_impl = generate_impl(&table_name, &fields, &attrs);

    let output = quote! {

        impl Location for #ident {
            #generated_impl
        }
    };

    // panic!("{}", output);
    output.into()
}

pub fn generate_location(
    table_name: &str,
    fields: &Vec<Field>,
    attrs: &Vec<Attribute>,
) -> proc_macro2::TokenStream {
    let location_name = format!("{}Location", table_name.to_camel());
    let location_ident = Ident::new(&location_name, Span::call_site());
    let fields_tokens = FieldsParser::from_vec(fields).get_location_expr_fields();
    let generated_impl = generate_impl(table_name, fields, attrs);

    quote!(
        #[derive(Clone, Debug)]
        pub struct #location_ident {
            #fields_tokens
        }

        impl Location for #location_ident {
            #generated_impl
        }
    )
}
