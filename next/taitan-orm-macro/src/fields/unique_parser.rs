use crate::fields::mappers::{ArgsConstructorMySql, ArgsConstructorPostgres, ArgsConstructorSqlite, NamesAddConstructor, NamesConstructor};
use crate::fields::table_name_parser::TableNameParser;
use crate::fields::{FieldsContainer};
use proc_macro2::TokenStream;
use std::fmt::Debug;

// 1. Unique中不包含option的field
// 2. 因为涉及为多个Unique生成类，所以parser里面应该传入正确的fields
pub trait UniqueParser:
    FieldsContainer
    + NamesConstructor
    + NamesAddConstructor
    + TableNameParser
    + ArgsConstructorPostgres
    + ArgsConstructorSqlite
    + ArgsConstructorMySql
{
    fn get_unique_field_names(&self) -> TokenStream {
        self.of_names_array()
    }

    fn gen_unique_arguments_sqlite(&self) -> TokenStream {
        self.of_not_option_args_sqlite()
    }

    fn gen_unique_arguments_mysql(&self) -> TokenStream {
        self.of_not_option_args_mysql()
    }

    fn gen_unique_arguments_postgres(&self) -> TokenStream {
        self.of_not_option_args_postgres()
    }
}
