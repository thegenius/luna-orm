use std::marker::PhantomData;
use crate::error::LunaOrmError;
use crate::LunaOrmResult;

use sqlx::mysql::MySqlQueryResult;
use sqlx::query::Query;
use sqlx::sqlite::SqliteQueryResult;
use sqlx::{Arguments, Database,  Executor, IntoArguments,  Pool, Row};
use luna_orm_trait::schema_trait::SelectedEntityNew;

pub trait GetAffectedRows {
    fn get_affected_rows(&self) -> u64;
}
impl GetAffectedRows for SqliteQueryResult {
    fn get_affected_rows(&self) -> u64 {
        self.rows_affected()
    }
}

impl GetAffectedRows for MySqlQueryResult {
    fn get_affected_rows(&self) -> u64 {
        self.rows_affected()
    }
}

pub trait SqlExecutorNew {
    type DB: Database;

    fn new_get_pool(&mut self) -> LunaOrmResult<&Pool<Self::DB>> {
        Err(LunaOrmError::NotImplement)
    }

    fn get_affected_rows(
        &mut self,
        query_result: &<Self::DB as Database>::QueryResult,
    ) -> LunaOrmResult<u64>
    {
        Err(LunaOrmError::NotImplement)
    }

    async fn new_fetch_optional_plain<'a, EX, SE, A>(
        &mut self,
        pool: EX,
        stmt: &'a str,
        selection: &SE::Selection,
        _: PhantomData<A>,
    ) -> LunaOrmResult<Option<SE>>
    where
        EX: Executor<'a, Database = Self::DB>,
        SE: SelectedEntityNew<Self::DB> + Send + Unpin,
        A: IntoArguments<'a, Self::DB> + 'a + Default,
    {
        let query: Query<'a, Self::DB, A> = sqlx::query_with(stmt, Default::default());
        let result_opt: Option<<Self::DB as Database>::Row> = query.fetch_optional(pool).await?;
        if let Some(result) = result_opt {
            Ok(Some(SE::from_row(selection, result)?))
        } else {
            Ok(None)
        }
    }
    async fn new_fetch_optional<'a, EX, SE, A>(
        &mut self,
        pool: EX,
        stmt: &'a str,
        selection: &SE::Selection,
        args: A
    ) -> LunaOrmResult<Option<SE>>
    where
        EX: Executor<'a, Database = Self::DB>,
        SE: SelectedEntityNew<Self::DB> + Send + Unpin,
        A: IntoArguments<'a, Self::DB> + 'a,
    {
        let query: Query<'a, Self::DB, A> = sqlx::query_with(stmt, args);
        let result_opt: Option<<Self::DB as Database>::Row> = query.fetch_optional(pool).await?;
        if let Some(result) = result_opt {
            Ok(Some(SE::from_row(selection, result)?))
        } else {
            Ok(None)
        }
    }


    async fn new_fetch_all_plain<'a, EX, SE, A>(
        &mut self,
        pool: EX,
        stmt: &'a str,
        selection: &SE::Selection,
        _: PhantomData<A>
    ) -> LunaOrmResult<Vec<SE>>
    where
        EX: Executor<'a, Database = Self::DB>,
        SE: SelectedEntityNew<Self::DB> + Send + Unpin,
        A: IntoArguments<'a, Self::DB> + 'a + Default,
    {
        let query: Query<'a, Self::DB, A> = sqlx::query_with(stmt, Default::default());
        let result_opt: Vec<<Self::DB as Database>::Row> = query.fetch_all(pool).await?;
        let mut result: Vec<SE> = Vec::new();
        for row in result_opt {
            let selected_result = SE::from_row(selection, row);
            if let Ok(selected_entity) = selected_result {
                result.push(selected_entity);
            } else {
                return Err(LunaOrmError::FromRowToEntityError);
            }
        }
        Ok(result)
    }

    async fn new_fetch_all<'a, EX, SE, A>(
        &mut self,
        pool: EX,
        stmt: &'a str,
        selection: &SE::Selection,
        args: A
    ) -> LunaOrmResult<Vec<SE>>
    where
        EX: Executor<'a, Database = Self::DB>,
        SE: SelectedEntityNew<Self::DB> + Send + Unpin,
        A: IntoArguments<'a, Self::DB> + 'a + Default,
    {
        let query: Query<'a, Self::DB, A> = sqlx::query_with(stmt, args);
        let result_opt: Vec<<Self::DB as Database>::Row> = query.fetch_all(pool).await?;
        let mut result: Vec<SE> = Vec::new();
        for row in result_opt {
            let selected_result = SE::from_row(selection, row);
            if let Ok(selected_entity) = selected_result {
                result.push(selected_entity);
            } else {
                return Err(LunaOrmError::FromRowToEntityError);
            }
        }
        Ok(result)
    }

    async fn new_execute_plain<'a, EX, A>(
        &mut self,
        pool: EX,
        query: &'a str,
        _args: PhantomData<A>
    ) -> LunaOrmResult<u64>
    where
        EX: Executor<'a, Database = Self::DB>,
        A: IntoArguments<'a, Self::DB> + 'a + Default,
    {
        let query: Query<'a, Self::DB, A> = sqlx::query_with(query, Default::default());
        let result: <Self::DB as Database>::QueryResult = query.execute(pool).await?;
        self.get_affected_rows(&result)
    }

    async fn new_execute<'a, EX, A>(
        &mut self,
        pool: EX,
        query: &'a str,
        args: A,
    ) -> LunaOrmResult<u64>
    where
        EX: Executor<'a, Database = Self::DB>,
        A: IntoArguments<'a, Self::DB> + 'a,
    {
        let query: Query<'a, Self::DB, A> = sqlx::query_with(query, args);
        let result: <Self::DB as Database>::QueryResult = query.execute(pool).await?;
        self.get_affected_rows(&result)
    }
}
