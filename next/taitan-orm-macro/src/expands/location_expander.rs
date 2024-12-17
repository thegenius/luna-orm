use case::CaseExt;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{Attribute, FieldsNamed};
use crate::attrs::{AttrParser, DefaultAttrParser};
use crate::fields::{FieldsFilter, FieldsParser, UniqueParser};

pub fn generate_location_struct_and_impl(
    ident: &Ident,
    attrs: &Vec<Attribute>,
    fields: &FieldsNamed,
) -> TokenStream {
    let table_name = DefaultAttrParser::extract_table_name(ident, attrs);
    let fields_vec = FieldsParser::from_named(fields).filter_not_annotated_fields("PrimaryKey");
    let struct_name =  format!("{}Location", table_name.to_camel());


    let unique_field_names = FieldsParser::from_vec(&fields_vec).get_unique_field_names();
    let unique_arguments_sqlite = FieldsParser::from_vec(&fields_vec).gen_unique_arguments_sqlite();
    let unique_arguments_mysql = FieldsParser::from_vec(&fields_vec).gen_unique_arguments_mysql();
    let unique_arguments_postgres = FieldsParser::from_vec(&fields_vec).gen_unique_arguments_postgres();

    let struct_ident = Ident::new(&struct_name, Span::call_site());
    let fields_tokens = FieldsParser::from_vec(&fields_vec).get_not_option_fields();

    let output = quote! {
        #[derive(Default, Debug, Clone)]
        pub struct #struct_ident {
            #fields_tokens
        }

        impl Unique for #struct_ident {
            fn get_table_name(&self) -> &'static str {
                #table_name
            }

            fn get_unique_field_names(&self) -> &'static [&'static str] {
                #unique_field_names
            }

            fn gen_unique_arguments_sqlite(&self) -> Result<SqliteArguments<'_>, BoxDynError> {
                #unique_arguments_sqlite
            }

            fn gen_unique_arguments_mysql(&self) -> Result<MySqlArguments, BoxDynError> {
                #unique_arguments_mysql
            }

            fn gen_unique_arguments_postgres(&self) -> Result<PgArguments, BoxDynError> {
                #unique_arguments_postgres
            }
        }
    };

    output
}