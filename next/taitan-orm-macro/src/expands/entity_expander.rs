use crate::attrs::{AttrParser, DefaultAttrParser};
use crate::fields::{DefaultFieldMapper, FieldMapper};
use crate::fields::{FieldMapType, FieldsParser};
use crate::types::{DefaultTypeChecker, TypeChecker};
use crate::types::{DefaultTypeExtractor, TypeExtractor};
use crate::fields::EntityParser;
use proc_macro2::Ident;
use quote::quote;
use syn::{Attribute, FieldsNamed};

fn validate_fields(fields: &FieldsNamed) {
    let primary_fields = FieldsParser::from_named(fields).filter_annotated_fields("PrimaryKey");
    if primary_fields.is_empty() {
        panic!("Entity must has at least one PrimaryKey!")
    }

    for field in primary_fields {
        let is_generated =
            <DefaultAttrParser as AttrParser>::check_has_attr(&field.attrs, "Generated");
        let is_auto =
            <DefaultAttrParser as AttrParser>::check_has_attr(&field.attrs, "AutoIncrement");
        if <DefaultTypeChecker as TypeChecker>::type_is_option(&field.ty) {
            if (!is_generated) && (!is_auto) {
                panic!(
                    "Primary Key with Option type must annotated with Generated or AutoIncrement"
                )
            }
        } else if (is_generated) || (is_auto) {
            panic!("Primary Key annotated with Generated or AutoIncrement must be Option")
        }
    }

    let auto_fields = FieldsParser::from_named(fields).filter_annotated_fields("AutoIncrement");
    if auto_fields.len() > 1 {
        panic!("There is more than one AutoIncrement field");
    }
    if auto_fields.len() == 1 {
        let auto_field = auto_fields.first().unwrap();
        if !<DefaultTypeChecker as TypeChecker>::type_is_option(&auto_field.ty) {
            panic!("AutoIncrement Field should be Option<i64>");
        }
        let auto_field_inner_type =
            <DefaultTypeExtractor as TypeExtractor>::get_option_inner_type(&auto_field.ty);
        let auto_field_inner_type = auto_field_inner_type.unwrap();
        if !<DefaultTypeChecker as TypeChecker>::type_has_one_of_names(
            auto_field_inner_type,
            &["i64"],
        ) {
            panic!("AutoIncrement Field should be Option<i64>");
        }
    }
}

pub fn generate_entity_impl(
    ident: &Ident,
    attrs: &Vec<Attribute>,
    fields: &FieldsNamed,
) -> proc_macro2::TokenStream {
    validate_fields(fields);


    let table_name = FieldsParser::get_table_name(ident, attrs);
    let insert_fields_name = FieldsParser::from_named(fields).get_insert_fields();
    let upsert_set_fields_name = FieldsParser::from_named(fields).get_upsert_set_fields();


    // let auto_field_token = if auto_field_opt.is_none() {
    //     quote! { None }
    // } else {
    //     let auto_field = auto_field_opt.clone().unwrap();
    //     let auto_field_name =
    //         <DefaultFieldMapper as FieldMapper>::map_field(auto_field, FieldMapType::Str);
    //     quote! {
    //         Some(#auto_field_name)
    //     }
    // };
    let auto_field_token = FieldsParser::from_named(fields).get_auto_increment_field();

    // let auto_field_opt = FieldsParser::from_named(fields).get_auto_increment_field();
    // let set_auto_field_token = if auto_field_opt.is_none() {
    //     quote! { false }
    // } else {
    //     let auto_field = auto_field_opt.unwrap();
    //     let auto_field_name = auto_field.ident.unwrap();
    //     quote! {
    //         self.#auto_field_name = value;
    //         true
    //     }
    // };
    let set_auto_field_token = FieldsParser::from_named(fields).set_auto_increment_field();


    let insert_args_sqlite = FieldsParser::from_named(fields).gen_insert_arguments_sqlite();
    let insert_args_mysql = FieldsParser::from_named(fields).gen_insert_arguments_mysql();
    let insert_args_postgres = FieldsParser::from_named(fields).gen_insert_arguments_postgres();

    let upsert_args_sqlite = FieldsParser::from_named(fields).gen_upsert_arguments_sqlite();
    let upsert_args_mysql = FieldsParser::from_named(fields).gen_upsert_arguments_mysql();
    let upsert_args_postgres = FieldsParser::from_named(fields).gen_upsert_arguments_postgres();

    let output = quote! {
        impl Entity for #ident {

            fn get_table_name(&self) -> &'static str {
                #table_name
            }

            fn get_insert_fields(&self) -> Vec<String> {
                #insert_fields_name
            }

            fn get_upsert_set_fields(&self) -> Vec<String> {
                #upsert_set_fields_name
            }

            fn get_auto_increment_field(&self) -> Option<&'static str> {
                #auto_field_token
            }

            fn set_auto_increment_field(&mut self, value: Option<i64>) -> bool {
                #set_auto_field_token
            }

            fn gen_insert_arguments_sqlite(&self) -> Result<SqliteArguments<'_>, BoxDynError> {
                #insert_args_sqlite
            }

            fn gen_upsert_arguments_sqlite(&self) -> Result<SqliteArguments<'_>, BoxDynError> {
                #upsert_args_sqlite
            }

            fn gen_insert_arguments_mysql(&self) -> Result<MySqlArguments, BoxDynError> {
                #insert_args_mysql
            }

            fn gen_upsert_arguments_mysql(&self) -> Result<MySqlArguments, BoxDynError> {
                #upsert_args_mysql
            }

            fn gen_insert_arguments_postgres(&self) -> Result<PgArguments, BoxDynError> {
                #insert_args_postgres
            }

            fn gen_upsert_arguments_postgres(&self) -> Result<PgArguments, BoxDynError> {
                #upsert_args_postgres
            }
        }
    };

    output
}

// pub fn impl_entity_macro(input: TokenStream) -> TokenStream {
//     let DeriveInput {
//         attrs, ident, data, ..
//     } = parse_macro_input!(input);
//
//     let fields = extract_fields(&data).unwrap();
//     let output = generate_entity_impl(&ident, &attrs, &fields);
//     //panic!("{}", output);
//     output.into()
// }
