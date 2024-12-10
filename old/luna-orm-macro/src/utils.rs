use crate::field_utils::*;
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

pub fn build_fields_name(fields: &Vec<Field>) -> Vec<proc_macro2::TokenStream> {
    map_field_vec(fields, &|field: Field| {
        map_field(field, FieldMapType::String)
    })
}

pub fn build_fields_name_with_option(fields: &Vec<Field>) -> Vec<proc_macro2::TokenStream> {
    let mut result: Vec<proc_macro2::TokenStream> = Vec::new();
    result.push(quote!(
        let mut fields: Vec<String> = Vec::new();
    ));
    let push_stmt = map_field_vec(fields, &|field: Field| {
        if field_is_option(&field) {
            map_field(field, FieldMapType::OptionNamePush)
        } else {
            map_field(field, FieldMapType::NamePush)
        }
    });
    result.extend(push_stmt);
    result.push(quote!(fields));
    return result;
}

pub fn build_args_add_clause(fields: &Vec<Field>, cloned: bool) -> Vec<proc_macro2::TokenStream> {
    if cloned {
        map_field_vec(fields, &|f: Field| map_field(f, FieldMapType::ArgsAddClone))
    } else {
        map_field_vec(fields, &|f: Field| map_field(f, FieldMapType::ArgsAdd))
    }
}
pub fn build_args_add_ref_clause_by_vec(fields: &Vec<Field>) -> Vec<proc_macro2::TokenStream> {
    map_field_vec(fields, &|f: Field| map_field(f, FieldMapType::ArgsAddClone))
}

pub fn build_args_push_clause(fields: &FieldsNamed) -> Vec<proc_macro2::TokenStream> {
    map_fields(fields, &|f: Field| map_field(f, FieldMapType::ArgsAdd))
}

pub fn build_args_add_ref_clause(fields: &FieldsNamed) -> Vec<proc_macro2::TokenStream> {
    map_fields(fields, &|f: Field| map_field(f, FieldMapType::ArgsAddRef))
}

pub fn build_args_add_option_ref_clause(fields: &FieldsNamed) -> Vec<proc_macro2::TokenStream> {
    map_fields(fields, &|f: Field| {
        map_field(f, FieldMapType::ArgsAddOptionRef)
    })
}

pub fn build_args_add_option_location_ref_clause(
    fields: &FieldsNamed,
) -> Vec<proc_macro2::TokenStream> {
    map_fields(fields, &|f: Field| {
        map_field(f, FieldMapType::ArgsAddOptionLocationRef)
    })
}

pub fn gen_args_add_maybe_option(fields: &Vec<Field>) -> Vec<proc_macro2::TokenStream> {
    let mut result: Vec<proc_macro2::TokenStream> = Vec::new();
    let add_clause = map_field_vec(fields, &|field: Field| {
        if field_is_option(&field) {
            map_field(field, FieldMapType::ArgsAddOptionRef)
        } else {
            map_field(field, FieldMapType::ArgsAddRef)
        }
    });
    result.extend(add_clause);
    result
}

pub fn extract_enum(data: &Data) -> Result<&DataEnum> {
    match data {
        Data::Enum(data_enum) => Ok(data_enum),

        _ => Err(Error::new(
            Span::call_site(),
            "Expected a `struct` with named fields",
        )),
    }
}

pub fn extract_enum_variants(data: &Data) -> Result<Vec<Ident>> {
    let data_enum = extract_enum(data)?;
    let mut variant_names = Vec::new();
    for variant in &data_enum.variants {
        let variant_name: Ident = variant.ident.clone();
        variant_names.push(variant_name);
    }
    Ok(variant_names)
}

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

pub fn extract_selected_fields_name(fields: &FieldsNamed) -> Vec<proc_macro2::TokenStream> {
    map_fields(fields, &|field: Field| {
        map_field(field, FieldMapType::BoolPush)
    })
}
pub fn extract_order_by_fields_name(fields: &FieldsNamed) -> Vec<proc_macro2::TokenStream> {
    map_fields(fields, &|field: Field| {
        map_field(field, FieldMapType::BoolPush)
    })
}

pub fn extract_fields_name_str(fields: &FieldsNamed) -> Vec<proc_macro2::TokenStream> {
    map_fields(fields, &|field: Field| map_field(field, FieldMapType::Str))
}

pub fn extract_fields_name(fields: &FieldsNamed) -> Vec<proc_macro2::TokenStream> {
    map_fields(fields, &|field: Field| {
        map_field(field, FieldMapType::String)
    })
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

pub fn extract_val_vev_from_attrs(attrs: &Vec<Attribute>, name: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for attr in attrs {
        let val_opt = extract_val_from_attr(attr, name);
        if val_opt.is_some() {
            result.push(val_opt.unwrap());
        }
    }
    return result;
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

pub fn extract_table_name(ident: &Ident, attrs: &Vec<Attribute>) -> String {
    let mut name = ident.to_string();
    name = extract_val_from_attrs(attrs, "TableName").unwrap_or(name);
    return name;
}

pub fn extract_template_sql(attrs: &Vec<Attribute>) -> Option<String> {
    extract_val_from_attrs(attrs, "TemplateSql")
}
pub fn extract_template_count_sql(attrs: &Vec<Attribute>) -> Option<String> {
    extract_val_from_attrs(attrs, "TemplateCountSql")
}

pub fn extract_unique_index(attrs: &Vec<Attribute>) -> Vec<Vec<String>> {
    let indexes = extract_val_vev_from_attrs(attrs, "UniqueIndex");
    let result: Vec<Vec<String>> = indexes
        .iter()
        .map(|s| s.split(',').map(|e| e.trim().to_string()).collect())
        .collect();
    return result;
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
