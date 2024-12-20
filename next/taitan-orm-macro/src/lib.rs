use crate::schema::impl_schema_macro;
use proc_macro::TokenStream;
use crate::template::impl_record_macro;

mod attrs;
mod expands;
mod fields;
mod schema;
mod types;
mod util;
mod template;

#[proc_macro_derive(
    Schema,
    attributes(TableName, PrimaryKey, UniqueKey, AutoIncrement, Generated)
)]
pub fn expand_schema_macro(input: TokenStream) -> TokenStream {
    impl_schema_macro(input)
}

#[proc_macro_derive(TemplateRecord, attributes(TemplateSql, TemplateCountSql))]
pub fn expand_template_record(input: TokenStream) -> TokenStream {
    impl_record_macro(input)
}
