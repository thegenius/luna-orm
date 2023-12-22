use proc_macro2::{self, Ident, Span, TokenStream};
use quote::quote_spanned;
use syn::{
    punctuated::Punctuated,
    token::{self, Comma},
    Attribute, Data, DataEnum, DataStruct, DataUnion, Error, Field, Fields, FieldsNamed, LitStr,
    Path, Result, Type, Variant,
};

pub fn type_has_prefix(ty: &Type, name: &str) -> bool {
    match ty {
        Type::Path(type_path) => {
            let idents_of_path =
                type_path
                    .path
                    .segments
                    .iter()
                    .fold(String::new(), |mut acc, v| {
                        acc.push_str(&v.ident.to_string());
                        acc.push_str("::");
                        acc
                    });
            idents_of_path.starts_with(name)
        }
        _ => false,
    }
}

pub fn get_field_type_name(field: &Field) -> String {
    let ty = &field.ty;
    match ty {
        Type::Path(type_path) => {
            let idents_of_path =
                type_path
                    .path
                    .segments
                    .iter()
                    .fold(String::new(), |mut acc, v| {
                        acc.push_str(&v.ident.to_string());
                        acc.push_str("::");
                        acc
                    });
            idents_of_path
        }
        _ => "".to_string(),
    }
}

pub fn type_has_one_of_names(ty: &Type, names: &[&str]) -> bool {
    names.iter().any(|name| type_has_prefix(ty, name))
}

pub fn type_is_option(ty: &Type) -> bool {
    type_has_one_of_names(
        ty,
        &[
            "Option::",
            "std::option::Option::",
            "core::option::Option::",
        ],
    )
}
pub fn field_is_option(field: &Field) -> bool {
    type_is_option(&field.ty)
}

pub fn field_is_type_of(field: &Field, type_name: &str) -> bool {
    let ty: &Type = &field.ty;
    type_has_prefix(ty, type_name)
}
