use crate::database::lib::DatabaseType;

use crate::database::lib::Database;
use crate::{error::LunaOrmError, LunaOrmResult};


use sqlx::any::AnyConnectOptions;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePool, SqliteSynchronous};
use sqlx::AnyPool;


use std::fs;
use std::path::Path;
use std::str::FromStr;

use crate::command_executor::CommandExecutor;
use crate::sql_executor::SqlExecutor;
use crate::sql_generator::DefaultSqlGenerator;



use path_absolutize::*;



pub struct SqliteLocalConfig {
    pub work_dir: String,
    pub db_file: String,
}

pub struct SqliteDatabase {
    database_type: DatabaseType,
    pool: AnyPool,
    sql_generator: DefaultSqlGenerator,
}

impl SqlExecutor for SqliteDatabase {
    fn get_pool(&self) -> LunaOrmResult<&AnyPool> {
        Ok(&self.pool)
    }
}

impl CommandExecutor for SqliteDatabase {
    type G = DefaultSqlGenerator;

    fn get_generator(&self) -> &Self::G {
        &self.sql_generator
    }
}

impl Database for SqliteDatabase {
    fn get_type(&self) -> &DatabaseType {
        &self.database_type
    }
}

impl SqliteDatabase {
    pub async fn init_local_sqlite(workspace_dir: &str, db_file: &str) -> LunaOrmResult<AnyPool> {
        let workspace = Path::new(workspace_dir);
        let workspace_absolute = workspace
            .absolutize()
            .map_err(|_e| LunaOrmError::DatabaseInitFail("workdir absolute fail".to_string()))?;

        fs::create_dir_all(&workspace_absolute)
            .map_err(|_e| LunaOrmError::DatabaseInitFail("create dir fail".to_string()))?;
        let db_file_path = workspace_absolute.join(db_file);
        {
            let options = SqliteConnectOptions::new()
                .filename(db_file_path.clone())
                .synchronous(SqliteSynchronous::Full)
                .journal_mode(SqliteJournalMode::Wal)
                .create_if_missing(true);
            let _ = SqlitePool::connect_with(options).await.map_err(|_e| {
                LunaOrmError::DatabaseInitFail("create is missing fail".to_string())
            })?;
        }

        sqlx::any::install_default_drivers();
        let url = format!("sqlite:{}", db_file_path.to_str().unwrap());
        let any_options = AnyConnectOptions::from_str(&url).unwrap();
        let any_pool = AnyPool::connect_with(any_options)
            .await
            .map_err(|_e| LunaOrmError::DatabaseInitFail("init pool fail".to_string()))?;
        return Ok(any_pool);
    }

    pub async fn build(config: SqliteLocalConfig) -> LunaOrmResult<Self> {
        let pool = SqliteDatabase::init_local_sqlite(&config.work_dir, &config.db_file).await?;
        let generator = DefaultSqlGenerator::new();
        let database = SqliteDatabase {
            database_type: DatabaseType::SqliteLocal,
            pool,
            sql_generator: generator,
        };
        return Ok(database);
    }
}
