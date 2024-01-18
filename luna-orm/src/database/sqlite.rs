use crate::database::lib::Database;
use crate::database::lib::DatabaseType;
use crate::database::DB;
use crate::{error::LunaOrmError, LunaOrmResult};

use luna_orm_trait::LastRowId;
use sqlx::any::AnyConnectOptions;
use sqlx::any::AnyPoolOptions;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePool, SqliteSynchronous};
use sqlx::AnyPool;

use std::fs;
use std::path::Path;
use std::str::FromStr;

use crate::command_executor::CommandExecutor;
use crate::sql_executor::SqlExecutor;
use crate::sql_generator::DefaultSqlGenerator;
use crate::sql_generator::SqlGenerator;
use luna_orm_trait::Entity;
use luna_orm_trait::SelectedEntity;
use sqlx::any::AnyArguments;
use sqlx::any::AnyRow;
use tracing::debug;

use path_absolutize::*;

pub struct SqliteLocalConfig {
    pub work_dir: String,
    pub db_file: String,
}

impl SqliteLocalConfig {
    pub fn new<S>(work_dir: S, db_file: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            work_dir: work_dir.into(),
            db_file: db_file.into(),
        }
    }
}

#[derive(Debug, Clone)]
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

    // sqlx sqlite driver has bug #2099, it returns result before the actual commit on insert returning clause
    // the work around is create a transaction
    async fn create<'a>(&mut self, entity: &'a mut dyn Entity) -> LunaOrmResult<bool> {
        debug!(target: "luna_orm2", command = "create",  entity = ?entity);
        let sql = self.get_generator().get_create_sql(entity);
        debug!(target: "luna_orm", command = "create", sql = sql);
        let args = entity.any_arguments_of_insert();
        if entity.get_auto_increment_field().is_some() {
            let last_row_id: LastRowId = self.fetch_one(&sql, args).await?;
            entity.set_auto_increment_field(Some(last_row_id.id));
        } else {
            self.execute(&sql, args).await?;
        }
        debug!(target: "luna_orm", command = "create", result = ?entity);

        // the work around
        let trx = self.pool.begin().await?;
        // query the record
        trx.commit().await?;
        return Ok(true);
    }
}

impl Database for SqliteDatabase {
    fn get_type(&self) -> &DatabaseType {
        &self.database_type
    }
}

impl From<SqliteDatabase> for DB<SqliteDatabase> {
    fn from(value: SqliteDatabase) -> Self {
        Self(value)
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

    /*
    pub async fn from_sqlite_pool(pool: SqlitePool) -> Self {
        let generator = DefaultSqlGenerator::new();

        Self {
            database_type: DatabaseType::SqliteLocal,
            pool:
            sql_generator: generator,
        }
    }
    */
}
