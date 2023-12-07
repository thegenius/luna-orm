use proc_macro2::{self, Ident, Span, TokenStream};
use quote::quote_spanned;
use syn::{
    token, Attribute, Data, DataEnum, DataStruct, DataUnion, Error, Field, Fields, FieldsNamed,
    LitStr, Path, Result,
};

pub fn extract_fields(data: &Data) -> Result<FieldsNamed> {
    let fields = match data {
        Data::Enum(DataEnum {
            enum_token: token::Enum { span },
            ..
        })
        | Data::Union(DataUnion {
            union_token: token::Union { span },
            ..
        }) => {
            return Err(Error::new(*span, "Expected a `struct`"));
        }

        Data::Struct(DataStruct {
            fields: Fields::Named(it),
            ..
        }) => it,

        Data::Struct(_) => {
            return Err(Error::new(
                Span::call_site(),
                "Expected a `struct` with named fields",
            ));
        }
    };
    return Ok(fields.clone());
}

pub fn extract_fields_name_str(
    fields: &FieldsNamed,
) -> impl Iterator<Item = proc_macro2::TokenStream> {
    let clone_fields = fields.clone();
    let data_expanded_members = clone_fields.named.into_iter().map(|field| {
        let field_name = field.ident.unwrap();
        let span = field_name.span();
        let field_name_stringified = LitStr::new(&field_name.to_string(), span);
        quote_spanned! { span=> #field_name_stringified }
    });
    return data_expanded_members;
}
pub fn extract_fields_name(fields: &FieldsNamed) -> impl Iterator<Item = proc_macro2::TokenStream> {
    let clone_fields = fields.clone();
    let data_expanded_members = clone_fields.named.into_iter().map(|field| {
        let field_name = field.ident.unwrap();
        let span = field_name.span();
        let field_name_stringified = LitStr::new(&field_name.to_string(), span);
        quote_spanned! { span=> #field_name_stringified.to_string()}
    });
    return data_expanded_members;
}

pub fn extract_val_from_attr(attr: &Attribute, name: &str) -> Option<String> {
    let path: &Path = &attr.path;
    let path_ident = path.get_ident().unwrap();
    let attr_path_name = path_ident.to_string();
    if attr_path_name != name {
        return None;
    }

    let meta_info_result = attr.parse_meta();
    if meta_info_result.is_err() {
        return None;
    }

    let meta_info = meta_info_result.unwrap();
    let value = match meta_info {
        syn::Meta::NameValue(syn::MetaNameValue {
            lit: syn::Lit::Str(s),
            ..
        }) => s.value(),
        _ => panic!("malformed attribute syntax"),
    };
    return Some(value);
}

pub fn check_is_attr(attr: &Attribute, name: &str) -> bool {
    let path: &Path = &attr.path;
    let path_ident = path.get_ident().unwrap();
    let attr_path_name = path_ident.to_string();
    return attr_path_name == name;
}

pub fn extract_val_from_attrs(attrs: &Vec<Attribute>, name: &str) -> Option<String> {
    for attr in attrs {
        let val_opt = extract_val_from_attr(attr, name);
        if val_opt.is_some() {
            return val_opt;
        }
    }
    return None;
}

pub fn check_has_attr(attrs: &Vec<Attribute>, name: &str) -> bool {
    for attr in attrs {
        let is_attr = check_is_attr(attr, name);
        if is_attr {
            return true;
        }
    }
    return false;
}

pub fn extract_table_name(_ident: &Ident, attrs: &Vec<Attribute>) -> String {
    let mut name = stringify!(#ident).to_string();
    name = extract_val_from_attrs(attrs, "TableName").unwrap_or(name);
    return name;
}

pub fn extract_annotated_fields(fields: &FieldsNamed, name: &str) -> Vec<Field> {
    let mut result: Vec<Field> = Vec::new();
    let cloned_named: FieldsNamed = fields.clone();
    for field in cloned_named.named.into_iter() {
        let has_attr = check_has_attr(&field.attrs, name);
        if has_attr {
            result.push(field.clone());
        }
    }
    return result;
}

pub fn extract_not_annotated_fields(fields: &FieldsNamed, name: &str) -> Vec<Field> {
    let mut result: Vec<Field> = Vec::new();
    let cloned_named: FieldsNamed = fields.clone();
    for field in cloned_named.named.into_iter() {
        let has_attr = check_has_attr(&field.attrs, name);
        if !has_attr {
            result.push(field.clone());
        }
    }
    return result;
}
