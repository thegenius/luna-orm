use std::marker::PhantomData;
use crate::database::sqlite::commanders::read::SqliteReadCommander;
use crate::database::sqlite::{SqliteDatabase, SqliteWriteCommander};
use crate::result::Result;
use crate::sql_generator::DefaultSqlGenerator;
use crate::sql_generator_container::SqlGeneratorContainer;
use crate::{execute_fn, SqlExecutor, TaitanOrmError};
use sqlx::query::Query;
use sqlx::sqlite::{SqliteArguments, SqliteQueryResult};
use sqlx::{query_with, Database, Executor, IntoArguments, Sqlite, SqliteConnection};
use sqlx::pool::PoolConnection;
use taitan_orm_trait::{Mutation, SelectedEntity, Unique};
use tracing::debug;
use crate::sql_generic_executor::SqlGenericExecutor;

#[derive(Debug)]
pub struct SqliteTransaction<'a> {
    transaction: sqlx::Transaction<'a, Sqlite>,
    sql_generator: &'a DefaultSqlGenerator,
}

impl<'a> SqliteTransaction<'a> {
    pub fn new(trx: sqlx::Transaction<'a, Sqlite>, sql_generator: &'a DefaultSqlGenerator) -> Self {
        Self {
            transaction: trx,
            sql_generator,
        }
    }

    #[inline]
    pub async fn commit(self) -> Result<()> {
        Ok(self.transaction.commit().await?)
    }

    #[inline]
    pub async fn rollback(self) -> Result<()> {
        Ok(self.transaction.rollback().await?)
    }
}

impl<'a> SqlGenericExecutor for SqliteTransaction<'a> {
    type DB = Sqlite;

    fn get_affected_rows(query_result: &<Self::DB as Database>::QueryResult) -> u64 {
        query_result.rows_affected()
    }
}

impl<'s> SqlExecutor for SqliteTransaction<'s> {

    type Connection = SqliteConnection;

    async fn execute<'a, A>(&'a mut self, stmt: &'a str, args: A) -> Result<u64>
    where
        A: IntoArguments<'a, Self::DB> + 'a
    {
        let ex = &mut *self.transaction;
        Self::generic_execute(ex, stmt, args).await
    }

    // async fn execute<'a, A>(&'a mut self, stmt: &'a str, args: A) -> Result<u64>
    // where
    //     A: IntoArguments<'a, Self::DB> + 'a
    // {
    //         let ex = &mut * self.transaction;
    //         Self::generic_execute(ex, stmt, args).await
    // }


    // async fn execute<'a>(&'a mut self, stmt: &'a str, args: SqliteArguments<'a>) -> Result<u64> {
    //     let ex = &mut * self.transaction;
    //     Self::generic_execute(ex, stmt, args).await
    // }



    async fn execute_plain<'a>(&'a mut self, stmt: &'a str) -> Result<u64> {
        let args: PhantomData<SqliteArguments> = PhantomData::default();
        Self::generic_execute_plain(&mut *(self.transaction), stmt, args).await
    }
    // execute_plain_fn!(SqliteArguments, self.transaction);

    async fn fetch_exists<'a>(
        &'a mut self,
        stmt: &'a str,
        args: <Self::DB as Database>::Arguments<'a>,
    ) -> Result<bool> {
        let query: Query<'a, Self::DB, SqliteArguments<'a>> = query_with(stmt, args);
        let result_opt: Option<<Self::DB as Database>::Row> =
            query.fetch_optional(&mut *self.transaction).await?;
        if let Some(_) = result_opt {
            Ok(true)
        } else {
            Ok(false)
        }
    }
    async fn fetch_exists_plain<'a, A>(&'a mut self, stmt: &'a str) -> Result<bool>
    where
        A: IntoArguments<'a, Self::DB> + 'a + Default
    {
        let query: Query<'a, Self::DB, SqliteArguments<'a>> = query_with(stmt, Default::default());
        let result_opt: Option<<Self::DB as Database>::Row> =
            query.fetch_optional(&mut *self.transaction).await?;
        if let Some(_) = result_opt {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    async fn fetch_option<'a, SE>(
        &'a mut self,
        stmt: &'a str,
        selection: &'a SE::Selection,
        args: SqliteArguments<'a>,
    ) -> Result<Option<SE>>
    where
        SE: SelectedEntity<Self::DB> + Send + Unpin,
    {
        let query: Query<'a, Self::DB, SqliteArguments<'a>> = query_with(stmt, args);
        let result_opt: Option<<Self::DB as Database>::Row> =
            query.fetch_optional(&mut *self.transaction).await?;
        if let Some(result) = result_opt {
            Ok(Some(SE::from_row(selection, result)?))
        } else {
            Ok(None)
        }
    }

    async fn fetch_option_plain<'a, SE>(
        &'a mut self,
        stmt: &'a str,
        selection: &'a SE::Selection,
    ) -> Result<Option<SE>>
    where
        SE: SelectedEntity<Self::DB> + Send + Unpin
    {
        // let ex = &mut *self.transaction;
        // let args: PhantomData<SqliteArguments> = PhantomData::default();
        // self.generic_fetch_option_plain(ex, stmt, selection, args)
        //     .await
        let query: Query<'a, Self::DB, SqliteArguments<'a>> = query_with(stmt, Default::default());
        let result_opt: Option<<Self::DB as Database>::Row> =
            query.fetch_optional(&mut *self.transaction).await?;
        if let Some(result) = result_opt {
            Ok(Some(SE::from_row(selection, result)?))
        } else {
            Ok(None)
        }
    }

    async fn fetch_all<'a, SE>(
        &'a mut self,
        stmt: &'a str,
        selection: &'a SE::Selection,
        args: SqliteArguments<'a>,
    ) -> Result<Vec<SE>>
    where
        SE: SelectedEntity<Self::DB> + Send + Unpin,
    {
        let query: Query<'a, Self::DB, SqliteArguments<'a>> = query_with(stmt, args);
        let result_opt: Vec<<Self::DB as Database>::Row> =
            query.fetch_all(&mut *self.transaction).await?;
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

    async fn fetch_all_plain<'a, SE>(
        &'a mut self,
        stmt: &'a str,
        selection: &'a SE::Selection,
    ) -> Result<Vec<SE>>
    where
        SE: SelectedEntity<Self::DB> + Send + Unpin,
    {
        let query: Query<'a, Self::DB, SqliteArguments<'a>> = query_with(stmt, Default::default());
        let result_opt: Vec<<Self::DB as Database>::Row> =
            query.fetch_all(&mut *self.transaction).await?;
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

    async fn fetch_one_full<'a, SE>(
        &'a mut self,
        stmt: &'a str,
        args: SqliteArguments<'a>,
    ) -> Result<SE>
    where
        SE: SelectedEntity<Self::DB> + Send + Unpin,
    {
        let query: Query<'a, Self::DB, SqliteArguments<'a>> = query_with(stmt, args);
        let result_opt: Option<<Self::DB as Database>::Row> =
            query.fetch_optional(&mut *self.transaction).await?;
        if let Some(result) = result_opt {
            Ok(SE::from_row_full(result)?)
        } else {
            Err(sqlx::error::Error::RowNotFound.into())
        }
    }

    async fn fetch_one_full_plain<'a, SE>(&'a mut self, stmt: &'a str) -> Result<SE>
    where
        SE: SelectedEntity<Self::DB> + Send + Unpin,
    {
        let query: Query<'a, Self::DB, SqliteArguments<'a>> = query_with(stmt, Default::default());
        let result_opt: Option<<Self::DB as Database>::Row> =
            query.fetch_optional(&mut *self.transaction).await?;
        if let Some(result) = result_opt {
            Ok(SE::from_row_full(result)?)
        } else {
            Err(sqlx::error::Error::RowNotFound.into())
        }
    }

    async fn fetch_option_full<'a, SE>(
        &'a mut self,
        stmt: &'a str,
        args: <Self::DB as Database>::Arguments<'a>,
    ) -> Result<Option<SE>>
    where
        SE: SelectedEntity<Self::DB> + Send + Unpin,
    {
        let ex = &mut *self.transaction;
        let query: Query<'a, Self::DB, SqliteArguments<'a>> = query_with(stmt, args);
        let result_opt: Option<<Self::DB as Database>::Row> =
            query.fetch_optional(ex).await?;
        if let Some(result) = result_opt {
            Ok(Some(SE::from_row_full(result)?))
        } else {
            Ok(None)
        }
    }

    async fn fetch_option_full_plain<'a, SE>(&'a mut self, stmt: &'a str) -> Result<Option<SE>>
    where
        SE: SelectedEntity<Self::DB> + Send + Unpin,
    {
        let ex = &mut *self.transaction;
        let query: Query<'a, Self::DB, SqliteArguments<'a>> = query_with(stmt, Default::default());
        let result_opt: Option<<Self::DB as Database>::Row> =
            query.fetch_optional(ex).await?;
        if let Some(result) = result_opt {
            Ok(Some(SE::from_row_full(result)?))
        } else {
            Ok(None)
        }
    }


    async fn fetch_all_full<'a, SE>(&'a mut self, stmt: &'a str, args: <Self::DB as Database>::Arguments<'a>) -> Result<Vec<SE>>
    where
        SE: SelectedEntity<Self::DB> + Send + Unpin
    {
        let query: Query<'a, Self::DB, SqliteArguments<'a>> = query_with(stmt, args);
        let result_vec: Vec<<Self::DB as Database>::Row> =
            query.fetch_all(&mut *self.transaction).await?;
        let mut result: Vec<SE> = Vec::new();
        for row in result_vec {
            result.push(SE::from_row_full(row)?);
        }
        Ok(result)
    }

    async fn fetch_all_full_plain<'a, SE>(&'a mut self, stmt: &'a str) -> Result<Vec<SE>>
    where
        SE: SelectedEntity<Self::DB> + Send + Unpin
    {
        let query: Query<'a, Self::DB, SqliteArguments<'a>> = query_with(stmt, Default::default());
        let result_vec: Vec<<Self::DB as Database>::Row> =
            query.fetch_all(&mut *self.transaction).await?;
        let mut result: Vec<SE> = Vec::new();
        for row in result_vec {
            result.push(SE::from_row_full(row)?);
        }
        Ok(result)
    }
}

impl<'a> SqlGeneratorContainer for SqliteTransaction<'a> {
    type G = DefaultSqlGenerator;

    fn get_generator(&mut self) -> &Self::G {
        &self.sql_generator
    }
}

impl<'a> SqliteWriteCommander for SqliteTransaction<'a> {}

impl<'a> SqliteReadCommander for SqliteTransaction<'a> {}
