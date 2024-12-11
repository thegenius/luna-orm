use std::fmt::Debug;
use sqlx::any::AnyArguments;
use sqlx::error::BoxDynError;
use sqlx::mysql::MySqlArguments;
use sqlx::postgres::PgArguments;
use sqlx::sqlite::SqliteArguments;
use crate::NotImplementError;
use crate::page::count_sql::CountSql;
use crate::pagination::Pagination;

pub trait TemplateRecord: Sync + Debug {
    fn get_sql(&self, page: Option<&Pagination>) -> String;

    fn get_count_sql(&self) -> CountSql;

    fn get_variables(&self) -> Vec<String>;

    fn gen_template_arguments_sqlite(&self) -> Result<SqliteArguments<'_>, BoxDynError> {
        Err(NotImplementError("gen_primary_arguments_sqlite".to_string()).into())
    }
    fn gen_template_arguments_mysql(&self) -> Result<MySqlArguments, BoxDynError> {
        Err(NotImplementError("gen_primary_arguments_mysql".to_string()).into())
    }
    fn gen_template_arguments_postgres(&self) -> Result<PgArguments, BoxDynError> {
        Err(NotImplementError("gen_primary_arguments_postgres".to_string()).into())
    }
}