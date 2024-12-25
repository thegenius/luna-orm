use crate::attrs::{AttrParser, DefaultAttrParser};
use crate::fields::{FieldsFilter, FieldsParser};
use crate::util::{build_impl_trait_token, copy_to_template_struct};
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};
use syn::{Attribute, Data, FieldsNamed, Generics};
use taitan_orm_trait::ParsedTemplateSql;

pub fn generate_template_struct_and_impl(
    ident: &Ident,
    attrs: &Vec<Attribute>,
    data: &Data,
    fields: &FieldsNamed,
    generics: &Generics,
) -> TokenStream {
    let template_sql = DefaultAttrParser::extract_template_sql(&attrs)
        .expect("TemplateRecord must have sql attribute, for example: #[sql = \" select name from user where id = #{id}\"]");

    let template_sql = ParsedTemplateSql::build(template_sql.as_str())
        .expect(format!("Failed to parse template sql: {}", template_sql).as_str());

    let limit_fields = FieldsParser::from_named(fields).filter_annotated_fields("limit_field");
    let limit_fields_names = limit_fields
        .into_iter()
        .map(|field| field.ident.unwrap().to_string())
        .collect::<Vec<String>>();
    // panic!("{:?}", limit_fields_names);

    let get_sql_render_fn_stream = gen_fn_get_sql(ident, data, generics, &template_sql);

    let count_sql = DefaultAttrParser::extract_template_count_sql(&attrs).unwrap_or_default();
    let get_count_sql_render_fn_stream = if count_sql.is_empty() {
        gen_fn_get_count_sql(ident, data, generics, None)
    } else {
        let parsed_count_sql = ParsedTemplateSql::build(count_sql.as_str())
            .expect(format!("Failed to parse template sql: {}", count_sql).as_str());
        gen_fn_get_count_sql(ident, data, generics, Some(&parsed_count_sql))
    };

    let variables = template_sql.variables;
    let args_add = gen_args_add_clause(&variables);


    let count_variables = variables
        .iter()
        .filter(|variable| !limit_fields_names.contains(variable))
        .collect::<Vec<&String>>();
    // panic!("{:?}", count_variables);
    let count_args_add = gen_args_add_clause(&count_variables);

    let impl_ident = build_impl_trait_token(ident, generics, "taitan_orm::traits::TemplateRecord");

    let template_struct_stream = get_sql_render_fn_stream.struct_stream.unwrap_or_default();
    let get_sql_fn_stream = get_sql_render_fn_stream.fn_stream;

    let count_template_struct_stream = get_count_sql_render_fn_stream
        .struct_stream
        .unwrap_or_default();
    let get_count_sql_fn_stream = get_count_sql_render_fn_stream.fn_stream;

    let output = quote! {

        #template_struct_stream

        #count_template_struct_stream

        #impl_ident {

            #get_sql_fn_stream

            #get_count_sql_fn_stream

            fn get_variables(&self) -> Vec<String> {
                vec![
                    #(#variables.to_string(), )*
                ]
            }

            fn gen_template_count_arguments_sqlite(&self) -> Result<sqlx::sqlite::SqliteArguments<'_>, sqlx::error::BoxDynError> {
                let mut args = sqlx::sqlite::SqliteArguments::default();
                #(#count_args_add)*
                Ok(args)
            }

            fn gen_template_count_arguments_mysql(&self) -> Result<sqlx::mysql::MySqlArguments, sqlx::error::BoxDynError> {
                let mut args = sqlx::mysql::MySqlArguments::default();
                #(#count_args_add)*
                Ok(args)
            }

            fn gen_template_count_arguments_postgres(&self) -> Result<sqlx::postgres::PgArguments, sqlx::error::BoxDynError> {
                let mut args = sqlx::postgres::PgArguments::default();
                #(#count_args_add)*
                Ok(args)
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

fn gen_args_add_clause<T: AsRef<str>>(fields: &Vec<T>) -> Vec<TokenStream> {
    fields
        .iter()
        .map(|variable| {
            let ident = Ident::new(variable.as_ref(), Span::call_site());
            quote! {
                sqlx::Arguments::add(&mut args, &self.#ident)?;
            }
        })
        .collect::<Vec<TokenStream>>()
}

struct SqlRenderFnStream {
    fn_stream: TokenStream,
    struct_stream: Option<TokenStream>,
}

fn gen_fn_get_count_sql(
    ident: &Ident,
    data: &Data,
    generics: &Generics,
    parsed_template_sql_opt: Option<&ParsedTemplateSql>,
) -> SqlRenderFnStream {
    if parsed_template_sql_opt.is_none() {
        let fn_stream = quote! {
            fn get_count_sql(&self) -> Option<String> { None }
        };
        return SqlRenderFnStream {
            fn_stream,
            struct_stream: None,
        };
    }

    let parsed_template_sql = parsed_template_sql_opt.unwrap();
    let marked_sql = &parsed_template_sql.sql;

    if parsed_template_sql.need_render() {
        let template_struct_stream =
            copy_to_template_struct(ident, data, generics, &marked_sql, "CountTemplate");
        let template_struct_name = format_ident!("{}CountTemplate", ident);
        let fn_stream = quote! {
            fn get_count_sql(&self) -> Option<String> {
                let template = #template_struct_name::from(self);
                rinja::Template::render(&template).ok()
            }
        };
        SqlRenderFnStream {
            fn_stream,
            struct_stream: Some(template_struct_stream),
        }
    } else {
        let fn_stream = quote! {
            fn get_count_sql(&self) -> Option<String> {
                Some(String::from(#marked_sql))
            }
        };
        SqlRenderFnStream {
            fn_stream,
            struct_stream: None,
        }
    }
}

fn gen_fn_get_sql(
    ident: &Ident,
    data: &Data,
    generics: &Generics,
    parsed_template_sql: &ParsedTemplateSql,
) -> SqlRenderFnStream {
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
        SqlRenderFnStream {
            fn_stream,
            struct_stream: None,
        }
    } else {
        let template_struct_stream =
            copy_to_template_struct(ident, data, generics, &marked_sql, "Template");
        let fn_stream = quote! {
            fn get_sql(&self, page: Option<&taitan_orm::traits::Pagination>) -> String {
                    let template = #template_struct_name::from(self);
                    let marked_sql_result = rinja::Template::render(&template);
                    if marked_sql_result.is_err() {
                        return String::default();
                    }
                    let marked_sql = marked_sql_result.unwrap();
                    if let Some(page) = page {
                        let offset = page.page_size * page.page_num;
                        let count = page.page_size;
                        format!("{} LIMIT {}, {}", marked_sql, offset, count)
                    } else {
                        marked_sql
                    }
                }
        };
        SqlRenderFnStream {
            fn_stream,
            struct_stream: Some(template_struct_stream),
        }
    }
}
