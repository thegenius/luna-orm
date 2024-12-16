use proc_macro2::TokenStream;
use syn::{Field, FieldsNamed};

use quote::quote_spanned;

use syn::LitStr;

use crate::types::{DefaultTypeChecker, TypeChecker};

pub struct DefaultFieldMapper {}
impl FieldMapper for DefaultFieldMapper {}

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

pub trait FieldMapper {
    fn map_to_any_args_add(field: Field) -> TokenStream {
        let field_name = field.ident.unwrap();
        let span = field_name.span();
        let field_type = field.ty;
        if <DefaultTypeChecker as TypeChecker>::type_is_option(&field_type) {
            quote_spanned! { span =>
                if let Some(#field_name) = &self.#field_name {
                    luna_add_arg(&mut arguments, &#field_name);
                }
            }
        } else {
            quote_spanned! { span =>
                luna_add_arg(&mut arguments, &self.#field_name);
            }
        }
    }

    fn map_to_args_add(field: Field) -> TokenStream {
        let field_name = field.ident.unwrap();
        let span = field_name.span();
        let field_type = field.ty;
        if <DefaultTypeChecker as TypeChecker>::type_is_option(&field_type) {
            quote_spanned! { span =>
                if let Some(#field_name) = &self.#field_name {
                    arguments.add(&self.#field_name);
                }
            }
        } else {
            quote_spanned! { span =>
                arguments.add(&self.#field_name);
            }
        }
    }

    fn map_to_selected_field(field: Field) -> TokenStream {
        let field_name = field.ident.unwrap();
        let span = field_name.span();
        let field_name_lit = LitStr::new(&field_name.to_string(), span);
        let field_type = field.ty;
        if <DefaultTypeChecker as TypeChecker>::type_is_option(&field_type) {
            quote_spanned! { span =>
                let #field_name: #field_type = row.try_get(#field_name_lit).ok();
            }
        } else {
            quote_spanned! { span =>
                let #field_name: Option<#field_type> = row.try_get(#field_name_lit).ok();
            }
        }
    }

    fn map_to_where_field(field: Field) -> TokenStream {
        let field_name = field.ident.unwrap();
        let span = field_name.span();
        let field_name_lit = LitStr::new(&field_name.to_string(), span);
        quote_spanned! { span =>
            if let Some(#field_name) = &self.#field_name {
                sql.push(wrap_char);
                sql.push_str(#field_name_lit);
                sql.push(wrap_char);
                sql.push_str(#field_name.cmp.get_sql());
                sql.push(place_holder);
            }
        }
    }

    fn map_field(field: Field, map_type: FieldMapType) -> TokenStream {
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
                    if self.#field_name.is_some() {
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

    fn map_fields<F>(field_list: &FieldsNamed, wrap_fn: &F) -> Vec<proc_macro2::TokenStream>
    where
        F: Fn(Field) -> proc_macro2::TokenStream,
    {
        let cloned_names = field_list.named.clone();
        cloned_names
            .into_iter()
            .map(wrap_fn)
            .collect::<Vec<TokenStream>>()
    }

    fn map_field_vec<F>(field_list: &Vec<Field>, wrap_fn: &F) -> Vec<proc_macro2::TokenStream>
    where
        F: Fn(Field) -> proc_macro2::TokenStream,
    {
        let cloned_names = field_list.clone();
        cloned_names
            .into_iter()
            .map(wrap_fn)
            .collect::<Vec<TokenStream>>()
    }
}
