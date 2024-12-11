use std::fs;
use std::path::Path;
use sqlx::{Sqlite, SqlitePool};
use sqlx::sqlite::{SqliteArguments, SqliteConnectOptions, SqliteJournalMode, SqliteSynchronous};
use tracing::debug;
use taitan_orm_trait::{Primary, SelectedEntity};
use crate::sql_generator::DefaultSqlGenerator;
use crate::{LunaOrmError, SqlApi, SqlExecutor, SqlGenerator};
use crate::database::sqlite::SqliteLocalConfig;
use crate::Result;
use crate::sql_executor::GetAffectedRows;
use path_absolutize::Absolutize;
#[derive(Debug, Clone)]
pub struct SqliteCommander {
    sql_generator: DefaultSqlGenerator,
    sqlite_pool: SqlitePool,
}





impl SqlExecutor for SqliteCommander {
    type DB = Sqlite;
    fn get_pool(&self) -> Result<&SqlitePool> {
        Ok(&self.sqlite_pool)
    }
    fn get_affected_rows(
        &self,
        query_result: &<Self::DB as sqlx::Database>::QueryResult,
    ) -> Result<u64> {
        Ok(query_result.get_affected_rows())
    }
}

impl SqlApi for SqliteCommander {
    type G = DefaultSqlGenerator;

    fn get_generator(&self) -> &Self::G {
        &self.sql_generator
    }

    async fn select<SE>(
        &self,
        primary: &dyn Primary,
        selection: &SE::Selection,
    ) -> Result<Option<SE>>
    where
        SE: SelectedEntity<Self::DB> + Send + Unpin,
    {
        let pool = self.get_pool()?;
        let mut conn = pool.acquire().await?;
        debug!(target: "luna_orm", command = "select", primary = ?primary, selection = ?selection);
        let sql = self.get_generator().get_select_sql(selection, primary);
        debug!(target: "luna_orm", command = "select", sql = sql);
        let args: SqliteArguments<'_> = primary.gen_primary_arguments_sqlite()?;
        let result: Option<SE> = self
            .fetch_optional(&mut *conn, &sql, selection, args)
            .await?;
        debug!(target: "luna_orm", command = "select", result = ?result);
        Ok(result)
    }

}


impl SqliteCommander {
    async fn init_local(
        workspace_dir: &str,
        db_file: &str,
    ) -> Result<SqlitePool> {
        let workspace = Path::new(workspace_dir);
        let workspace_absolute = workspace
            .absolutize()
            .map_err(|_e| LunaOrmError::DatabaseInitFail("workdir absolute fail".to_string()))?;

        fs::create_dir_all(&workspace_absolute)
            .map_err(|_e| LunaOrmError::DatabaseInitFail("create dir fail".to_string()))?;
        let db_file_path = workspace_absolute.join(db_file);

        let options = SqliteConnectOptions::new()
            .filename(db_file_path.clone())
            .synchronous(SqliteSynchronous::Full)
            .journal_mode(SqliteJournalMode::Wal)
            .create_if_missing(true);
        let sqlite_pool = SqlitePool::connect_with(options)
            .await
            .map_err(|_e| LunaOrmError::DatabaseInitFail("create is missing fail".to_string()))?;
        Ok(sqlite_pool)
    }

    pub async fn build(config: SqliteLocalConfig) -> Result<Self> {
        let pool = SqliteCommander::init_local(&config.work_dir, &config.db_file).await?;
        let generator = DefaultSqlGenerator::new();
        let database = SqliteCommander {
            sql_generator: generator,
            sqlite_pool: pool,
        };
        Ok(database)
    }
}
