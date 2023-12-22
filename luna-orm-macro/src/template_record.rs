use proc_macro::{self, TokenStream};
use quote::quote;
use quote::quote_spanned;

use crate::utils::*;

use luna_orm_trait::ParsedTemplateSql;
use proc_macro2::{Ident, Span};
use syn::Attribute;
use syn::Field;
use syn::{
    parse_macro_input, token, Data, DataEnum, DataStruct, DataUnion, DeriveInput, Error, Fields,
    FieldsNamed, LitStr, Path, Result,
};

pub fn impl_template_record_by_macro(input: TokenStream) -> TokenStream {
    let DeriveInput {
        attrs, ident, data, ..
    } = parse_macro_input!(input);
    let fields = extract_fields(&data).unwrap();
    let args_push_clause = build_args_push_clause(&fields);
    let args_add_clause = build_args_add_ref_clause(&fields);
    let fields_name = extract_fields_name(&fields);
    let fields_name_str = extract_fields_name_str(&fields);
    let table_name = extract_table_name(&ident, &attrs);

    let template_sql = extract_template_sql(&attrs);
    if template_sql.is_none() {
        panic!("TemplateRecord must have TemplateSql attribute.");
    }
    let template_sql = template_sql.unwrap();
    let template_sql = ParsedTemplateSql::build(&template_sql).unwrap();
    let marked_sql = template_sql.sql;
    let variables = template_sql.variables;

    let count_sql_token;
    let count_sql = extract_template_count_sql(&attrs).unwrap_or_default();
    if count_sql.is_empty() {
        count_sql_token = quote!(CountSql::Empty);
    } else {
        let parsed_count_sql = ParsedTemplateSql::build(&count_sql).unwrap();
        if parsed_count_sql.variables.len() != variables.len() {
            panic!("template sql variables not equal to count sql");
        }
        let sql = parsed_count_sql.sql;
        if parsed_count_sql.variables.is_empty() {
            count_sql_token = quote!(CountSql::PlainSql(String::from(#sql)));
        } else {
            count_sql_token = quote!(CountSql::VariabledSql(String::from(#sql)));
        }
    }

    let arguments_add = variables
        .iter()
        .map(|variable| {
            let span = Span::call_site();
            let ident = Ident::new(variable, span);
            quote!(
                luna_orm_trait::add_arg(&mut arguments, &self.#ident);
            )
        })
        .collect::<Vec<proc_macro2::TokenStream>>();

    let output = quote! {

        impl TemplateRecord for #ident {

            fn get_sql(&self, page: Option<&Pagination>) -> String {
                if let Some(page) = page {
                    let offset = page.page_size * page.page_num;
                    let count = page.page_size;
                    format!("{} LIMIT {}, {}", #marked_sql, offset, count)
                } else {
                    String::from(#marked_sql)
                }
            }

            fn get_count_sql(&self) -> CountSql {
                #count_sql_token
            }

            fn get_variables(&self) -> Vec<String> {
                vec![
                    #(#variables.to_string(), )*
                ]
            }


            fn any_arguments(&self) -> sqlx::any::AnyArguments<'_> {
                let mut arguments = AnyArguments::default();
                #(#arguments_add)*
                return arguments;
            }
        }
    };

    //panic!("{}", output);
    output.into()
}
