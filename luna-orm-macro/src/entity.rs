use crate::field_utils::{map_field, FieldMapType};
use crate::fields_parser::FieldsParser;
use crate::type_check::type_has_one_of_names;
use crate::type_check::type_is_option;
use crate::type_extract::get_option_inner_type;
use crate::utils::check_has_attr;
use crate::utils::*;
use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, token::Else, Attribute, DeriveInput, Field, FieldsNamed, Ident};

fn validate_fields(fields: &FieldsNamed) {
    let primary_fields = FieldsParser::from_named(fields).filter_annotated_fields("PrimaryKey");
    if primary_fields.is_empty() {
        panic!("Entity must has at least one PrimaryKey!")
    }

    for field in primary_fields {
        let is_generated = check_has_attr(&field.attrs, "Generated");
        let is_auto = check_has_attr(&field.attrs, "AutoIncrement");
        if type_is_option(&field.ty) {
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
        if !type_is_option(&auto_field.ty) {
            panic!("AutoIncrement Field should be Option<i64>");
        }
        let auto_field_inner_type = get_option_inner_type(&auto_field.ty);
        let auto_field_inner_type = auto_field_inner_type.unwrap();
        if !type_has_one_of_names(auto_field_inner_type, &["i64"]) {
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

    let table_name = extract_table_name(ident, attrs);

    let insert_fields = FieldsParser::from_named(fields).get_insert_fields();
    let insert_fields_name = FieldsParser::from_vec(&insert_fields).get_maybe_option_name_vec();

    let upsert_set_fields = FieldsParser::from_named(fields).get_upsert_set_fields();
    let upsert_set_fields_name =
        FieldsParser::from_vec(&upsert_set_fields).get_maybe_option_name_vec();

    let auto_field_opt = FieldsParser::from_named(fields).get_auto_increment_field();
    let auto_field_token = if auto_field_opt.is_none() {
        quote! { None }
    } else {
        let auto_field = auto_field_opt.clone().unwrap();
        let auto_field_name = map_field(auto_field, FieldMapType::Str);
        quote! {
            Some(#auto_field_name)
        }
    };

    let set_auto_field_token = if auto_field_opt.is_none() {
        quote! { false }
    } else {
        let auto_field = auto_field_opt.unwrap();
        let auto_field_name = auto_field.ident.unwrap();
        quote! {
            self.#auto_field_name = value;
            true
        }
    };

    let insert_args = FieldsParser::from_named(fields).get_insert_args();
    let upsert_args = FieldsParser::from_named(fields).get_upsert_args();

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

pub fn impl_entity_macro(input: TokenStream) -> TokenStream {
    let DeriveInput {
        attrs, ident, data, ..
    } = parse_macro_input!(input);

    let fields = extract_fields(&data).unwrap();
    let output = generate_entity_impl(&ident, &attrs, &fields);
    //panic!("{}", output);
    output.into()
}
