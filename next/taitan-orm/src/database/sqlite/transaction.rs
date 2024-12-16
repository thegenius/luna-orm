use crate::database::sqlite::SqliteCommander;
use crate::result::Result;
use crate::sql_generator::DefaultSqlGenerator;
use crate::{SqlExecutor, TaitanOrmError};
use sqlx::query::Query;
use sqlx::sqlite::{SqliteArguments, SqliteQueryResult};
use sqlx::{query_with, Database, Sqlite};
use taitan_orm_trait::SelectedEntity;
use tracing::debug;

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

impl<'s> SqlExecutor for SqliteTransaction<'s> {
    type DB = Sqlite;

    fn get_affected_rows(&mut self, query_result: &SqliteQueryResult) -> Result<u64> {
        Ok(query_result.rows_affected())
    }

    async fn fetch_optional_plain<'a, SE>(
        &'a mut self,
        stmt: &'a str,
        selection: &'a SE::Selection,
    ) -> Result<Option<SE>>
    where
        SE: SelectedEntity<Self::DB> + Send + Unpin,
    {
        let query: Query<'a, Self::DB, SqliteArguments<'a>> = query_with(stmt, Default::default());
        let result_opt: Option<<Self::DB as Database>::Row> =
            query.fetch_optional(&mut *self.transaction).await?;
        if let Some(result) = result_opt {
            Ok(Some(SE::from_row(selection, result)?))
        } else {
            Ok(None)
        }
    }

    async fn fetch_optional<'a, SE>(
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

    async fn execute_plain<'a>(&'a mut self, stmt: &'a str) -> Result<u64> {
        let query: Query<'a, Self::DB, SqliteArguments<'a>> = query_with(stmt, Default::default());
        let result: <Self::DB as Database>::QueryResult =
            query.execute(&mut *self.transaction).await?;
        self.get_affected_rows(&result)
    }

    async fn execute<'a, A>(&'a mut self, stmt: &'a str, args: SqliteArguments<'a>) -> Result<u64> {
        let query: Query<'a, Self::DB, SqliteArguments<'a>> = query_with(stmt, args);
        let result: SqliteQueryResult = query.execute(&mut *self.transaction).await?;
        self.get_affected_rows(&result)
    }

    async fn fetch_execute_plain<'a, SE>(&'a mut self, stmt: &'a str) -> Result<SE>
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

    async fn fetch_execute<'a, SE>(
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
}

impl<'a> SqliteCommander for SqliteTransaction<'a> {
    type G = DefaultSqlGenerator;

    fn get_generator(&mut self) -> &Self::G {
        &self.sql_generator
    }
}
