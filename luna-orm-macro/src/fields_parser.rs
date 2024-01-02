use proc_macro2::TokenStream;
use syn::Field;
use syn::FieldsNamed;

use crate::field_utils::map_field;
use crate::field_utils::map_field_vec;
use crate::field_utils::FieldMapType;

pub struct FieldsParser {
    fields: Vec<Field>,
}

impl FieldsParser {
    pub fn from_vec(fields: &Vec<Field>) -> FieldsParser {
        Self {
            fields: fields.clone(),
        }
    }
    pub fn from_named(fields: &FieldsNamed) -> FieldsParser {
        let fields: Vec<Field> = fields.clone().named.into_iter().collect();
        Self { fields: fields }
    }
}

impl FieldsParser {
    pub fn map_with<F>(self, map_fn: &F) -> Vec<TokenStream>
    where
        F: Fn(Field) -> TokenStream,
    {
        self.fields
            .into_iter()
            .map(map_fn)
            .collect::<Vec<TokenStream>>()
    }

    pub fn get_names(&self) -> Vec<TokenStream> {
        map_field_vec(&self.fields, &|field: Field| {
            map_field(field, FieldMapType::Str)
        })
    }

    pub fn get_name_vec(&self) -> Vec<TokenStream> {
        map_field_vec(&self.fields, &|field: Field| {
            map_field(field, FieldMapType::String)
        })
    }
}
