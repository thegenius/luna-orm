
use crate::database::lib2::{DB, Database, DatabaseType};
use crate::{error::LunaOrmError, LunaOrmResult};

use luna_orm_trait::Selection;
use sqlx::any::AnyConnectOptions;

use sqlx::sqlite::{
    SqliteArguments, SqliteConnectOptions, SqliteJournalMode, SqlitePool, SqliteSynchronous,
};
use sqlx::{AnyPool, Pool, Sqlite};

use std::fs;
use std::path::Path;
use std::str::FromStr;

use crate::sql_generator2::{DefaultSqlGenerator, SqlGenerator};

use crate::command_executor2::CommandExecutorNew;
use crate::sql_executor2::{GetAffectedRows, SqlExecutorNew};
use luna_orm_trait::schema_trait::{
    EntityNew, LocationNew, PrimaryNew, SelectedEntityNew, UpdateCommand,
};
use path_absolutize::*;
use sqlx::error::BoxDynError;
use tracing::debug;

pub struct SqliteLocalConfig {
    pub work_dir: String,
    pub db_file: String,
    pub use_specified: bool,
}

impl SqliteLocalConfig {
    pub fn new<S>(work_dir: S, db_file: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            work_dir: work_dir.into(),
            db_file: db_file.into(),
            use_specified: false,
        }
    }

    pub fn new_specified<S>(work_dir: S, db_file: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            work_dir: work_dir.into(),
            db_file: db_file.into(),
            use_specified: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SqliteDatabaseNew {
    database_type: DatabaseType,
    sql_generator: DefaultSqlGenerator,
    sqlite_pool: Option<SqlitePool>,
}

impl SqlExecutorNew for SqliteDatabaseNew {
    type DB = Sqlite;
    fn new_get_pool(&self) -> LunaOrmResult<&Pool<Self::DB>> {
        if let Some(sqlite_pool) = &self.sqlite_pool {
            Ok(sqlite_pool)
        } else {
            Err(LunaOrmError::DatabaseInitFail(
                "sqlite connection not found when get pool".to_string(),
            ))
        }
    }

    fn get_affected_rows(
        &self,
        query_result: &<Self::DB as sqlx::Database>::QueryResult,
    ) -> LunaOrmResult<u64> {
        Ok(query_result.get_affected_rows())
    }
}

impl CommandExecutorNew for SqliteDatabaseNew {
    type G = DefaultSqlGenerator;

    fn get_generator(&self) -> &Self::G {
        &self.sql_generator
    }

    fn gen_insert_arguments<'a>(
        &'a self,
        entity: &'a dyn EntityNew,
    ) -> Result<<Self::DB as sqlx::Database>::Arguments<'a>, BoxDynError> {
        entity.gen_insert_arguments_sqlite()
    }

    fn gen_upsert_arguments<'a>(
        &'a self,
        entity: &'a dyn EntityNew,
    ) -> Result<<Self::DB as sqlx::Database>::Arguments<'a>, BoxDynError> {
        entity.gen_upsert_arguments_sqlite()
    }

    fn gen_update_arguments<'a>(
        &'a self,
        update_command: &'a dyn UpdateCommand,
    ) -> Result<<Self::DB as sqlx::Database>::Arguments<'a>, BoxDynError> {
        update_command.gen_update_arguments_sqlite()
    }

    fn gen_primary_arguments<'a>(
        &'a self,
        primary: &'a dyn PrimaryNew,
    ) -> Result<<Self::DB as sqlx::Database>::Arguments<'a>, BoxDynError> {
        primary.gen_primary_arguments_sqlite()
    }

    fn gen_location_arguments<'a>(
        &'a self,
        location: &'a dyn LocationNew,
    ) -> Result<<Self::DB as sqlx::Database>::Arguments<'a>, BoxDynError> {
        location.gen_location_arguments_sqlite()
    }

    async fn select<SE>(
        &self,
        primary: &dyn PrimaryNew,
        selection: &SE::Selection,
    ) -> LunaOrmResult<Option<SE>>
    where
        SE: SelectedEntityNew<Self::DB> + Send + Unpin,
    {
        let pool = self.new_get_pool()?;
        let mut conn = pool.acquire().await?;
        debug!(target: "luna_orm", command = "select", primary = ?primary, selection = ?selection);
        let sql = self.get_generator().get_select_sql(selection, primary);
        debug!(target: "luna_orm", command = "select", sql = sql);
        let args: SqliteArguments<'_> = self.gen_primary_arguments(primary)?;
        let result: Option<SE> = self
            .new_fetch_optional(&mut *conn, &sql, selection, args)
            .await?;
        debug!(target: "luna_orm", command = "select", result = ?result);
        Ok(result)
    }

    // sqlx sqlite driver has bug #2099, it returns result before the actual commit on insert returning clause
    // the work around is create a transaction
    // async fn create<'a>(&mut self, entity: &'a mut dyn Entity) -> LunaOrmResult<bool> {
    //     debug!(target: "luna_orm2", command = "create",  entity = ?entity);
    //     let sql = self.get_generator().get_create_sql(entity);
    //     debug!(target: "luna_orm", command = "create", sql = sql);
    //     let args = entity.any_arguments_of_insert();
    //     if entity.get_auto_increment_field().is_some() {
    //         let last_row_id: LastRowId = self.fetch_one(&sql, args).await?;
    //         entity.set_auto_increment_field(Some(last_row_id.id));
    //     } else {
    //         self.execute(&sql, args).await?;
    //     }
    //     debug!(target: "luna_orm", command = "create", result = ?entity);
    //
    //     // the work around
    //     let trx = self.pool.begin().await?;
    //     // query the record
    //     trx.commit().await?;
    //     return Ok(true);
    // }
}

impl Database for SqliteDatabaseNew {
    fn get_type(&self) -> &DatabaseType {
        &self.database_type
    }
}

impl From<SqliteDatabaseNew> for DB<SqliteDatabaseNew> {
    fn from(value: SqliteDatabaseNew) -> Self {
        Self(value)
    }
}

impl SqliteDatabaseNew {
    pub async fn init_local_sqlite(
        workspace_dir: &str,
        db_file: &str,
    ) -> LunaOrmResult<(AnyPool, SqlitePool)> {
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

        sqlx::any::install_default_drivers();
        let url = format!("sqlite:{}", db_file_path.to_str().unwrap());
        let any_options = AnyConnectOptions::from_str(&url).unwrap();
        let any_pool = AnyPool::connect_with(any_options)
            .await
            .map_err(|_e| LunaOrmError::DatabaseInitFail("init pool fail".to_string()))?;
        Ok((any_pool, sqlite_pool))
    }

    pub async fn build(config: SqliteLocalConfig) -> LunaOrmResult<Self> {
        let pool = SqliteDatabaseNew::init_local_sqlite(&config.work_dir, &config.db_file).await?;
        let generator = DefaultSqlGenerator::new();
        if config.use_specified {
            let database = SqliteDatabaseNew {
                database_type: DatabaseType::SqliteLocal,
                sql_generator: generator,
                sqlite_pool: Some(pool.1),
            };
            Ok(database)
        } else {
            let database = SqliteDatabaseNew {
                database_type: DatabaseType::SqliteLocal,
                sql_generator: generator,
                sqlite_pool: None,
            };
            Ok(database)
        }
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
