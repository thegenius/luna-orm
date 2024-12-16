use crate::schema::impl_schema_macro;
use proc_macro::TokenStream;

mod attrs;
mod expands;
mod fields;
mod schema;
mod types;
mod util;

#[proc_macro_derive(
    Schema,
    attributes(TableName, PrimaryKey, UniqueIndex, AutoIncrement, Generated)
)]
pub fn expand_schema_macro(input: TokenStream) -> TokenStream {
    impl_schema_macro(input)
}
