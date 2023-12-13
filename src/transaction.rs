use std::marker::PhantomData;

use crate::error::LunaOrmError;
use crate::mapper::{GenericDaoMapper, GenericDaoMapperImpl};
use crate::sql_generator::SqlGenerator;
use crate::LunaOrmResult;
use sqlx::any::AnyArguments;
use sqlx::any::AnyQueryResult;
use sqlx::any::AnyRow;

use crate::command_executor::CommandExecutor;
use crate::sql_executor::SqlExecutor;
use async_trait::async_trait;
use luna_orm_trait::SqlxError;
use luna_orm_trait::{Entity, Location, Mutation, PagedList, Primary, SelectedEntity, Selection};

pub struct Transaction<'a, G>
where
    G: SqlGenerator + Sync,
{
    transaction: sqlx::Transaction<'a, sqlx::Any>,
    sql_generator: &'a G,
}

impl<'a, G> CommandExecutor for Transaction<'a, G>
where
    G: SqlGenerator + Sync,
{
    type G = G;
    fn get_generator(&self) -> &Self::G {
        &self.sql_generator
    }
}

#[async_trait]
impl<'a, G> SqlExecutor for Transaction<'a, G>
where
    G: SqlGenerator + Sync,
{
    async fn fetch_optional<SE>(
        &mut self,
        stmt: &str,
        args: AnyArguments<'_>,
    ) -> LunaOrmResult<Option<SE>>
    where
        SE: SelectedEntity + Send + Unpin,
    {
        let query = sqlx::query_with(stmt, args).try_map(|row: AnyRow| SE::from_any_row(row));
        let result_opt: Option<SE> = query.fetch_optional(&mut *self.transaction).await?;
        Ok(result_opt)
    }

    async fn fetch_all<SE>(&mut self, stmt: &str, args: AnyArguments<'_>) -> LunaOrmResult<Vec<SE>>
    where
        SE: SelectedEntity + Send + Unpin,
    {
        let query = sqlx::query_with(stmt, args).try_map(|row: AnyRow| SE::from_any_row(row));
        let result_vec: Vec<SE> = query.fetch_all(&mut *self.transaction).await?;
        Ok(result_vec)
    }
    async fn execute(
        &mut self,
        stmt: &str,
        args: AnyArguments<'_>,
    ) -> LunaOrmResult<AnyQueryResult> {
        Ok(sqlx::query_with(stmt, args)
            .execute(&mut *self.transaction)
            .await?)
    }

    async fn execute_plain(&mut self, stmt: &str) -> LunaOrmResult<AnyQueryResult> {
        Ok(sqlx::query(stmt).execute(&mut *self.transaction).await?)
    }
}

impl<'a, G> Transaction<'a, G>
where
    G: SqlGenerator + Sync,
{
    pub fn new(trx: sqlx::Transaction<'a, sqlx::Any>, sql_generator: &'a G) -> Self {
        Self {
            transaction: trx,
            sql_generator,
        }
    }

    #[inline]
    pub async fn commit(self) -> Result<(), LunaOrmError> {
        Ok(self.transaction.commit().await?)
    }

    #[inline]
    pub async fn rollback(self) -> Result<(), LunaOrmError> {
        Ok(self.transaction.rollback().await?)
    }

    pub async fn query(&mut self, sql: &str) -> Result<usize, LunaOrmError> {
        let result = sqlx::query(sql).execute(&mut *self.transaction).await?;
        Ok(result.rows_affected() as usize)
    }

    /*
    async fn fetch_optional<SE>(
        &mut self,
        stmt: &str,
        args: AnyArguments<'_>,
    ) -> Result<Option<SE>, LunaOrmError>
    where
        SE: SelectedEntity + Send + Unpin,
    {
        let query = sqlx::query_with(stmt, args).try_map(|row: AnyRow| SE::from_any_row(row));
        let result_opt: Option<SE> = query.fetch_optional(&mut *self.transaction).await?;
        Ok(result_opt)
    }

    async fn fetch_all<'e, SE>(
        &mut self,
        stmt: &str,
        args: AnyArguments<'_>,
    ) -> Result<Vec<SE>, LunaOrmError>
    where
        SE: SelectedEntity + Send + Unpin,
    {
        let query = sqlx::query_with(stmt, args).try_map(|row: AnyRow| SE::from_any_row(row));
        let result_vec: Vec<SE> = query.fetch_all(&mut *self.transaction).await?;
        Ok(result_vec)
    }

    async fn execute(
        &mut self,
        stmt: &str,
        args: AnyArguments<'_>,
    ) -> Result<AnyQueryResult, LunaOrmError> {
        Ok(sqlx::query_with(stmt, args)
            .execute(&mut *self.transaction)
            .await?)
    }

    #[inline]
    pub async fn select<'e, P, S, SE>(
        &mut self,
        primary: P,
        selection: S,
    ) -> Result<Option<SE>, LunaOrmError>
    where
        P: Primary + Send,
        S: Selection + Send,
        SE: SelectedEntity + Send + Unpin,
    {
        let sql = self.sql_generator.get_select_sql(&selection, &primary);
        let args = primary.into_any_arguments();
        let result: Option<SE> = self.fetch_optional(&sql, args).await?;
        Ok(result)
    }

    #[inline]
    pub async fn create<'e, E>(&mut self, entity: E) -> Result<E, LunaOrmError>
    where
        E: Entity + Send + Clone,
    {
        let result: E =
            <GenericDaoMapperImpl as GenericDaoMapper>::create(&mut *self.transaction, entity)
                .await?;
        return Ok(result);
    }

    #[inline]
    pub async fn insert<'e, E>(&mut self, entity: E) -> Result<bool, LunaOrmError>
    where
        E: Entity + Send + Clone,
    {
        let result: bool =
            <GenericDaoMapperImpl as GenericDaoMapper>::insert(&mut *self.transaction, entity)
                .await?;
        return Ok(result);
    }

    #[inline]
    pub async fn upsert<'e, E>(&mut self, entity: E) -> Result<bool, LunaOrmError>
    where
        E: Entity + Send + Clone,
    {
        let result: bool =
            <GenericDaoMapperImpl as GenericDaoMapper>::upsert(&mut *self.transaction, entity)
                .await?;
        return Ok(result);
    }

    #[inline]
    pub async fn update<'e, E>(&mut self, entity: E) -> Result<bool, LunaOrmError>
    where
        E: Entity + Send + Clone,
    {
        let result: bool =
            <GenericDaoMapperImpl as GenericDaoMapper>::update(&mut *self.transaction, entity)
                .await?;
        return Ok(result);
    }

    #[inline]
    pub async fn remove<'e, P, E>(&mut self, primary: P) -> Result<E, LunaOrmError>
    where
        P: Primary + Send,
        E: Entity + Send + Clone,
    {
        let result: E =
            <GenericDaoMapperImpl as GenericDaoMapper>::remove(&mut *self.transaction, primary)
                .await?;
        return Ok(result);
    }

    #[inline]
    pub async fn delete<'e, P>(&mut self, primary: P) -> Result<bool, LunaOrmError>
    where
        P: Primary + Send,
    {
        let result: bool =
            <GenericDaoMapperImpl as GenericDaoMapper>::delete(&mut *self.transaction, primary)
                .await?;
        return Ok(result);
    }

    #[inline]
    pub async fn search<'e, EX, L, S, SE>(
        &mut self,
        location: L,
        selection: S,
    ) -> Result<Vec<SE>, LunaOrmError>
    where
        L: Location + Send,
        S: Selection + Send,
        SE: SelectedEntity + Send + Unpin,
    {
        let result: Vec<SE> = <GenericDaoMapperImpl as GenericDaoMapper>::search(
            &mut *self.transaction,
            location,
            selection,
        )
        .await?;
        return Ok(result);
    }

    #[inline]
    pub async fn search_paged<'e, EX, L, S, SE>(
        &mut self,
        location: L,
        selection: S,
    ) -> Result<PagedList<SE>, LunaOrmError>
    where
        L: Location + Send,
        S: Selection + Send,
        SE: SelectedEntity + Send + Unpin,
    {
        let result: PagedList<SE> = <GenericDaoMapperImpl as GenericDaoMapper>::search_paged(
            &mut *self.transaction,
            location,
            selection,
        )
        .await?;
        return Ok(result);
    }

    #[inline]
    pub async fn purify<'e, EX, L>(&mut self, location: L) -> Result<usize, LunaOrmError>
    where
        L: Location + Send,
    {
        let result: usize =
            <GenericDaoMapperImpl as GenericDaoMapper>::purify(&mut *self.transaction, location)
                .await?;
        return Ok(result);
    }

    #[inline]
    pub async fn change<'e, EX, L, M>(
        &mut self,
        location: L,
        mutation: M,
    ) -> Result<usize, LunaOrmError>
    where
        L: Location + Send,
        M: Mutation + Send,
    {
        let result: usize = <GenericDaoMapperImpl as GenericDaoMapper>::change(
            &mut *self.transaction,
            location,
            mutation,
        )
        .await?;
        return Ok(result);
    }
    */
}
