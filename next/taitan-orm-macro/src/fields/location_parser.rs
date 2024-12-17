use proc_macro2::TokenStream;
use quote::quote;
use syn::Field;
use crate::fields::{DefaultFieldMapper, FieldMapType, FieldMapper, FieldsContainer, FieldsParser, TableNameParser};
use crate::fields::mappers::{ArgsConstructorMySql, ArgsConstructorPostgres, ArgsConstructorSqlite};

pub trait LocationParser: FieldsContainer + TableNameParser {
    fn get_location_fields_name(&self) -> TokenStream;
    fn get_where_clause(&self) -> TokenStream;
    fn gen_location_arguments_sqlite(&self) -> TokenStream;
    fn gen_location_arguments_mysql(&self) -> TokenStream;
    fn gen_location_arguments_postgres(&self) -> TokenStream;
}

impl LocationParser for FieldsParser {
    fn get_location_fields_name(&self) -> TokenStream {
        let tokens =
            DefaultFieldMapper::map_field_vec(self.get_fields(), &|field: Field| {
                DefaultFieldMapper::map_field(field, FieldMapType::OptionNamePush)
            });
        quote!(
            let mut fields: Vec<String> = Vec::new();
            #(#tokens)*
            fields
        )
    }

    fn get_where_clause(&self) -> TokenStream {
        let tokens =
            DefaultFieldMapper::map_field_vec(self.get_fields(), &DefaultFieldMapper::map_to_where_field);
        quote! {
            let mut sql = String::default();
            #(#tokens)*
            sql
        }
    }

    fn gen_location_arguments_sqlite(&self) -> TokenStream {
        FieldsParser::from_vec(self.get_fields()).of_location_args_sqlite()
    }

    fn gen_location_arguments_mysql(&self) -> TokenStream {
        FieldsParser::from_vec(self.get_fields()).of_location_args_mysql()
    }

    fn gen_location_arguments_postgres(&self) -> TokenStream {
        FieldsParser::from_vec(self.get_fields()).of_location_args_postgres()
    }
}