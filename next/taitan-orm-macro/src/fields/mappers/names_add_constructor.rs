use crate::types::{DefaultTypeChecker, TypeChecker};
use proc_macro2::TokenStream;
use quote::quote_spanned;
use syn::spanned::Spanned;
use syn::{Field, LitStr};

pub trait NamesAddConstructor {
    fn of_str(field: Field) -> TokenStream {
        let span = field.span();
        let field_name = field.ident.unwrap();
        let field_name_string = LitStr::new(&field_name.to_string(), span);
        quote_spanned! { span=> #field_name_string }
    }

    fn of_string(field: Field) -> TokenStream {
        let span = field.span();
        let field_name = field.ident.unwrap();
        let field_name_string = LitStr::new(&field_name.to_string(), span);
        quote_spanned! { span=> #field_name_string.to_string() }
    }

    fn of_maybe_option(field: Field) -> TokenStream {
        let field_type = &field.ty;
        let span = field.span();
        let field_name = field.ident.unwrap();
        let field_name_string = LitStr::new(&field_name.to_string(), span);
        if DefaultTypeChecker::type_is_option(field_type) {
            quote_spanned! { span=>
                if self.#field_name.is_some() {
                    fields.push(#field_name_string.to_string());
                }
            }
        } else {
            quote_spanned! { span=>
                fields.push(#field_name_string.to_string());
            }
        }
    }

    fn of_option(field: Field) -> TokenStream {
        let span = field.span();
        let field_name = field.ident.unwrap();
        let field_name_string = LitStr::new(&field_name.to_string(), span);
        quote_spanned! { span=>
            if let taitan_orm::Optional::Some(_) = self.#field_name {
                fields.push(#field_name_string.to_string());
            }
        }
    }

    fn of_not_option(field: Field) -> TokenStream {
        let span = field.span();
        let field_name = field.ident.unwrap();
        let field_name_string = LitStr::new(&field_name.to_string(), span);
        quote_spanned! { span=>
            fields.push(#field_name_string.to_string());
        }
    }

    fn of_option_bool(field: Field) -> TokenStream {
        let span = field.span();
        let field_name = field.ident.unwrap();
        let field_name_string = LitStr::new(&field_name.to_string(), span);
        quote_spanned! { span=>
            if let taitan_orm::Optional::Some(true) = self.#field_name {
                fields.push(#field_name_string.to_string());
            }
        }
    }
    fn of_bool(field: Field) -> TokenStream {
        let span = field.span();
        let field_name = field.ident.unwrap();
        let field_name_string = LitStr::new(&field_name.to_string(), span);
        quote_spanned! { span=>
            if self.#field_name {
                fields.push(#field_name_string.to_string());
            }
        }
    }

    fn of_where_seg(field: Field) -> TokenStream {
        let field_name = field.ident.unwrap();
        let span = field_name.span();
        let field_name_lit = LitStr::new(&field_name.to_string(), span);
        quote_spanned! { span =>
            if let taitan_orm::Optional::Some(#field_name) = &self.#field_name {
                sql.push(wrap_char);
                sql.push_str(#field_name_lit);
                sql.push(wrap_char);
                sql.push_str(#field_name.cmp.get_sql());
                sql.push(place_holder);
            }
        }
    }
}
