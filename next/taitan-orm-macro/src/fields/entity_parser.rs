use crate::attrs::{AttrParser, DefaultAttrParser};
use crate::fields::{DefaultFieldMapper, FieldMapType, FieldMapper, FieldsContainer};
use crate::fields::FieldsParser;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Attribute, Fields};

/**
和trait Entity 一一对应
*/
pub trait EntityParser: FieldsContainer {
    fn get_table_name(ident: &Ident, attrs: &Vec<Attribute>) -> TokenStream;
    fn get_insert_fields(&self) -> TokenStream;
    fn get_upsert_set_fields(&self) -> TokenStream;
    fn get_auto_increment_field(&self) -> TokenStream;
    fn set_auto_increment_field(&mut self) -> TokenStream;
    fn gen_insert_arguments_sqlite(&self) -> TokenStream;
    fn gen_upsert_arguments_sqlite(&self) -> TokenStream;
    fn gen_insert_arguments_mysql(&self) -> TokenStream;
    fn gen_upsert_arguments_mysql(&self) -> TokenStream;
    fn gen_insert_arguments_postgres(&self) -> TokenStream;
    fn gen_upsert_arguments_postgres(&self) -> TokenStream;
}

impl EntityParser for FieldsParser {
    fn get_table_name(ident: &Ident, attrs: &Vec<Attribute>) -> TokenStream {
        let table_name = <DefaultAttrParser as AttrParser>::extract_table_name(ident, attrs);
        quote! {
            #table_name
        }
    }

    fn get_insert_fields(&self) -> TokenStream {
        let insert_fields = FieldsParser::from_vec(self.get_fields()).get_insert_fields_vec();
        let insert_fields_name = FieldsParser::from_vec(&insert_fields).get_maybe_option_name_vec();
        quote! {
            #insert_fields_name
        }
    }

    fn get_upsert_set_fields(&self) -> TokenStream {
        let upsert_set_fields = FieldsParser::from_vec(self.get_fields()).get_upsert_set_fields_vec();
        let upsert_set_fields_name =
            FieldsParser::from_vec(&upsert_set_fields).get_maybe_option_name_vec();
        quote! {
            #upsert_set_fields_name
        }
    }

    fn get_auto_increment_field(&self) -> TokenStream {
        let auto_field_opt = FieldsParser::from_vec(self.get_fields()).get_auto_increment_field();
        if auto_field_opt.is_none() {
            quote! { None }
        } else {
            // let auto_field = auto_field_opt.clone().unwrap();
            // let auto_field_name =
            //     <DefaultFieldMapper as FieldMapper>::map_field(auto_field, FieldMapType::Str);
            // quote! {
            //     Some(#auto_field_name)
            // }
            quote! { None }
        }
    }

    fn set_auto_increment_field(&mut self) -> TokenStream {
        let auto_field_opt = FieldsParser::from_vec(self.get_fields()).get_auto_increment_field();
        if auto_field_opt.is_none() {
            quote! { false }
        } else {
            let auto_field = auto_field_opt.unwrap();
            let auto_field_name = auto_field.ident.unwrap();
            quote! {
                self.#auto_field_name = value;
                true
            }
        }
    }

    fn gen_insert_arguments_sqlite(&self) -> TokenStream {
        let all_fields = FieldsParser::from_vec(self.get_fields()).get_insert_fields_vec();
        FieldsParser::from_vec(&all_fields).get_maybe_option_args_sqlite()
    }

    fn gen_upsert_arguments_sqlite(&self) -> TokenStream {
        let all_fields = FieldsParser::from_vec(self.get_fields()).get_upsert_fields_vec();
        FieldsParser::from_vec(&all_fields).get_maybe_option_args_sqlite()
    }

    fn gen_insert_arguments_mysql(&self) -> TokenStream {
        let all_fields = FieldsParser::from_vec(self.get_fields()).get_insert_fields_vec();
        FieldsParser::from_vec(&all_fields).get_maybe_option_args_mysql()
    }

    fn gen_upsert_arguments_mysql(&self) -> TokenStream {
        let all_fields = FieldsParser::from_vec(self.get_fields()).get_upsert_fields_vec();
        FieldsParser::from_vec(&all_fields).get_maybe_option_args_mysql()
    }

    fn gen_insert_arguments_postgres(&self) -> TokenStream {
        let all_fields = FieldsParser::from_vec(self.get_fields()).get_insert_fields_vec();
        FieldsParser::from_vec(&all_fields).get_maybe_option_args_postgres()
    }

    fn gen_upsert_arguments_postgres(&self) -> TokenStream {
        let all_fields = FieldsParser::from_vec(self.get_fields()).get_upsert_fields_vec();
        FieldsParser::from_vec(&all_fields).get_maybe_option_args_postgres()
    }
}
