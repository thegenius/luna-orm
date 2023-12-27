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

pub fn impl_mutation_macro(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    let fields = extract_fields(&data).unwrap();
    //let args_push_clause = build_args_push_clause(&fields);
    let args_push_ref_clause = build_args_add_option_ref_clause(&fields);
    let fields_name = extract_fields_name(&fields);

    let output = quote! {
        impl Mutation for #ident {
            fn get_fields_name(&self) -> Vec<String> {
                vec![
                    #(#fields_name, )*
                ]
            }

            fn any_arguments(&self) -> AnyArguments<'_> {
                let mut arguments = AnyArguments::default();
                #(#args_push_ref_clause ;)*
                return arguments;
            }
        }
    };

    output.into()
}

pub fn generate_mutation(table_name: &str, fields: &Vec<Field>) -> proc_macro2::TokenStream {
    let mutation_name = format!("{}Mutation", table_name.to_camel());
    let mutation_ident = Ident::new(&mutation_name, Span::call_site());
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

    let args_push_ref_clause = map_field_vec(fields, &|field: Field| {
        let field_name = field.ident.unwrap();
        let span = field_name.span();
        let field_name_stringified = LitStr::new(&field_name.to_string(), span);
        let field_type = field.ty;
        quote_spanned! { span =>
            if let Some(#field_name) = &self.#field_name {
                luna_add_arg(&mut arguments, &#field_name);
            }
        }
    });

    let fields_name = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let span = field_name.span();
        let field_name_stringified = LitStr::new(&field_name.to_string(), span);
        quote_spanned! { span =>
                if let Some(_) = self.#field_name {
                    fields.push(#field_name_stringified.to_string());
                }
        }
    });

    let output = quote!(
        #[derive(Default, Debug, Clone, PartialEq, Eq)]
        pub struct #mutation_ident {
            #(#fields_tokens, )*
        }

        impl Mutation for #mutation_ident {
            fn get_fields_name(&self) -> Vec<String> {
                let mut fields = Vec::new();
                #(#fields_name)*
                return fields;
            }

            fn any_arguments(&self) -> AnyArguments<'_> {
                let mut arguments = AnyArguments::default();
                #(#args_push_ref_clause ;)*
                return arguments;
            }
        }
    );

    //panic!("{}", output);
    output
}
