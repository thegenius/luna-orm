use crate::type_check::field_is_option;
use proc_macro2::{self, Ident, Span, TokenStream};
use quote::quote;
use quote::quote_spanned;
use syn::{
    punctuated::Punctuated,
    token::{self, Comma},
    Attribute, Data, DataEnum, DataStruct, DataUnion, Error, Field, Fields, FieldsNamed, LitStr,
    Path, Result, Variant,
};

pub enum FieldMapType {
    Str,
    String,
    OptionBoolPush,
    NamePush,
    OptionNamePush,
    BoolPush,

    RowGet,
    RowGetOption,

    ArgsAdd,
    ArgsAddRef,
    ArgsAddOptionRef,
    ArgsAddOptionLocationRef,
    ArgsAddClone,
}

pub fn map_field(field: Field, map_type: FieldMapType) -> TokenStream {
    let field_name = field.ident.unwrap();
    let span = field_name.span();
    let field_name_stringified = LitStr::new(&field_name.to_string(), span);
    let field_type = field.ty;
    match map_type {
        FieldMapType::Str => {
            quote_spanned! { span=> #field_name_stringified }
        }
        FieldMapType::String => {
            quote_spanned! { span=> #field_name_stringified.to_string() }
        }
        FieldMapType::OptionBoolPush => {
            quote_spanned! { span=>
                if let Some(true) = self.#field_name {
                    fields.push(#field_name_stringified.to_string());
                }
            }
        }

        FieldMapType::NamePush => {
            quote_spanned! { span=>
                fields.push(#field_name_stringified.to_string());
            }
        }

        FieldMapType::OptionNamePush => {
            quote_spanned! { span=>
                if let Some(_) = self.#field_name {
                    fields.push(#field_name_stringified.to_string());
                }
            }
        }

        FieldMapType::BoolPush => {
            quote_spanned! { span=>
                if self.#field_name {
                    fields.push(#field_name_stringified.to_string());
                }
            }
        }

        FieldMapType::RowGet => {
            quote_spanned! { span =>
                let #field_name: #field_type = row.try_get(#field_name_stringified)?;
            }
        }

        FieldMapType::RowGetOption => {
            quote_spanned! { span =>
                let #field_name: #field_type = row.try_get(#field_name_stringified).ok();
            }
        }

        FieldMapType::ArgsAdd => {
            quote_spanned! { span =>
                arguments.add(self.#field_name);
            }
        }
        FieldMapType::ArgsAddRef => {
            quote_spanned! { span =>
                luna_add_arg(&mut arguments, &self.#field_name);
            }
        }
        FieldMapType::ArgsAddOptionRef => {
            quote_spanned! { span =>
                if let Some(#field_name) = &self.#field_name {
                    luna_add_arg(&mut arguments, &#field_name);
                }
            }
        }
        FieldMapType::ArgsAddOptionLocationRef => {
            quote_spanned! { span =>
                if let Some(#field_name) = &self.#field_name {
                    luna_add_arg(&mut arguments, &#field_name.val);
                }
            }
        }
        FieldMapType::ArgsAddClone => {
            quote_spanned! { span=>
                arguments.add(self.#field_name.clone());
            }
        }
    }
}

pub fn map_fields<F>(field_list: &FieldsNamed, wrap_fn: &F) -> Vec<proc_macro2::TokenStream>
where
    F: Fn(Field) -> proc_macro2::TokenStream,
{
    let cloned_names = field_list.named.clone();
    cloned_names
        .into_iter()
        .map(wrap_fn)
        .collect::<Vec<TokenStream>>()
}

pub fn map_field_vec<F>(field_list: &Vec<Field>, wrap_fn: &F) -> Vec<proc_macro2::TokenStream>
where
    F: Fn(Field) -> proc_macro2::TokenStream,
{
    let cloned_names = field_list.clone();
    cloned_names
        .into_iter()
        .map(wrap_fn)
        .collect::<Vec<TokenStream>>()
}
