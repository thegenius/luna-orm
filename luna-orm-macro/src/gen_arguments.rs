use crate::fields_parser::FieldsParser;
use crate::utils::extract_fields;
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{parse_macro_input, Attribute, DeriveInput, FieldsNamed};

pub fn generate_entity_impl(
    ident: &Ident,
    attrs: &Vec<Attribute>,
    fields: &FieldsNamed,
) -> proc_macro2::TokenStream {
    let insert_args = FieldsParser::from_named(fields).get_insert_args();
    let upsert_args = FieldsParser::from_named(fields).get_upsert_args();

    let output = quote! {
        impl Entity for #ident {

            fn gen_mysql_arguments(&self) -> MySqlArguments {
                MySqlArguments::default()
            }

            fn gen_sqlite_arguments(&self) -> SqliteArguments<'_> {
                SqliteArguments::default()
            }

            fn any_arguments_of_insert(&self) -> AnyArguments<'_> {
                #insert_args
            }

            fn any_arguments_of_upsert(&self) -> AnyArguments<'_> {
                #upsert_args
            }
        }
    };

    output
}

pub fn impl_gen_arguments_macro(input: TokenStream) -> TokenStream {
    let DeriveInput {
        attrs, ident, data, ..
    } = parse_macro_input!(input);

    let fields = extract_fields(&data).unwrap();
    let output = generate_entity_impl(&ident, &attrs, &fields);
    //panic!("{}", output);
    output.into()
}
