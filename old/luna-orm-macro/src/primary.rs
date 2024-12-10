use case::CaseExt;
use proc_macro::{self, TokenStream};
use quote::quote;

use crate::fields_parser::FieldsParser;
use crate::utils::*;

use proc_macro2::{Ident, Span};
use syn::Field;
use syn::{parse_macro_input, DeriveInput};

pub fn impl_primary_macro(input: TokenStream) -> TokenStream {
    let DeriveInput {
        attrs, ident, data, ..
    } = parse_macro_input!(input);
    let fields = extract_fields(&data).unwrap();
    let fields = fields.named.into_iter().collect::<Vec<Field>>();
    let table_name = extract_table_name(&ident, &attrs);
    let impl_token = generate_impl(&table_name, &fields);

    let output = quote! {

        impl Primary for #ident {
            #impl_token
        }
    };

    //panic!("{}", output);
    output.into()
}

pub fn generate_primary(table_name: &str, fields: &Vec<Field>) -> proc_macro2::TokenStream {
    let primary_name = format!("{}Primary", table_name.to_camel());
    let primary_ident = Ident::new(&primary_name, Span::call_site());
    let fields_tokens = FieldsParser::from_vec(fields).get_not_option_fields();
    let impl_token = generate_impl(table_name, fields);

    let output = quote!(
        #[derive(Default, Debug, Clone)]
        pub struct #primary_ident {
            #fields_tokens
        }

        impl Primary for #primary_ident {
            #impl_token
        }
    );

    //panic!("{}", output);
    output
}

fn generate_impl(table_name: &str, fields: &Vec<Field>) -> proc_macro2::TokenStream {
    let parser = FieldsParser::from_vec(fields);
    let fields_name = parser.get_name_array();
    let args_add_stmt = parser.get_args();
    let output = quote!(
            fn get_table_name(&self) -> &'static str {
                #table_name
            }

            fn get_primary_field_names(&self) -> &'static [&'static str] {
                #fields_name
            }

            fn any_arguments(&self) -> sqlx::any::AnyArguments<'_> {
                #args_add_stmt
            }
    );
    output
}
