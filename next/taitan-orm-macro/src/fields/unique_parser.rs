use crate::fields::table_name_parser::TableNameParser;
use crate::fields::{DefaultFieldMapper, FieldMapType, FieldMapper, FieldsContainer, FieldsFilter, FieldsParser};
use proc_macro2::{Ident, TokenStream};
use std::fmt::Debug;
use quote::quote;
use syn::{Attribute, Field};
use taitan_orm_trait::NotImplementError;
use crate::fields::mappers::{ArgsConstructorMySql, ArgsConstructorPostgres, ArgsConstructorSqlite};
use crate::types::DefaultTypeChecker;

pub trait Unique: Sync + Debug {}

pub trait UniqueParser: FieldsContainer + TableNameParser {
    fn get_unique_field_names(&self) -> TokenStream;
    fn gen_unique_arguments_sqlite(&self) -> TokenStream;
    fn gen_unique_arguments_mysql(&self) -> TokenStream;
    fn gen_unique_arguments_postgres(&self) -> TokenStream;
}

// 1. Unique中不包含option的field
// 2. 因为涉及为多个Unique生成类，所以parser里面应该传入正确的fields
impl UniqueParser for FieldsParser {
    fn get_unique_field_names(&self) -> TokenStream {
        let tokens =
            DefaultFieldMapper::map_field_vec(self.get_fields(), &|field: Field| {
                DefaultFieldMapper::map_field(field, FieldMapType::Str)
            });
        quote!(
            &[ #(#tokens,)* ]
        )
    }

    fn gen_unique_arguments_sqlite(&self) -> TokenStream {
        FieldsParser::from_vec(self.get_fields()).of_not_option_args_sqlite()
    }

    fn gen_unique_arguments_mysql(&self) -> TokenStream {
        FieldsParser::from_vec(self.get_fields()).of_not_option_args_mysql()
    }

    fn gen_unique_arguments_postgres(&self) -> TokenStream {
        FieldsParser::from_vec(self.get_fields()).of_not_option_args_postgres()
    }
}
