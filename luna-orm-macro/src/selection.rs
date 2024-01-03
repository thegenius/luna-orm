use proc_macro::{self, TokenStream};
use quote::quote;

use crate::fields_parser::FieldsParser;
use crate::utils::*;
use case::CaseExt;
use proc_macro2::{Ident, Span};
use syn::Field;
use syn::{parse_macro_input, DeriveInput};

pub fn generate_impl(ident: &Ident, fields: &Vec<Field>) -> proc_macro2::TokenStream {
    let parser = FieldsParser::from_vec(fields);
    let fields_name = parser.get_bool_name_vec();
    quote! {
        impl Selection for #ident {
            fn get_selected_fields(&self) -> Vec<String> {
                #fields_name
            }
        }
    }
}

pub fn impl_selection_macro(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);
    let fields = extract_fields(&data).unwrap();
    let fields = fields.named.into_iter().collect();
    let output = generate_impl(&ident, &fields).into();
    //panic!("{}", output);
    output
}

pub fn generate_selection(table_name: &str, fields: &Vec<Field>) -> proc_macro2::TokenStream {
    let selection_name = format!("{}Selection", table_name.to_camel());
    let selection_ident = Ident::new(&selection_name, Span::call_site());

    let fields_tokens = FieldsParser::from_vec(fields).get_bool_fields();
    let generated_impl = generate_impl(&selection_ident, fields);

    let output = quote!(
        #[derive(Default, Clone)]
        pub struct #selection_ident {
            #fields_tokens
        }

        #generated_impl
    );

    //panic!("{}", output);
    output
}
