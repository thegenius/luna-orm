use crate::error::LunaOrmError;
use crate::mapper::{GenericDaoMapper, GenericDaoMapperImpl};
use crate::transaction::Transaction;
use crate::LunaOrmResult;

use luna_orm_trait::*;
use path_absolutize::*;
use sqlx::any::AnyConnectOptions;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePool, SqliteSynchronous};

use sqlx::AnyPool;
use std::fs;
use std::path::Path;
use std::str::FromStr;

pub enum DatabaseType {
    SqliteLocal,
    MySql,
    PostgreSql,
}

pub struct SqliteLocalConfig {
    pub work_dir: String,
    pub db_file: String,
}

pub enum DatabaseConfig {
    SqliteLocal(SqliteLocalConfig),
}

pub struct Database {
    database_type: DatabaseType,
    pool: AnyPool,
}

impl Database {
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

    pub async fn build(config: DatabaseConfig) -> LunaOrmResult<Self> {
        match config {
            DatabaseConfig::SqliteLocal(sqlite_local) => {
                let pool =
                    Database::init_local_sqlite(&sqlite_local.work_dir, &sqlite_local.db_file)
                        .await?;
                let database = Database {
                    database_type: DatabaseType::SqliteLocal,
                    pool,
                };
                return Ok(database);
            }
        }
    }

    pub async fn transaction<'a>(&self) -> LunaOrmResult<Transaction<'a>> {
        let trx = self.pool.begin().await?;
        let transaction = Transaction::new(trx);
        return Ok(transaction);
    }
}

impl Database {
    pub async fn query(&self, sql: &str) -> Result<usize, LunaOrmError> {
        let result = sqlx::query(sql).execute(&self.pool).await?;
        return Ok(result.rows_affected() as usize);
    }

    #[inline]
    pub async fn select<'e, P, S, SE>(
        &self,
        primary: P,
        selection: S,
    ) -> Result<Option<SE>, LunaOrmError>
    where
        P: Primary + Send,
        S: Selection + Send,
        SE: SelectedEntity + Send + Unpin,
    {
        let result: Option<SE> =
            <GenericDaoMapperImpl as GenericDaoMapper>::select(&self.pool, primary, selection)
                .await?;
        return Ok(result);
    }

    #[inline]
    pub async fn create<'e, E>(&self, entity: E) -> Result<E, LunaOrmError>
    where
        E: Entity + Send + Clone,
    {
        let result: E =
            <GenericDaoMapperImpl as GenericDaoMapper>::create(&self.pool, entity).await?;
        return Ok(result);
    }

    #[inline]
    pub async fn insert<'e, E>(&self, entity: E) -> Result<bool, LunaOrmError>
    where
        E: Entity + Send + Clone,
    {
        let result: bool =
            <GenericDaoMapperImpl as GenericDaoMapper>::insert(&self.pool, entity).await?;
        return Ok(result);
    }

    #[inline]
    pub async fn upsert<'e, E>(&self, entity: E) -> Result<bool, LunaOrmError>
    where
        E: Entity + Send + Clone,
    {
        let result: bool =
            <GenericDaoMapperImpl as GenericDaoMapper>::upsert(&self.pool, entity).await?;
        return Ok(result);
    }

    #[inline]
    pub async fn update<'e, E>(&self, entity: E) -> Result<bool, LunaOrmError>
    where
        E: Entity + Send + Clone,
    {
        let result: bool =
            <GenericDaoMapperImpl as GenericDaoMapper>::update(&self.pool, entity).await?;
        return Ok(result);
    }

    #[inline]
    pub async fn remove<'e, P, E>(&self, primary: P) -> Result<E, LunaOrmError>
    where
        P: Primary + Send,
        E: Entity + Send + Clone,
    {
        let result: E =
            <GenericDaoMapperImpl as GenericDaoMapper>::remove(&self.pool, primary).await?;
        return Ok(result);
    }

    #[inline]
    pub async fn delete<'e, P>(&self, primary: P) -> Result<bool, LunaOrmError>
    where
        P: Primary + Send,
    {
        let result: bool =
            <GenericDaoMapperImpl as GenericDaoMapper>::delete(&self.pool, primary).await?;
        return Ok(result);
    }

    #[inline]
    pub async fn search<'e, EX, L, S, SE>(
        &self,
        location: L,
        selection: S,
    ) -> Result<Vec<SE>, LunaOrmError>
    where
        L: Location + Send,
        S: Selection + Send,
        SE: SelectedEntity + Send + Unpin,
    {
        let result: Vec<SE> =
            <GenericDaoMapperImpl as GenericDaoMapper>::search(&self.pool, location, selection)
                .await?;
        return Ok(result);
    }

    #[inline]
    pub async fn search_paged<'e, EX, L, S, SE>(
        &self,
        location: L,
        selection: S,
    ) -> Result<PagedList<SE>, LunaOrmError>
    where
        L: Location + Send,
        S: Selection + Send,
        SE: SelectedEntity + Send + Unpin,
    {
        let result: PagedList<SE> = <GenericDaoMapperImpl as GenericDaoMapper>::search_paged(
            &self.pool, location, selection,
        )
        .await?;
        return Ok(result);
    }

    #[inline]
    pub async fn purify<'e, EX, L>(&self, location: L) -> Result<usize, LunaOrmError>
    where
        L: Location + Send,
    {
        let result: usize =
            <GenericDaoMapperImpl as GenericDaoMapper>::purify(&self.pool, location).await?;
        return Ok(result);
    }

    #[inline]
    pub async fn change<'e, EX, L, M>(
        &self,
        location: L,
        mutation: M,
    ) -> Result<usize, LunaOrmError>
    where
        L: Location + Send,
        M: Mutation + Send,
    {
        let result: usize =
            <GenericDaoMapperImpl as GenericDaoMapper>::change(&self.pool, location, mutation)
                .await?;
        return Ok(result);
    }
}
