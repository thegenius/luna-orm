use sqlx::{MySql, MySqlConnection, MySqlPool, PgConnection, PgPool, Postgres, Sqlite};
use crate::database::sqlite::{SqliteDatabase, SqliteReadCommander, SqliteWriteCommander};
use crate::sql_generator::{MySqlGenerator, PostgresGenerator};
use crate::{executor_impl, CountResult, DefaultSqlGenerator, SqlExecutor, SqlGeneratorContainer, SqlGenericExecutor};

#[derive(Debug, Clone)]
pub struct PostgresDatabase {
    generator: PostgresGenerator,
    pool: PgPool,
}
impl PostgresDatabase {
    pub fn get_pool(&mut self) -> crate::Result<&PgPool> {
        Ok(&self.pool)
    }
}
impl SqlGenericExecutor for PostgresDatabase {
    type DB = Postgres;
    type CountType = CountResult;

    fn get_affected_rows(query_result: &<Self::DB as sqlx::Database>::QueryResult) -> u64 {
        query_result.rows_affected()
    }

}
// SqlExecutor + SqlGeneratorContainer + Extractor
impl SqlGeneratorContainer for PostgresDatabase {
    type G = PostgresGenerator;

    fn get_generator(&mut self) -> &Self::G {
        &self.generator
    }
}

impl SqlExecutor for PostgresDatabase {
    executor_impl!(PgConnection);
}


