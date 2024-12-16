use crate::attrs::{AttrParser, DefaultAttrParser};
use crate::fields::FieldsContainer;
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
    fn set_auto_increment_field(&mut self, value: Option<i64>) -> TokenStream;
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
        todo!()
    }

    fn get_upsert_set_fields(&self) -> TokenStream {
        todo!()
    }

    fn get_auto_increment_field(&self) -> TokenStream {
        todo!()
    }

    fn set_auto_increment_field(&mut self, value: Option<i64>) -> TokenStream {
        todo!()
    }

    fn gen_insert_arguments_sqlite(&self) -> TokenStream {
        let all_fields = FieldsParser::from_vec(self.get_fields()).get_insert_fields();
        FieldsParser::from_vec(&all_fields).get_maybe_option_args_sqlite()
    }

    fn gen_upsert_arguments_sqlite(&self) -> TokenStream {
        let all_fields = FieldsParser::from_vec(self.get_fields()).get_upsert_fields();
        FieldsParser::from_vec(&all_fields).get_maybe_option_args_sqlite()
    }

    fn gen_insert_arguments_mysql(&self) -> TokenStream {
        let all_fields = FieldsParser::from_vec(self.get_fields()).get_insert_fields();
        FieldsParser::from_vec(&all_fields).get_maybe_option_args_mysql()
    }

    fn gen_upsert_arguments_mysql(&self) -> TokenStream {
        let all_fields = FieldsParser::from_vec(self.get_fields()).get_upsert_fields();
        FieldsParser::from_vec(&all_fields).get_maybe_option_args_mysql()
    }

    fn gen_insert_arguments_postgres(&self) -> TokenStream {
        let all_fields = FieldsParser::from_vec(self.get_fields()).get_insert_fields();
        FieldsParser::from_vec(&all_fields).get_maybe_option_args_postgres()
    }

    fn gen_upsert_arguments_postgres(&self) -> TokenStream {
        let all_fields = FieldsParser::from_vec(self.get_fields()).get_upsert_fields();
        FieldsParser::from_vec(&all_fields).get_maybe_option_args_postgres()
    }
}
