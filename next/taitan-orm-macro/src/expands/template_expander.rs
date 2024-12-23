use crate::attrs::{AttrParser, DefaultAttrParser};
use crate::util::{
    build_impl_trait_token,  copy_to_template_struct,

};
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};
use syn::{Attribute, Data, FieldsNamed, Generics, };
use taitan_orm_trait::ParsedTemplateSql;

pub fn generate_template_struct_and_impl(
    ident: &Ident,
    attrs: &Vec<Attribute>,
    data: &Data,
    fields: &FieldsNamed,
    generics: &Generics,
) -> TokenStream {
    // panic!("{:?}", attrs);
    let template_sql = DefaultAttrParser::extract_template_sql(&attrs);
    if template_sql.is_none() {
        panic!("TemplateRecord must have TemplateSql attribute.");
    }

    let template_sql_str = template_sql.unwrap();
    // panic!("template sql: {}", &template_sql_str);
    // assert_eq!(template_sql_str, "select * from #{name}");
    let template_sql_result = ParsedTemplateSql::build(template_sql_str.as_str());
    if template_sql_result.is_err() {
        panic!(
            "ParsedTemplateSql parse failed: {} with error {}",
            template_sql_str,
            template_sql_result.err().unwrap()
        );
    }
    let template_sql = template_sql_result.unwrap();
    let (get_sql_stream, template_struct_opt) = gen_fn_get_sql(ident, &template_sql);
    let marked_sql = template_sql.sql;
    let variables = template_sql.variables;

    let count_sql_token;
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
            count_sql_token =
                quote!(taitan_orm::traits::CountSql::VariabledSql(String::from(#sql)));
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

    let template_struct  = if template_sql.dollar_signs.is_empty() {
        quote! {}
    } else {
        copy_to_template_struct(ident, data, generics, &marked_sql)
    };
    // panic!("{}", template_struct);

    let impl_ident = build_impl_trait_token(ident, generics, "taitan_orm::traits::TemplateRecord");



    let output = quote! {

        #template_struct

        #impl_ident {

           #get_sql_stream

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

fn gen_fn_get_sql(ident: &Ident, parsed_template_sql: &ParsedTemplateSql) -> (TokenStream, Option<TokenStream>) {
    let marked_sql = &parsed_template_sql.sql;
    let template_struct_name = format_ident!("{}Template", ident);
    if parsed_template_sql.dollar_signs.is_empty() {
        let fn_stream = quote! {
            fn get_sql(&self, page: Option<&taitan_orm::traits::Pagination>) -> String {
                    if let Some(page) = page {
                        let offset = page.page_size * page.page_num;
                        let count = page.page_size;
                        format!("{} LIMIT {}, {}", #marked_sql, offset, count)
                    } else {
                        String::from(#marked_sql)
                    }
                }
        };
        (fn_stream, None)
    } else {
        let fn_stream = quote! {
            fn get_sql(&self, page: Option<&taitan_orm::traits::Pagination>) -> String {
                    let template = #template_struct_name::from(self);
                    let marked_sql = rinja::Template::render(&template).unwrap();
                    if let Some(page) = page {
                        let offset = page.page_size * page.page_num;
                        let count = page.page_size;
                        format!("{} LIMIT {}, {}", marked_sql, offset, count)
                    } else {
                        marked_sql
                    }
                }
        };
        (fn_stream, None)
    }
}
