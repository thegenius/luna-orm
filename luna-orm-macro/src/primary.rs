use case::CaseExt;
use proc_macro::{self, TokenStream};
use quote::quote;
use quote::quote_spanned;

use crate::field_utils::{map_field, map_field_vec, FieldMapType};

use crate::type_check::*;
use crate::type_extract::*;
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
    let args_add_clause = build_args_add_ref_clause(&fields);
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

pub fn generate_primary(table_name: &str, fields: &Vec<Field>) -> proc_macro2::TokenStream {
    let primary_name = format!("{}Primary", table_name.to_camel());
    let primary_ident = Ident::new(&primary_name, Span::call_site());
    let fields_tokens = map_field_vec(fields, &|field| {
        let field_ident = field.ident;
        if type_is_option(&field.ty) {
            let inner_type = get_option_inner_type(&field.ty).unwrap();
            quote!(
                #field_ident: #inner_type
            )
        } else {
            let field_ty = field.ty;
            quote!(
                #field_ident: #field_ty
            )
        }
    });
    let impl_token = generate_impl(table_name, fields);

    let output = quote!(
        #[derive(Default, Clone)]
        pub struct #primary_ident {
            #(#fields_tokens, )*
        }

        impl Primary for #primary_ident {
            #impl_token
        }
    );

    //panic!("{}", output);
    output.into()
}

fn generate_impl(table_name: &str, fields: &Vec<Field>) -> proc_macro2::TokenStream {
    let args_add_clause = map_field_vec(fields, &|f: Field| map_field(f, FieldMapType::ArgsAddRef));
    let fields_name_str =
        map_field_vec(fields, &|field: Field| map_field(field, FieldMapType::Str));
    let output = quote!(
            fn get_table_name(&self) -> &'static str {
                #table_name
            }

            fn get_primary_field_names(&self) -> &'static [&'static str] {
                &[ #(#fields_name_str)* ]
            }

            fn any_arguments(&self) -> sqlx::any::AnyArguments<'_> {
                let mut arguments = AnyArguments::default();
                #(#args_add_clause;)*
                return arguments;
            }
    );
    output.into()
}
