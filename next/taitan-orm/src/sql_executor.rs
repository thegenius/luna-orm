use crate::error::TaitanOrmError;
use crate::result::Result;
use std::marker::PhantomData;

use sqlx::query::Query;
use sqlx::{Database, Executor, IntoArguments, Pool};
use taitan_orm_trait::SelectedEntity;

pub trait SqlExecutor {
    type DB: Database;

    fn get_pool(&mut self) -> Result<&Pool<Self::DB>> {
        Err(TaitanOrmError::NotImplement("get_pool".to_string()))
    }

    fn get_affected_rows(
        &mut self,
        _query_result: &<Self::DB as Database>::QueryResult,
    ) -> Result<u64> {
        Err(TaitanOrmError::NotImplement(
            "get_affected_rows".to_string(),
        ))
    }

    async fn fetch_optional_plain<'a, SE>(
        &'a mut self,
        stmt: &'a str,
        selection: &'a SE::Selection,
    ) -> Result<Option<SE>>
    where
        SE: SelectedEntity<Self::DB> + Send + Unpin;

    async fn fetch_optional<'a, SE>(
        &'a mut self,
        stmt: &'a str,
        selection: &'a SE::Selection,
        args: <Self::DB as Database>::Arguments<'a>,
    ) -> Result<Option<SE>>
    where
        SE: SelectedEntity<Self::DB> + Send + Unpin;

    async fn fetch_exists<'a>(
        &'a mut self,
        stmt: &'a str,
        args: <Self::DB as Database>::Arguments<'a>,
    ) -> Result<bool>;

    async fn fetch_all_plain<'a, SE>(
        &'a mut self,
        stmt: &'a str,
        selection: &'a SE::Selection,
    ) -> Result<Vec<SE>>
    where
        SE: SelectedEntity<Self::DB> + Send + Unpin;

    async fn fetch_all<'a, SE>(
        &'a mut self,
        stmt: &'a str,
        selection: &'a SE::Selection,
        args: <Self::DB as Database>::Arguments<'a>,
    ) -> Result<Vec<SE>>
    where
        SE: SelectedEntity<Self::DB> + Send + Unpin;

    async fn execute_plain<'a>(&'a mut self, stmt: &'a str) -> Result<u64>;

    async fn execute<'a, A>(
        &'a mut self,
        stmt: &'a str,
        args: <Self::DB as Database>::Arguments<'a>,
    ) -> Result<u64>;

    async fn fetch_execute_plain<'a, SE>(&'a mut self, stmt: &'a str) -> Result<SE>
    where
        SE: SelectedEntity<Self::DB> + Send + Unpin;

    async fn fetch_execute<'a, SE>(
        &'a mut self,
        stmt: &'a str,
        args: <Self::DB as Database>::Arguments<'a>,
    ) -> Result<SE>
    where
        SE: SelectedEntity<Self::DB> + Send + Unpin;

    async fn fetch_execute_option<'a, SE>(
        &'a mut self,
        stmt: &'a str,
        args: <Self::DB as Database>::Arguments<'a>,
    ) -> Result<Option<SE>>
    where
        SE: SelectedEntity<Self::DB> + Send + Unpin;

    // async fn fetch_execute_all<'a, SE>(
    //     &'a mut self,
    //     stmt: &'a str,
    //     args: <Self::DB as Database>::Arguments<'a>,
    // ) -> Result<Vec<SE>>
    // where
    //     SE: SelectedEntity<Self::DB> + Send + Unpin;

    async fn generic_fetch_optional_plain<'a, EX, SE, A>(
        &mut self,
        ex: EX,
        stmt: &'a str,
        selection: &SE::Selection,
        _: PhantomData<A>,
    ) -> Result<Option<SE>>
    where
        EX: Executor<'a, Database = Self::DB>,
        SE: SelectedEntity<Self::DB> + Send + Unpin,
        A: IntoArguments<'a, Self::DB> + 'a + Default,
    {
        let query: Query<'a, Self::DB, A> = sqlx::query_with(stmt, Default::default());
        let result_opt: Option<<Self::DB as Database>::Row> = query.fetch_optional(ex).await?;
        if let Some(result) = result_opt {
            Ok(Some(SE::from_row(selection, result)?))
        } else {
            Ok(None)
        }
    }

    async fn generic_fetch_execute_plain<'a, EX, SE, A>(
        &mut self,
        ex: EX,
        stmt: &'a str,
        _: PhantomData<A>,
    ) -> Result<SE>
    where
        EX: Executor<'a, Database = Self::DB>,
        SE: SelectedEntity<Self::DB> + Send + Unpin,
        A: IntoArguments<'a, Self::DB> + 'a + Default,
    {
        let query: Query<'a, Self::DB, A> = sqlx::query_with(stmt, Default::default());
        let result_opt: Option<<Self::DB as Database>::Row> = query.fetch_optional(ex).await?;
        if let Some(result) = result_opt {
            Ok(SE::from_row_full(result)?)
        } else {
            Err(sqlx::error::Error::RowNotFound.into())
        }
    }

    async fn generic_fetch_execute<'a, EX, SE, A>(
        &mut self,
        ex: EX,
        stmt: &'a str,
        args: A,
    ) -> Result<SE>
    where
        EX: Executor<'a, Database = Self::DB>,
        SE: SelectedEntity<Self::DB> + Send + Unpin,
        A: IntoArguments<'a, Self::DB> + 'a,
    {
        let query: Query<'a, Self::DB, A> = sqlx::query_with(stmt, args);
        let result_opt: Option<<Self::DB as Database>::Row> = query.fetch_optional(ex).await?;
        if let Some(result) = result_opt {
            Ok(SE::from_row_full(result)?)
        } else {
            Err(sqlx::error::Error::RowNotFound.into())
        }
    }
    async fn generic_fetch_execute_option<'a, EX, SE, A>(
        &mut self,
        ex: EX,
        stmt: &'a str,
        args: A,
    ) -> Result<Option<SE>>
    where
        EX: Executor<'a, Database = Self::DB>,
        SE: SelectedEntity<Self::DB> + Send + Unpin,
        A: IntoArguments<'a, Self::DB> + 'a,
    {
        let query: Query<'a, Self::DB, A> = sqlx::query_with(stmt, args);
        let result_opt: Option<<Self::DB as Database>::Row> = query.fetch_optional(ex).await?;
        if let Some(result) = result_opt {
            Ok(Some(SE::from_row_full(result)?))
        } else {
            Ok(None)
        }
    }



    async fn generic_exists<'a, EX, A>(
        &mut self,
        ex: EX,
        stmt: &'a str,
        args: A,
    ) -> Result<bool>
    where
        EX: Executor<'a, Database = Self::DB>,
        A: IntoArguments<'a, Self::DB> + 'a,
    {
        let query: Query<'a, Self::DB, A> = sqlx::query_with(stmt, args);
        let result_opt: Option<<Self::DB as Database>::Row> = query.fetch_optional(ex).await?;
        if let Some(_) = result_opt {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    async fn generic_fetch_optional<'a, EX, SE, A>(
        &mut self,
        ex: EX,
        stmt: &'a str,
        selection: &SE::Selection,
        args: A,
    ) -> Result<Option<SE>>
    where
        EX: Executor<'a, Database = Self::DB>,
        SE: SelectedEntity<Self::DB> + Send + Unpin,
        A: IntoArguments<'a, Self::DB> + 'a,
    {
        let query: Query<'a, Self::DB, A> = sqlx::query_with(stmt, args);
        let result_opt: Option<<Self::DB as Database>::Row> = query.fetch_optional(ex).await?;
        if let Some(result) = result_opt {
            Ok(Some(SE::from_row(selection, result)?))
        } else {
            Ok(None)
        }
    }

    async fn generic_fetch_all_plain<'a, EX, SE, A>(
        &mut self,
        ex: EX,
        stmt: &'a str,
        selection: &SE::Selection,
        _: PhantomData<A>,
    ) -> Result<Vec<SE>>
    where
        EX: Executor<'a, Database = Self::DB>,
        SE: SelectedEntity<Self::DB> + Send + Unpin,
        A: IntoArguments<'a, Self::DB> + 'a + Default,
    {
        let query: Query<'a, Self::DB, A> = sqlx::query_with(stmt, Default::default());
        let result_opt: Vec<<Self::DB as Database>::Row> = query.fetch_all(ex).await?;
        let mut result: Vec<SE> = Vec::new();
        for row in result_opt {
            let selected_result = SE::from_row(selection, row);
            if let Ok(selected_entity) = selected_result {
                result.push(selected_entity);
            } else {
                return Err(TaitanOrmError::FromRowToEntityError);
            }
        }
        Ok(result)
    }

    async fn generic_fetch_all<'a, EX, SE, A>(
        &mut self,
        ex: EX,
        stmt: &'a str,
        selection: &SE::Selection,
        args: A,
    ) -> Result<Vec<SE>>
    where
        EX: Executor<'a, Database = Self::DB>,
        SE: SelectedEntity<Self::DB> + Send + Unpin,
        A: IntoArguments<'a, Self::DB> + 'a + Default,
    {
        let query: Query<'a, Self::DB, A> = sqlx::query_with(stmt, args);
        let result_opt: Vec<<Self::DB as Database>::Row> = query.fetch_all(ex).await?;
        let mut result: Vec<SE> = Vec::new();
        for row in result_opt {
            let selected_result = SE::from_row(selection, row);
            if let Ok(selected_entity) = selected_result {
                result.push(selected_entity);
            } else {
                return Err(TaitanOrmError::FromRowToEntityError);
            }
        }
        Ok(result)
    }

    async fn generic_execute_plain<'a, EX, A>(
        &mut self,
        ex: EX,
        query: &'a str,
        _args: PhantomData<A>,
    ) -> Result<u64>
    where
        EX: Executor<'a, Database = Self::DB>,
        A: IntoArguments<'a, Self::DB> + 'a + Default,
    {
        let query: Query<'a, Self::DB, A> = sqlx::query_with(query, Default::default());
        let result: <Self::DB as Database>::QueryResult = query.execute(ex).await?;
        self.get_affected_rows(&result)
    }

    async fn generic_execute<'a, EX, A>(&mut self, ex: EX, query: &'a str, args: A) -> Result<u64>
    where
        EX: Executor<'a, Database = Self::DB>,
        A: IntoArguments<'a, Self::DB> + 'a,
    {
        let query: Query<'a, Self::DB, A> = sqlx::query_with(query, args);
        let result: <Self::DB as Database>::QueryResult = query.execute(ex).await?;
        self.get_affected_rows(&result)
    }
}
