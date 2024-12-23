use std::process::id;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{Attribute, FieldsNamed, Lifetime};
use taitan_orm_trait::ParsedTemplateSql;
use crate::attrs::{AttrParser, DefaultAttrParser};
use crate::util::{extract_generic_lifetimes, check_type_lifetime, build_struct_ident};

pub fn generate_template_struct_and_impl(
    ident: &Ident,
    attrs: &Vec<Attribute>,
    fields: &FieldsNamed,
    lifetimes: &Vec<Lifetime>,
) -> TokenStream {

    // panic!("{:?}", attrs);
    let template_sql = DefaultAttrParser::extract_template_sql(&attrs);
    if template_sql.is_none() {
        panic!("TemplateRecord must have TemplateSql attribute.");
    }

    let template_sql_str = template_sql.unwrap();
    // panic!("template sql: {}", &template_sql_str);
    // assert_eq!(template_sql_str, "select * from #{name}");
    let template_sql_result  = ParsedTemplateSql::build(template_sql_str.as_str());
    if template_sql_result.is_err() {
        panic!("ParsedTemplateSql parse failed: {} with error {}", template_sql_str, template_sql_result.err().unwrap());
    }
    let template_sql = template_sql_result.unwrap();
    let marked_sql = template_sql.sql;
    let variables = template_sql.variables;

    let count_sql_token ;
    let count_sql = DefaultAttrParser::extract_template_count_sql(&attrs).unwrap_or_default();
    if count_sql.is_empty() {
        count_sql_token = quote!(taitan_orm::traits::CountSql::Empty);
    } else {
        let parsed_count_sql = ParsedTemplateSql::build(&count_sql).unwrap();
        if parsed_count_sql.variables.len() != variables.len() {
            panic!("template sql variables not equal to count sql");
        }
        let sql = parsed_count_sql.sql;
        if parsed_count_sql.variables.is_empty() {
            count_sql_token = quote!(taitan_orm::traits::CountSql::PlainSql(String::from(#sql)));
        } else {
            count_sql_token = quote!(taitan_orm::traits::CountSql::VariabledSql(String::from(#sql)));
        }
    }

    let args_add = variables
        .iter()
        .map(|variable| {
            let ident = Ident::new(variable, Span::call_site());
            quote! {
                sqlx::Arguments::add(&mut args, &self.#ident)?;
            }
        })
        .collect::<Vec<TokenStream>>();

    let struct_ident = build_struct_ident(ident, lifetimes);

    let impl_ident = if !lifetimes.is_empty() {
        quote! {
            impl <#(#lifetimes),*> taitan_orm::traits::TemplateRecord for #struct_ident
        }
    } else {
        quote! {
            impl  taitan_orm::traits::TemplateRecord for #struct_ident
        }
    };

    let output = quote! {

        #impl_ident {

            fn get_sql(&self, page: Option<&taitan_orm::traits::Pagination>) -> String {
                if let Some(page) = page {
                    let offset = page.page_size * page.page_num;
                    let count = page.page_size;
                    format!("{} LIMIT {}, {}", #marked_sql, offset, count)
                } else {
                    String::from(#marked_sql)
                }
            }

            fn get_count_sql(&self) -> taitan_orm::traits::CountSql {
                #count_sql_token
            }

            fn get_variables(&self) -> Vec<String> {
                vec![
                    #(#variables.to_string(), )*
                ]
            }

            fn gen_template_arguments_sqlite(&self) -> Result<sqlx::sqlite::SqliteArguments<'_>, sqlx::error::BoxDynError> {
                let mut args = sqlx::sqlite::SqliteArguments::default();
                #(#args_add)*
                Ok(args)
            }

            fn gen_template_arguments_mysql(&self) -> Result<sqlx::mysql::MySqlArguments, sqlx::error::BoxDynError> {
                let mut args = sqlx::mysql::MySqlArguments::default();
                #(#args_add)*
                Ok(args)
            }

            fn gen_template_arguments_postgres(&self) -> Result<sqlx::postgres::PgArguments, sqlx::error::BoxDynError> {
                let mut args = sqlx::postgres::PgArguments::default();
                #(#args_add)*
                Ok(args)
            }

        }
    };
    output
}