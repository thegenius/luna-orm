use crate::database::sqlite::SqliteCommander;
use crate::Result;
use crate::SqlExecutor;
use sqlx::sqlite::SqliteArguments;
use sqlx::{Arguments, Database, Executor, Sqlite, SqlitePool};
use std::marker::PhantomData;
use taitan_orm_trait::SelectedEntity;

impl SqlExecutor for SqliteCommander {
    type DB = Sqlite;
    fn get_pool(&self) -> Result<&SqlitePool> {
        Ok(&self.sqlite_pool)
    }
    fn get_affected_rows(&self, query_result: &<Self::DB as Database>::QueryResult) -> Result<u64> {
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
        let mut ex = self.get_pool()?.acquire().await?;
        let args: PhantomData<SqliteArguments> = PhantomData::default();
        self.generic_fetch_optional_plain(&mut *ex, stmt, selection, args)
            .await
    }

    async fn fetch_execute<'a, SE>(&'a self, stmt: &'a str, args: SqliteArguments<'a>) -> Result<SE>
    where
        SE: SelectedEntity<Self::DB> + Send + Unpin,
    {
        let mut ex = self.get_pool()?.acquire().await?;
        self.generic_fetch_execute(&mut *ex, stmt, args).await
    }

    async fn fetch_execute_plain<'a, SE>(&'a self, stmt: &'a str) -> Result<SE>
    where
        SE: SelectedEntity<Self::DB> + Send + Unpin,
    {
        let mut ex = self.get_pool()?.acquire().await?;
        let args: PhantomData<SqliteArguments> = PhantomData::default();
        self.generic_fetch_execute_plain(&mut *ex, stmt, args).await
    }

    async fn fetch_optional<'a, SE>(
        &'a self,
        stmt: &'a str,
        selection: &'a SE::Selection,
        args: SqliteArguments<'a>,
    ) -> Result<Option<SE>>
    where
        SE: SelectedEntity<Self::DB> + Send + Unpin,
    {
        let mut ex = self.get_pool()?.acquire().await?;
        self.generic_fetch_optional(&mut *ex, stmt, selection, args)
            .await
    }

    async fn fetch_all_plain<'a, SE>(
        &'a self,
        stmt: &'a str,
        selection: &'a SE::Selection,
    ) -> Result<Vec<SE>>
    where
        SE: SelectedEntity<Self::DB> + Send + Unpin,
    {
        let mut ex = self.get_pool()?.acquire().await?;
        let args: PhantomData<SqliteArguments> = PhantomData::default();
        self.generic_fetch_all_plain(&mut *ex, stmt, selection, args)
            .await
    }

    async fn fetch_all<'a, SE>(
        &'a self,
        stmt: &'a str,
        selection: &'a SE::Selection,
        args: SqliteArguments<'a>,
    ) -> Result<Vec<SE>>
    where
        SE: SelectedEntity<Self::DB> + Send + Unpin,
    {
        let mut ex = self.get_pool()?.acquire().await?;
        self.generic_fetch_all(&mut *ex, stmt, selection, args)
            .await
    }

    async fn execute_plain<'a>(&'a self, stmt: &'a str) -> Result<u64> {
        let mut ex = self.get_pool()?.acquire().await?;
        let args: PhantomData<SqliteArguments> = PhantomData::default();
        self.generic_execute_plain(&mut *ex, stmt, args).await
    }

    async fn execute<'a, A>(&'a self, stmt: &'a str, args: SqliteArguments<'a>) -> Result<u64> {
        let mut ex = self.get_pool()?.acquire().await?;
        self.generic_execute(&mut *ex, stmt, args).await
    }
}
