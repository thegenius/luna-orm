use proc_macro2::TokenStream;
use syn::Field;

use crate::type_check::type_is_option;
use quote::{quote, quote_spanned};

use syn::LitStr;

pub struct FieldMapper {}

impl FieldMapper {
    pub fn map_to_any_args_add(field: Field) -> TokenStream {
        let field_name = field.ident.unwrap();
        let span = field_name.span();
        let field_type = field.ty;
        if type_is_option(&field_type) {
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

    pub fn map_to_args_add(field: Field) -> TokenStream {
        let field_name = field.ident.unwrap();
        let span = field_name.span();
        let field_type = field.ty;
        if type_is_option(&field_type) {
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


    pub fn map_to_selected_field(field: Field) -> TokenStream {
        let field_name = field.ident.unwrap();
        let span = field_name.span();
        let field_name_lit = LitStr::new(&field_name.to_string(), span);
        let field_type = field.ty;
        if type_is_option(&field_type) {
            quote_spanned! { span =>
                let #field_name: #field_type = row.try_get(#field_name_lit).ok();
            }
        } else {
            quote_spanned! { span =>
                let #field_name: Option<#field_type> = row.try_get(#field_name_lit).ok();
            }
        }
    }

    pub fn map_to_where_field(field: Field) -> TokenStream {
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
}
