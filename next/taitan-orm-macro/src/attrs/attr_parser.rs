use proc_macro2::Ident;
use syn::{Attribute, Path};

pub trait AttrParser {
    fn extract_val_from_attr(attr: &Attribute, name: &str) -> Option<String>;

    fn check_is_attr(attr: &Attribute, name: &str) -> bool;

    fn extract_val_from_attrs(attrs: &Vec<Attribute>, name: &str) -> Option<String>;

    fn extract_val_vev_from_attrs(attrs: &Vec<Attribute>, name: &str) -> Vec<String>;

    fn check_has_attr(attrs: &Vec<Attribute>, name: &str) -> bool;

    fn extract_table_name(ident: &Ident, attrs: &Vec<Attribute>) -> String;

    fn extract_template_sql(attrs: &Vec<Attribute>) -> Option<String>;
    fn extract_template_count_sql(attrs: &Vec<Attribute>) -> Option<String>;

    fn extract_unique_index(attrs: &Vec<Attribute>) -> Vec<Vec<String>>;
}

pub struct DefaultAttrParser {}

impl AttrParser for DefaultAttrParser {
    fn extract_val_from_attr(attr: &Attribute, name: &str) -> Option<String> {
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

    fn check_is_attr(attr: &Attribute, name: &str) -> bool {
        let path: &Path = &attr.path;
        let path_ident = path.get_ident().unwrap();
        let attr_path_name = path_ident.to_string();
        return attr_path_name == name;
    }

    fn extract_val_from_attrs(attrs: &Vec<Attribute>, name: &str) -> Option<String> {
        for attr in attrs {
            let val_opt = <DefaultAttrParser as AttrParser>::extract_val_from_attr(attr, name);
            if val_opt.is_some() {
                return val_opt;
            }
        }
        return None;
    }

    fn extract_val_vev_from_attrs(attrs: &Vec<Attribute>, name: &str) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        for attr in attrs {
            let val_opt = <DefaultAttrParser as AttrParser>::extract_val_from_attr(attr, name);
            if val_opt.is_some() {
                result.push(val_opt.unwrap());
            }
        }
        return result;
    }

    fn check_has_attr(attrs: &Vec<Attribute>, name: &str) -> bool {
        for attr in attrs {
            let is_attr = <DefaultAttrParser as AttrParser>::check_is_attr(attr, name);
            if is_attr {
                return true;
            }
        }
        return false;
    }

    fn extract_table_name(ident: &Ident, attrs: &Vec<Attribute>) -> String {
        let mut name = ident.to_string();
        name = <DefaultAttrParser as AttrParser>::extract_val_from_attrs(attrs, "TableName")
            .unwrap_or(name);
        return name;
    }

    fn extract_template_sql(attrs: &Vec<Attribute>) -> Option<String> {
        <DefaultAttrParser as AttrParser>::extract_val_from_attrs(attrs, "TemplateSql")
    }
    fn extract_template_count_sql(attrs: &Vec<Attribute>) -> Option<String> {
        <DefaultAttrParser as AttrParser>::extract_val_from_attrs(attrs, "TemplateCountSql")
    }

    fn extract_unique_index(attrs: &Vec<Attribute>) -> Vec<Vec<String>> {
        let indexes =
            <DefaultAttrParser as AttrParser>::extract_val_vev_from_attrs(attrs, "UniqueIndex");
        let result: Vec<Vec<String>> = indexes
            .iter()
            .map(|s| s.split(',').map(|e| e.trim().to_string()).collect())
            .collect();
        return result;
    }
}