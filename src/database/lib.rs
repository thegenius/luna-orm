use crate::command_executor::CommandExecutor;
use crate::error::LunaOrmError;
use crate::sql_executor::SqlExecutor;
use crate::sql_generator::{DefaultSqlGenerator, SqlGenerator};
use crate::transaction::Transaction;
use crate::LunaOrmResult;
use std::ops::{Deref, DerefMut};

use async_trait::async_trait;
use luna_orm_trait::*;

use sqlx::any::AnyArguments;
use sqlx::any::AnyQueryResult;
use sqlx::any::AnyRow;
use sqlx::Executor;
use sqlx::{AnyConnection, AnyPool};

#[async_trait]
pub trait Database: CommandExecutor + SqlExecutor {
    //type G: SqlGenerator + Sync;

    fn get_type(&self) -> &DatabaseType;
    //fn get_pool(&self) -> &AnyPool;
    //fn get_generator(&self) -> &Self::G;

    async fn transaction<'a>(&'a self) -> LunaOrmResult<Transaction<'a, Self::G>> {
        let trx = self.get_pool()?.begin().await?;
        let generator = self.get_generator();
        let transaction = Transaction::new(trx, generator);
        return Ok(transaction);
    }

    /*
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
        let sql = self.get_generator().get_select_sql(&selection, &primary);
        let args = primary.into_any_arguments();
        let result: Option<SE> = self.fetch_optional(&sql, args).await?;
        /*
        let result: Option<SE> =
            <GenericDaoMapperImpl as GenericDaoMapper>::select(self.get_pool(), primary, selection)
                .await?;
        */
        return Ok(result);
    }

    #[inline]
    async fn create<'e, E>(&self, entity: E) -> Result<E, LunaOrmError>
    where
        E: Entity + Send + Clone,
    {
        let sql = self.get_generator().get_insert_sql(&entity);
        let entity_clone = entity.clone();
        let args = entity.into_insert_any_arguments();
        self.execute(&sql, args).await?;
        return Ok(entity_clone);
        /*
        let result: E =
            <GenericDaoMapperImpl as GenericDaoMapper>::create(self.get_pool(), entity).await?;
        return Ok(result);
        */
    }

    #[inline]
    async fn insert<'e, E>(&self, entity: E) -> Result<bool, LunaOrmError>
    where
        E: Entity + Send + Clone,
    {
        let sql = self.get_generator().get_insert_sql(&entity);
        let args = entity.into_insert_any_arguments();
        let result = self.execute(&sql, args).await?;
        return Ok(result.rows_affected() > 0);

        /*
        let result: bool =
            <GenericDaoMapperImpl as GenericDaoMapper>::insert(self.get_pool(), entity).await?;
        return Ok(result);
        */
    }

    #[inline]
    async fn upsert<'e, E>(&self, entity: E) -> Result<bool, LunaOrmError>
    where
        E: Entity + Send + Clone,
    {
        let sql = self.get_generator().get_upsert_sql(&entity);
        let args = entity.into_upsert_any_arguments();
        let result = self.execute(&sql, args).await?;
        return Ok(result.rows_affected() > 0);
        /*
        let result: bool =
            <GenericDaoMapperImpl as GenericDaoMapper>::upsert(self.get_pool(), entity).await?;
        return Ok(result);
        */
    }

    #[inline]
    async fn update<'e, E>(&self, entity: E) -> Result<bool, LunaOrmError>
    where
        E: Entity + Send + Clone,
    {
        let sql = self.get_generator().get_update_sql(&entity);
        let args = entity.into_update_any_arguments();
        let result = self.execute(&sql, args).await?;
        return Ok(result.rows_affected() > 0);
        /*
        let result: bool =
            <GenericDaoMapperImpl as GenericDaoMapper>::update(self.get_pool(), entity).await?;
        return Ok(result);
        */
    }

    #[inline]
    async fn remove<'e, P, E>(&self, primary: P) -> Result<E, LunaOrmError>
    where
        P: Primary + Send,
        E: Entity + Send + Clone,
    {
        todo!()
        /*
        let sql = self.get_generator().get_delete_sql(&primary);
        let args = primary.into_any_arguments();
        let result = self.execute(&sql, args).await?;
        return Ok(result.rows_affected() > 0);
        */
        /*
        let result: E =
            <GenericDaoMapperImpl as GenericDaoMapper>::remove(self.get_pool(), primary).await?;
        return Ok(result);
        */
    }

    #[inline]
    async fn delete<'e, P>(&self, primary: P) -> Result<bool, LunaOrmError>
    where
        P: Primary + Send,
    {
        let sql = self.get_generator().get_delete_sql(&primary);
        let args = primary.into_any_arguments();
        let result = self.execute(&sql, args).await?;
        return Ok(result.rows_affected() > 0);

        /*
        let result: bool =
            <GenericDaoMapperImpl as GenericDaoMapper>::delete(self.get_pool(), primary).await?;
        return Ok(result);
        */
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
        let sql = self.get_generator().get_search_sql(&selection, &location);
        let args = location.into_any_arguments();
        let result: Vec<SE> = self.fetch_all(&sql, args).await?;
        return Ok(result);

        /*
        let result: Vec<SE> = <GenericDaoMapperImpl as GenericDaoMapper>::search(
            self.get_pool(),
            location,
            selection,
        )
        .await?;
        return Ok(result);
        */
    }

    #[inline]
    async fn search_paged<'e, EX, L, S, SE>(
        &self,
        location: L,
        selection: S,
        page: &Pagination,
    ) -> Result<PagedList<SE>, LunaOrmError>
    where
        L: Location + Send,
        S: Selection + Send,
        SE: SelectedEntity + Send + Unpin,
    {
        let sql = self
            .get_generator()
            .get_paged_search_sql(&selection, &location, page);
        let args = location.into_any_arguments();
        let entity_list: Vec<SE> = self.fetch_all(&sql, args).await?;
        let page_info = PageInfo {
            page_size: 10,
            page_num: 10,
            page_total: 10,
            total: 100,
        };
        return Ok(PagedList {
            data: entity_list,
            page: page_info,
        });
    }

    #[inline]
    async fn purify<'e, EX, L>(&self, location: L) -> Result<usize, LunaOrmError>
    where
        L: Location + Send,
    {
        let sql = self.get_generator().get_purify_sql(&location);
        let args = location.into_any_arguments();
        let result = self.execute(&sql, args).await?;
        return Ok(result.rows_affected() as usize);
    }

    #[inline]
    async fn change<'e, EX, L, M>(&self, location: L, mutation: M) -> Result<usize, LunaOrmError>
    where
        L: Location + Send,
        M: Mutation + Send,
    {
        let sql = self.get_generator().get_change_sql(&mutation, &location);
        let mut args = mutation.into_any_arguments();
        let where_args = location.into_any_arguments();
        args = merge_any_arguments(args, where_args);
        let result = self.execute(&sql, args).await?;
        return Ok(result.rows_affected() as usize);
    }
    */

    /*
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
    */
    /*
    async fn fetch_optional<SE>(
        &self,
        stmt: &str,
        args: AnyArguments<'_>,
    ) -> Result<Option<SE>, SqlxError>
    where
        SE: SelectedEntity + Send + Unpin,
    {
        let query = sqlx::query_with(stmt, args).try_map(|row: AnyRow| SE::from_any_row(row));
        let result_opt: Option<SE> = query.fetch_optional(self.get_pool()).await?;
        Ok(result_opt)
    }
    */

    /*
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
    */
    /*
    async fn fetch_all<'e, SE>(
        &self,
        stmt: &str,
        args: AnyArguments<'_>,
    ) -> Result<Vec<SE>, SqlxError>
    where
        SE: SelectedEntity + Send + Unpin,
    {
        let query = sqlx::query_with(stmt, args).try_map(|row: AnyRow| SE::from_any_row(row));
        let result_vec: Vec<SE> = query.fetch_all(self.get_pool()).await?;
        Ok(result_vec)
    }
    */
    /*
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
    */

    /*
    async fn execute(
        &self,
        stmt: &str,
        args: AnyArguments<'_>,
    ) -> Result<AnyQueryResult, SqlxError> {
        Ok(sqlx::query_with(stmt, args)
            .execute(self.get_pool())
            .await?)
    }
    */
}

pub enum DatabaseType {
    SqliteLocal,
    MySql,
    PostgreSql,
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
