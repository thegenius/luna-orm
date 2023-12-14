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

pub fn impl_entity_macro(input: TokenStream) -> TokenStream {
    let DeriveInput {
        attrs, ident, data, ..
    } = parse_macro_input!(input);

    let fields = extract_fields(&data).unwrap();

    let _fields_name = extract_fields_name(&fields);
    let primary_fields = extract_annotated_fields(&fields, "PrimaryKey");
    if primary_fields.is_empty() {
        panic!("Entity must has at least one PrimaryKey!")
    }
    let primary_fields_name = build_fields_name(&primary_fields);

    let body_fields = extract_not_annotated_fields(&fields, "PrimaryKey");
    let body_fields_name = build_fields_name(&body_fields);

    let name = extract_table_name(&ident, &attrs);
    let primary_args_add: Vec<proc_macro2::TokenStream> =
        build_args_add_clause(&primary_fields, false);

    let primary_args_add_ref: Vec<proc_macro2::TokenStream> =
        build_args_add_ref_clause_by_vec(&primary_fields);

    let body_args_add: Vec<proc_macro2::TokenStream> = build_args_add_clause(&body_fields, false);
    let body_args_add_cloned: Vec<proc_macro2::TokenStream> =
        build_args_add_clause(&body_fields, true);

    let body_args_add_ref: Vec<proc_macro2::TokenStream> =
        build_args_add_ref_clause_by_vec(&body_fields);

    let mut full_fields: Vec<Field> = Vec::new();
    full_fields.extend(primary_fields);
    full_fields.extend(body_fields);

    let clone_full_fields = full_fields.clone();
    let from_row_get_statement_members = clone_full_fields.into_iter().map(|field| {
        let field_name = field.ident.unwrap();
        let span = field_name.span();
        let field_name_str = LitStr::new(&field_name.to_string(), span);
        let field_type = field.ty;
        let span = field_name.span();
        quote_spanned! { span =>
            let #field_name: #field_type = row.try_get(#field_name_str)?;
        }
    });

    let clone_full_fields = full_fields.clone();
    let from_row_field_members = clone_full_fields.into_iter().map(|field| {
        let field_name = field.ident.unwrap();
        let span = field_name.span();
        quote_spanned! { span =>
            #field_name
        }
    });

    let output = quote! {
    impl Entity for #ident {
        fn name(&self) -> String {
            String::from(#name)
        }

        fn get_table_name(&self) -> &'static str {
                #name
        }

        fn from_any_row(row: AnyRow) -> Result<Self, SqlxError> where Self: Sized {
            #(#from_row_get_statement_members ;)*
            let result = #ident{ #(#from_row_field_members ,)*  };
            return Ok(result);
        }

        fn get_primary_fields_name(&self) -> Vec<String> {
            vec![
                #(#primary_fields_name, )*
            ]
        }

        fn get_body_fields_name(&self) -> Vec<String> {
            vec![
                #(#body_fields_name, )*
            ]
        }

        fn any_arguments_of_insert<'p>(&self) -> AnyArguments<'p> {
            let mut arguments = AnyArguments::default();
            #(#primary_args_add_ref; )*
            #(#body_args_add_ref; )*
            return arguments;
        }

        fn any_arguments_of_upsert<'p>(&self) -> AnyArguments<'p> {
            let mut arguments = AnyArguments::default();
            #(#primary_args_add_ref; )*
            #(#body_args_add_ref; )*
            #(#body_args_add_ref; )*
            return arguments;
        }

        fn any_arguments_of_update<'p>(&self) -> AnyArguments<'p> {
            let mut arguments = AnyArguments::default();
            #(#body_args_add_ref; )*
            #(#primary_args_add_ref; )*
            return arguments;
        }

        /*
        fn into_insert_any_arguments<'p>(self) -> AnyArguments<'p> where Self: Sized {
            let mut arguments = AnyArguments::default();
            #(#primary_args_add; )*
            #(#body_args_add; )*
            return arguments;
        }

        fn into_update_any_arguments<'p>(self) -> AnyArguments<'p> where Self: Sized {
            let mut arguments = AnyArguments::default();
            #(#body_args_add; )*
            #(#primary_args_add; )*
            return arguments;
        }

        fn into_upsert_any_arguments<'p>(self) -> AnyArguments<'p> where Self: Sized {
            let mut arguments = AnyArguments::default();
            #(#primary_args_add; )*
            #(#body_args_add_cloned; )*
            #(#body_args_add; )*
            return arguments;
        }
        */

    }
    };
    // panic!("{}", output);
    output.into()
}
