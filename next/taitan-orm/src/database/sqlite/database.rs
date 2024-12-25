use crate::database::sqlite::commanders::read::SqliteReadCommander;
use crate::database::sqlite::{SqliteLocalConfig, SqliteTransaction, SqliteWriteCommander};
use crate::sql_generator::DefaultSqlGenerator;
use crate::sql_generator_container::SqlGeneratorContainer;
use crate::{SqlExecutor, TaitanOrmError};
use path_absolutize::Absolutize;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqliteSynchronous};
use sqlx::SqlitePool;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct SqliteDatabase {
    sql_generator: DefaultSqlGenerator,
    pub(crate) sqlite_pool: SqlitePool,
}

impl SqliteDatabase {
    async fn init_local(workspace_dir: &str, db_file: &str) -> crate::Result<SqlitePool> {
        let workspace = Path::new(workspace_dir);
        let workspace_absolute = workspace
            .absolutize()
            .map_err(|_e| TaitanOrmError::DatabaseInitFail("workdir absolute fail".to_string()))?;

        fs::create_dir_all(&workspace_absolute)
            .map_err(|_e| TaitanOrmError::DatabaseInitFail("create dir fail".to_string()))?;
        let db_file_path = workspace_absolute.join(db_file);

        let options = SqliteConnectOptions::new()
            .filename(db_file_path.clone())
            .synchronous(SqliteSynchronous::Full)
            .journal_mode(SqliteJournalMode::Wal)
            .create_if_missing(true);
        let sqlite_pool = SqlitePool::connect_with(options)
            .await
            .map_err(|_e| TaitanOrmError::DatabaseInitFail("create is missing fail".to_string()))?;
        Ok(sqlite_pool)
    }

    pub async fn build(config: SqliteLocalConfig<'_>) -> crate::Result<SqliteDatabase> {
        let pool = SqliteDatabase::init_local(&config.work_dir, &config.db_file).await?;
        let generator = DefaultSqlGenerator::new();
        let database = SqliteDatabase {
            sql_generator: generator,
            sqlite_pool: pool,
        };
        Ok(database)
    }

    pub async fn transaction<'a>(&'a mut self) -> crate::Result<SqliteTransaction<'a>> {
        let trx = self.get_pool()?.begin().await?;
        let generator = self.get_generator();
        let transaction = SqliteTransaction::new(trx, generator);
        Ok(transaction)
    }
}

impl SqlGeneratorContainer for SqliteDatabase {
    type G = DefaultSqlGenerator;

    fn get_generator(&mut self) -> &Self::G {
        &self.sql_generator
    }
}

impl SqliteWriteCommander for SqliteDatabase {}

impl SqliteReadCommander for SqliteDatabase {}
