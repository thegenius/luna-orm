use proc_macro::{self, TokenStream};
use quote::quote;
use quote::quote_spanned;

use crate::field_utils::{map_field, map_field_vec, FieldMapType};
use crate::type_check::*;
use crate::type_extract::*;
use crate::utils::*;
use case::CaseExt;
use proc_macro2::{Ident, Span};
use syn::Attribute;
use syn::Field;
use syn::{
    parse_macro_input, token, Data, DataEnum, DataStruct, DataUnion, DeriveInput, Error, Fields,
    FieldsNamed, LitStr, Path, Result,
};
pub fn impl_selection_macro(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    let fields = extract_fields(&data).unwrap();
    let fields_name = extract_selected_fields_name(&fields);

    let output = quote! {
        impl Selection for #ident {
            fn get_selected_fields(&self) -> Vec<String> {
                let mut fields = Vec::new();
                #(#fields_name)*
                return fields;
            }
        }
    };
    output.into()
}

pub fn generate_selection(table_name: &str, fields: &Vec<Field>) -> proc_macro2::TokenStream {
    let selection_name = format!("{}Selection", table_name.to_camel());
    let selection_ident = Ident::new(&selection_name, Span::call_site());
    let fields_tokens = map_field_vec(fields, &|field| {
        let field_ident = field.ident;
        quote!(
            #field_ident: bool
        )
    });

    let fields_name = map_field_vec(fields, &|field: Field| {
        map_field(field, FieldMapType::BoolPush)
    });

    let output = quote!(
        #[derive(Default, Clone)]
        pub struct #selection_ident {
            #(#fields_tokens, )*
        }

        impl Selection for #selection_ident {
            fn get_selected_fields(&self) -> Vec<String> {
                let mut fields = Vec::new();
                #(#fields_name)*
                return fields;
            }
        }
    );

    //panic!("{}", output);
    output
}
