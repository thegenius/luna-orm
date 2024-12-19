use crate::attrs::{AttrParser, DefaultAttrParser};
use crate::fields::{FieldsContainer, FieldsParser};
use syn::Field;

pub trait FieldsFilter: FieldsContainer {
    fn filter_annotated_fields(&self, annotation_str: &str) -> Vec<Field>;
    fn filter_not_annotated_fields(&self, annotation_str: &str) -> Vec<Field>;
    fn filter_not_auto_generated(&self) -> Vec<Field>;
    fn filter_named_fields(&self, annotation_str: &Vec<String>) -> Vec<Field>;
    fn get_sorted_fields_vec(&self) -> Vec<Field>;
    fn get_insert_fields_vec(&self) -> Vec<Field>;
    fn get_upsert_fields_vec(&self) -> Vec<Field>;
    fn get_upsert_set_fields_vec(&self) -> Vec<Field>;
    fn get_auto_increment_field_opt(&self) -> Option<Field>;
}

impl FieldsFilter for FieldsParser {
    fn filter_annotated_fields(&self, annotation_str: &str) -> Vec<Field> {
        let mut result: Vec<Field> = Vec::new();
        for field in self.get_fields().iter() {
            let has_attr =
                <DefaultAttrParser as AttrParser>::check_has_attr(&field.attrs, annotation_str);
            if has_attr {
                result.push(field.clone());
            }
        }
        result
    }

    fn filter_not_annotated_fields(&self, annotation_str: &str) -> Vec<Field> {
        let mut result: Vec<Field> = Vec::new();
        for field in self.get_fields().iter() {
            let has_attr =
                <DefaultAttrParser as AttrParser>::check_has_attr(&field.attrs, annotation_str);
            if !has_attr {
                result.push(field.clone());
            }
        }
        result
    }

    fn filter_not_auto_generated(&self) -> Vec<Field> {
        let mut result: Vec<Field> = Vec::new();
        for field in self.get_fields().iter() {
            let is_generated =
                <DefaultAttrParser as AttrParser>::check_has_attr(&field.attrs, "Generated");
            let is_auto =
                <DefaultAttrParser as AttrParser>::check_has_attr(&field.attrs, "AutoIncrement");
            if (!is_generated) && (!is_auto) {
                result.push(field.clone());
            }
        }
        result
    }


    fn filter_named_fields(&self, names: &Vec<String>) -> Vec<Field> {
        let mut result: Vec<Field> = Vec::new();
        for field in self.get_fields().iter() {
            let has_name = names.contains(&field.ident.as_ref().unwrap().to_string());
            if has_name {
                result.push(field.clone());
            }
        }
        result
    }

    fn get_sorted_fields_vec(&self) -> Vec<Field> {
        let primary_fields =
            FieldsParser::from_vec(self.get_fields()).filter_annotated_fields("PrimaryKey");
        let body_fields =
            FieldsParser::from_vec(self.get_fields()).filter_not_annotated_fields("PrimaryKey");
        let mut all_fields: Vec<Field> = Vec::new();
        all_fields.extend(primary_fields);
        all_fields.extend(body_fields);
        all_fields
    }

    fn get_insert_fields_vec(&self) -> Vec<Field> {
        let mut all_fields: Vec<Field> = self.get_sorted_fields_vec();
        all_fields = FieldsParser::from_vec(&all_fields).filter_not_auto_generated();
        all_fields
    }

    fn get_upsert_fields_vec(&self) -> Vec<Field> {
        let primary_fields =
            FieldsParser::from_vec(self.get_fields()).filter_annotated_fields("PrimaryKey");
        let body_fields =
            FieldsParser::from_vec(self.get_fields()).filter_not_annotated_fields("PrimaryKey");
        let mut all_fields: Vec<Field> = Vec::new();
        all_fields.extend(primary_fields);
        all_fields.extend(body_fields.clone());
        all_fields.extend(body_fields);
        all_fields = FieldsParser::from_vec(&all_fields).filter_not_auto_generated();
        all_fields
    }

    fn get_upsert_set_fields_vec(&self) -> Vec<Field> {
        let mut body_fields =
            FieldsParser::from_vec(self.get_fields()).filter_not_annotated_fields("PrimaryKey");
        body_fields = FieldsParser::from_vec(&body_fields).filter_not_auto_generated();
        body_fields
    }

    fn get_auto_increment_field_opt(&self) -> Option<Field> {
        let auto_increment_fields =
            FieldsParser::from_vec(self.get_fields()).filter_annotated_fields("AutoIncrement");
        let first_one = auto_increment_fields.first();
        if first_one.is_none() {
            return None;
        } else {
            return Some(first_one.unwrap().to_owned());
        }
    }
}
