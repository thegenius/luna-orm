use proc_macro2::TokenStream;
use quote::quote;
use syn::Field;
use crate::fields::{DefaultFieldMapper, FieldMapType, FieldMapper, FieldsContainer, FieldsParser, TableNameParser};

pub trait LocationParser: FieldsContainer + TableNameParser {
    fn get_fields_name(&self) -> TokenStream;
    fn get_where_clause(&self) -> TokenStream;
    fn gen_location_arguments_sqlite(&self) -> TokenStream;
    fn gen_location_arguments_mysql(&self) -> TokenStream;
    fn gen_location_arguments_postgres(&self) -> TokenStream;
}

impl LocationParser for FieldsParser {
    fn get_fields_name(&self) -> TokenStream {
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
        let args_add_clause = DefaultFieldMapper::map_field_vec(
            self.get_fields(),
            &DefaultFieldMapper::map_to_option_args_add_val,
        );
        quote! {
            let mut args = SqliteArguments::default();
            #(#args_add_clause)*
            Ok(args)
        }
    }

    fn gen_location_arguments_mysql(&self) -> TokenStream {
        let args_add_clause = DefaultFieldMapper::map_field_vec(
            self.get_fields(),
            &DefaultFieldMapper::map_to_option_args_add_val,
        );
        quote! {
            let mut args = MySqlArguments::default();
            #(#args_add_clause)*
            Ok(args)
        }
    }

    fn gen_location_arguments_postgres(&self) -> TokenStream {
        let args_add_clause = DefaultFieldMapper::map_field_vec(
            self.get_fields(),
            &DefaultFieldMapper::map_to_option_args_add_val,
        );
        quote! {
            let mut args = PgArguments::default();
            #(#args_add_clause)*
            Ok(args)
        }
    }
}