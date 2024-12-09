use std::fmt::Debug;
use sqlx::error::BoxDynError;
use sqlx::mysql::MySqlArguments;
use sqlx::postgres::PgArguments;
use sqlx::sqlite::SqliteArguments;
use crate::NotImplementError;

pub trait Primary: Sync + Debug {

    fn get_table_name(&self) -> &'static str;

    fn get_primary_field_names(&self) -> &'static [&'static str];

    fn gen_primary_arguments_sqlite(&self) -> Result<SqliteArguments<'_>, BoxDynError> {
        Err(NotImplementError("gen_primary_arguments_sqlite".to_string()).into())
    }
    fn gen_primary_arguments_mysql(&self) -> Result<MySqlArguments, BoxDynError> {
        Err(NotImplementError("gen_primary_arguments_mysql".to_string()).into())
    }
    fn gen_primary_arguments_postgres(&self) -> Result<PgArguments, BoxDynError> {
        Err(NotImplementError("gen_primary_arguments_postgres".to_string()).into())
    }
}