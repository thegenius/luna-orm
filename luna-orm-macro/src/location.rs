use proc_macro::{self, TokenStream};
use quote::quote;
use quote::quote_spanned;

use crate::field_utils::map_field;
use crate::field_utils::map_field_vec;
use crate::field_utils::map_fields;
use crate::field_utils::FieldMapType;
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

pub fn impl_location_macro(input: TokenStream) -> TokenStream {
    let DeriveInput {
        attrs, ident, data, ..
    } = parse_macro_input!(input);

    let fields = extract_fields(&data).unwrap();
    // let args_push_clause = build_args_push_clause(&fields);
    let fields_name = extract_fields_name(&fields);
    let cloned_named = fields.named.clone();
    let arguments_ref_expanded_members = build_args_add_option_location_ref_clause(&fields);

    let cloned_named = fields.named.clone();
    let where_clause_members = cloned_named.into_iter().map(|field| {
        let field_name = field.ident.unwrap();
        let span = field_name.span();
        let field_name_stringified = LitStr::new(&field_name.to_string(), span);
        quote_spanned! { span=>
            if let Some(#field_name) = &self.#field_name {
                sql.push_str(#field_name_stringified);
                sql.push_str(#field_name.cmp.get_sql());
                sql.push(place_holder);
            }
        }
    });

    let table_name = extract_table_name(&ident, &attrs);
    let unique_indexes = extract_unique_index(&attrs);

    let unique_index_check = if unique_indexes.is_empty() {
        quote!(fields.len() == 0)
    } else {
        let mut check_token_stream = quote!();
        for unique_index in unique_indexes {
            let check_token = quote!(
                if array_str_equal(&[#(#unique_index,)*], fields) {
                    return true;
                }
            );
            check_token_stream.extend(check_token);
        }
        check_token_stream.extend(quote!(

            dbg!(&fields);
            return false;
        ));
        check_token_stream
    };

    let output = quote! {

        impl Location for #ident {

            fn get_table_name(&self) -> &'static str {
                #table_name
            }


            fn get_fields_name(&self) -> Vec<String> {
                vec![
                    #(#fields_name ,)*
                ]
            }


            fn check_valid_order_by(&self, fields: &[&str]) -> bool {
                #unique_index_check
            }

            fn get_where_clause(&self, wrap_char: char, place_holder: char) -> String {
                let mut sql = String::default();
                #(#where_clause_members )*
                return sql;
            }

            fn any_arguments(&self) -> AnyArguments<'_> {
                let mut arguments = AnyArguments::default();
                #(#arguments_ref_expanded_members ;)*
                return arguments;
            }
        }
    };

    // panic!("{}", output);
    output.into()
}

pub fn generate_location(
    table_name: &str,
    fields: &Vec<Field>,
    unique_indexes: Vec<Vec<String>>,
) -> proc_macro2::TokenStream {
    let location_name = format!("{}Location", table_name.to_camel());
    let location_ident = Ident::new(&location_name, Span::call_site());
    let fields_tokens = map_field_vec(fields, &|field| {
        let field_ident = field.ident;
        let field_ty = field.ty;
        if type_is_option(&field_ty) {
            let inner_type = get_option_inner_type(&field_ty);
            quote!(
                #field_ident: Option<LocationExpr<#inner_type>>
            )
        } else {
            quote!(
                #field_ident: Option<LocationExpr<#field_ty>>
            )
        }
    });

    let fields_name = map_field_vec(fields, &|field| {
        let field_name = field.ident.unwrap();
        let span = field_name.span();
        let field_name_stringified = LitStr::new(&field_name.to_string(), span);
        quote_spanned! { span=> #field_name_stringified.to_string() }
    });

    let unique_index_check = if unique_indexes.is_empty() {
        quote!(fields.len() == 0)
    } else {
        let mut check_token_stream = quote!();
        for unique_index in unique_indexes {
            let check_token = quote!(
                if array_str_equal(&[#(#unique_index,)*], fields) {
                    return true;
                }
            );
            check_token_stream.extend(check_token);
        }
        check_token_stream.extend(quote!(

            dbg!(&fields);
            return false;
        ));
        check_token_stream
    };

    let where_clause_members = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let span = field_name.span();
        let field_name_stringified = LitStr::new(&field_name.to_string(), span);
        quote_spanned! { span=>
            if let Some(#field_name) = &self.#field_name {
                sql.push_str(#field_name_stringified);
                sql.push_str(#field_name.cmp.get_sql());
                sql.push(place_holder);
            }
        }
    });

    let arguments_ref_expanded_members = map_field_vec(fields, &|field| {
        map_field(field, FieldMapType::ArgsAddOptionLocationRef)
    });

    quote!(
        #[derive(Clone, Debug)]
        pub struct #location_ident {
            #(#fields_tokens, )*
        }

        impl Location for #location_ident {

            fn get_table_name(&self) -> &'static str {
                #table_name
            }


            fn get_fields_name(&self) -> Vec<String> {
                vec![
                    #(#fields_name ,)*
                ]
            }


            fn check_valid_order_by(&self, fields: &[&str]) -> bool {
                #unique_index_check
            }

            fn get_where_clause(&self, wrap_char: char, place_holder: char) -> String {
                let mut sql = String::default();
                #(#where_clause_members )*
                return sql;
            }

            fn any_arguments(&self) -> AnyArguments<'_> {
                let mut arguments = AnyArguments::default();
                #(#arguments_ref_expanded_members ;)*
                return arguments;
            }
        }
    )
}
