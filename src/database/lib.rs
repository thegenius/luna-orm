use crate::error::LunaOrmError;
use crate::mapper::{GenericDaoMapper, GenericDaoMapperImpl};
use crate::sql_generator::{DefaultSqlGenerator, SqlGenerator};
use crate::transaction::Transaction;
use crate::LunaOrmResult;
use std::ops::{Deref, DerefMut};

use async_trait::async_trait;
use luna_orm_trait::*;
use path_absolutize::*;
use sqlx::any::AnyConnectOptions;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePool, SqliteSynchronous};

use sqlx::any::AnyArguments;
use sqlx::any::AnyQueryResult;
use sqlx::any::AnyRow;
use sqlx::Any;
use sqlx::AnyPool;
use sqlx::Executor;
use std::fs;
use std::path::Path;
use std::str::FromStr;

#[async_trait]
pub trait Database {
    fn get_type(&self) -> DatabaseType;
    fn get_pool(&self) -> &AnyPool;
    fn get_generator(&self) -> &dyn SqlGenerator;
    //fn transaction<'a>(&self) -> LunaOrmResult<Transaction<'a>>;

    async fn transaction<'a>(&self) -> LunaOrmResult<Transaction<'a>> {
        let trx = self.get_pool().begin().await?;
        let transaction = Transaction::new(trx);
        return Ok(transaction);
    }

    #[inline]
    async fn query(&self, sql: &str) -> Result<usize, LunaOrmError> {
        let result = sqlx::query(sql).execute(self.get_pool()).await?;
        return Ok(result.rows_affected() as usize);
    }

    #[inline]
    async fn select<'e, P, S, SE>(
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
            <GenericDaoMapperImpl as GenericDaoMapper>::select(self.get_pool(), primary, selection)
                .await?;
        return Ok(result);
    }

    #[inline]
    async fn create<'e, E>(&self, entity: E) -> Result<E, LunaOrmError>
    where
        E: Entity + Send + Clone,
    {
        let result: E =
            <GenericDaoMapperImpl as GenericDaoMapper>::create(self.get_pool(), entity).await?;
        return Ok(result);
    }

    #[inline]
    async fn insert<'e, E>(&self, entity: E) -> Result<bool, LunaOrmError>
    where
        E: Entity + Send + Clone,
    {
        let result: bool =
            <GenericDaoMapperImpl as GenericDaoMapper>::insert(self.get_pool(), entity).await?;
        return Ok(result);
    }

    #[inline]
    async fn upsert<'e, E>(&self, entity: E) -> Result<bool, LunaOrmError>
    where
        E: Entity + Send + Clone,
    {
        let result: bool =
            <GenericDaoMapperImpl as GenericDaoMapper>::upsert(self.get_pool(), entity).await?;
        return Ok(result);
    }

    #[inline]
    async fn update<'e, E>(&self, entity: E) -> Result<bool, LunaOrmError>
    where
        E: Entity + Send + Clone,
    {
        let result: bool =
            <GenericDaoMapperImpl as GenericDaoMapper>::update(self.get_pool(), entity).await?;
        return Ok(result);
    }

    #[inline]
    async fn remove<'e, P, E>(&self, primary: P) -> Result<E, LunaOrmError>
    where
        P: Primary + Send,
        E: Entity + Send + Clone,
    {
        let result: E =
            <GenericDaoMapperImpl as GenericDaoMapper>::remove(self.get_pool(), primary).await?;
        return Ok(result);
    }

    #[inline]
    async fn delete<'e, P>(&self, primary: P) -> Result<bool, LunaOrmError>
    where
        P: Primary + Send,
    {
        let result: bool =
            <GenericDaoMapperImpl as GenericDaoMapper>::delete(self.get_pool(), primary).await?;
        return Ok(result);
    }

    #[inline]
    async fn search<'e, EX, L, S, SE>(
        &self,
        location: L,
        selection: S,
    ) -> Result<Vec<SE>, LunaOrmError>
    where
        L: Location + Send,
        S: Selection + Send,
        SE: SelectedEntity + Send + Unpin,
    {
        let result: Vec<SE> = <GenericDaoMapperImpl as GenericDaoMapper>::search(
            self.get_pool(),
            location,
            selection,
        )
        .await?;
        return Ok(result);
    }

    #[inline]
    async fn search_paged<'e, EX, L, S, SE>(
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
            self.get_pool(),
            location,
            selection,
        )
        .await?;
        return Ok(result);
    }

    #[inline]
    async fn purify<'e, EX, L>(&self, location: L) -> Result<usize, LunaOrmError>
    where
        L: Location + Send,
    {
        let result: usize =
            <GenericDaoMapperImpl as GenericDaoMapper>::purify(self.get_pool(), location).await?;
        return Ok(result);
    }

    #[inline]
    async fn change<'e, EX, L, M>(&self, location: L, mutation: M) -> Result<usize, LunaOrmError>
    where
        L: Location + Send,
        M: Mutation + Send,
    {
        let result: usize =
            <GenericDaoMapperImpl as GenericDaoMapper>::change(self.get_pool(), location, mutation)
                .await?;
        return Ok(result);
    }

    async fn fetch_optional<'e, EX, SE>(
        &self,
        executor: EX,
        stmt: &str,
        args: AnyArguments<'_>,
    ) -> Result<Option<SE>, SqlxError>
    where
        EX: 'e + Executor<'e, Database = Any>,
        SE: SelectedEntity + Send + Unpin,
    {
        let query = sqlx::query_with(stmt, args).try_map(|row: AnyRow| SE::from_any_row(row));
        let result_opt: Option<SE> = query.fetch_optional(executor).await?;
        Ok(result_opt)
    }

    async fn fetch_all<'e, EX, SE>(
        &self,
        executor: EX,
        stmt: &str,
        args: AnyArguments<'_>,
    ) -> Result<Vec<SE>, SqlxError>
    where
        EX: 'e + Executor<'e, Database = Any>,
        SE: SelectedEntity + Send + Unpin,
    {
        let query = sqlx::query_with(stmt, args).try_map(|row: AnyRow| SE::from_any_row(row));
        let result_vec: Vec<SE> = query.fetch_all(executor).await?;
        Ok(result_vec)
    }

    async fn execute<'e, EX>(
        &self,
        executor: EX,
        stmt: &str,
        args: AnyArguments<'_>,
    ) -> Result<AnyQueryResult, SqlxError>
    where
        EX: 'e + Executor<'e, Database = Any>,
    {
        Ok(sqlx::query_with(stmt, args).execute(executor).await?)
    }
}

pub enum DatabaseType {
    SqliteLocal,
    MySql,
    PostgreSql,
}

pub trait DatabaseExecutor<'a> {
    const SQL_GENERATOR: &'a dyn SqlGenerator = &DefaultSqlGenerator {};
}

pub struct DB<T: Database>(pub T);

impl<T> Deref for DB<T>
where
    T: Database,
{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for DB<T>
where
    T: Database,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
