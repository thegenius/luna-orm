use crate::error::LunaOrmError;
use crate::result::Result;
use std::marker::PhantomData;

use sqlx::mysql::MySqlQueryResult;
use sqlx::query::Query;
use sqlx::sqlite::SqliteQueryResult;
use sqlx::{Database, Executor, IntoArguments};
use taitan_orm_trait::SelectedEntity;

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

pub trait SqlExecutor {
    type DB: Database;

    fn get_affected_rows(&self, _query_result: &<Self::DB as Database>::QueryResult) -> Result<u64> {
        Err(LunaOrmError::NotImplement)
    }

    async fn fetch_optional_plain<'a, EX, SE, A>(
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
    async fn fetch_optional<'a, EX, SE, A>(
        &self,
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

    async fn fetch_all_plain<'a, EX, SE, A>(
        &self,
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
                return Err(LunaOrmError::FromRowToEntityError);
            }
        }
        Ok(result)
    }

    async fn fetch_all<'a, EX, SE, A>(
        &self,
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
                return Err(LunaOrmError::FromRowToEntityError);
            }
        }
        Ok(result)
    }

    async fn execute_plain<'a, EX, A>(
        &self,
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

    async fn execute<'a, EX, A>(&self, ex: EX, query: &'a str, args: A) -> Result<u64>
    where
        EX: Executor<'a, Database = Self::DB>,
        A: IntoArguments<'a, Self::DB> + 'a,
    {
        let query: Query<'a, Self::DB, A> = sqlx::query_with(query, args);
        let result: <Self::DB as Database>::QueryResult = query.execute(ex).await?;
        self.get_affected_rows(&result)
    }
}
