use proc_macro::{self, TokenStream};
use quote::quote;
use quote::quote_spanned;

use crate::field_utils::map_field;
use crate::field_utils::map_field_vec;
use crate::field_utils::map_fields;
use crate::field_utils::FieldMapType;
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

pub fn impl_selected_entity_macro(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    let fields = extract_fields(&data).unwrap();

    let get_statement_members = map_fields(&fields, &|field: Field| {
        if field_is_option(&field) {
            map_field(field, FieldMapType::RowGetOption)
        } else {
            map_field(field, FieldMapType::RowGet)
        }
    });

    let field_members = fields.named.into_iter().map(|field| {
        let field_name = field.ident.unwrap();
        let span = field_name.span();
        quote_spanned! { span =>
            #field_name
        }
    });

    let output = quote! {
        impl SelectedEntity for #ident {
            fn from_any_row(row: AnyRow) -> Result<Self, SqlxError> where Self: Sized {
                #(#get_statement_members ;)*
                let result = #ident{ #(#field_members ,)*  };
                return Ok(result);
            }
        }
    };
    // panic!("{}", output);
    output.into()
}

pub fn generate_selected_entity(table_name: &str, fields: &Vec<Field>) -> proc_macro2::TokenStream {
    let selected_name = format!("{}SelectedEntity", table_name.to_camel());
    let selected_ident = Ident::new(&selected_name, Span::call_site());
    let fields_tokens = map_field_vec(fields, &|field| {
        let field_ident = field.ident;
        let field_ty = field.ty;
        if type_is_option(&field_ty) {
            quote!(
                #field_ident: #field_ty
            )
        } else {
            quote!(
                #field_ident: Option<#field_ty>
            )
        }
    });

    let get_statement_members = map_field_vec(fields, &|field: Field| {
        let field_name = field.ident.unwrap();
        let span = field_name.span();
        let field_name_stringified = LitStr::new(&field_name.to_string(), span);
        let field_type = field.ty;
        if type_is_option(&field_type) {
            quote_spanned! { span =>
                let #field_name: #field_type = row.try_get(#field_name_stringified).ok();
            }
        } else {
            quote_spanned! { span =>
                let #field_name: Option<#field_type> = row.try_get(#field_name_stringified).ok();
            }
        }
    });

    let field_members = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let span = field_name.span();
        quote_spanned! { span =>
            #field_name
        }
    });

    let output = quote!(
        #[derive(Default, Debug, Clone, PartialEq, Eq)]
        pub struct #selected_ident {
            #(#fields_tokens, )*
        }

        impl SelectedEntity for #selected_ident {
            fn from_any_row(row: AnyRow) -> Result<Self, SqlxError> where Self: Sized {
                #(#get_statement_members ;)*
                let result = #selected_ident{ #(#field_members ,)*  };
                return Ok(result);
            }
        }
    );

    //panic!("{}", output);
    output
}
