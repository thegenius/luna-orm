use proc_macro2::TokenStream;

/**
和trait Entity 一一对应
*/
pub trait EntityParser {
    fn get_table_name(&self) -> TokenStream;
    fn get_insert_fields(&self) -> TokenStream;
    fn get_upsert_set_fields(&self) -> TokenStream;
    fn get_auto_increment_field(&self) -> TokenStream;
    fn set_auto_increment_field(&mut self, value: Option<i64>) -> TokenStream;
    fn gen_insert_arguments_sqlite(&self) -> TokenStream;
    fn gen_upsert_arguments_sqlite(&self) -> TokenStream;
    fn gen_insert_arguments_mysql(&self) -> TokenStream;
    fn gen_upsert_arguments_mysql(&self) -> TokenStream;
    fn gen_insert_arguments_postgres(&self) -> TokenStream;
    fn gen_upsert_arguments_postgres(&self) -> TokenStream;
}

