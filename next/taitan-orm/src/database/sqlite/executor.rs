use crate::database::sqlite::database::SqliteDatabase;
use crate::sql_generic_executor::SqlGenericExecutor;
use crate::{execute_fn, execute_plain_fn, fetch_exists_fn, fetch_exists_plain_fn, Result};
use crate::SqlExecutor;
use sqlx::pool::PoolConnection;
use sqlx::sqlite::SqliteArguments;
use sqlx::{Database, IntoArguments, Sqlite, SqliteConnection};
use std::marker::PhantomData;
use taitan_orm_trait::SelectedEntity;

impl SqlGenericExecutor for SqliteDatabase {
    type DB = Sqlite;

    fn get_affected_rows(query_result: &<Self::DB as Database>::QueryResult) -> u64 {
        query_result.rows_affected()
    }
}

impl SqlExecutor for SqliteDatabase {
    type Connection = SqliteConnection;

    #[inline(always)]
    async fn get_connection(&mut self) -> Result<PoolConnection<Self::DB>> {
        Ok(self.get_pool()?.acquire().await?)
    }

    execute_fn!();

    execute_plain_fn!();


    // async fn fetch_exists<'a>(
    //     &'a mut self,
    //     stmt: &'a str,
    //     args: <Self::DB as Database>::Arguments<'a>,
    // ) -> Result<bool> {
    //     let mut ex = self.get_pool()?.acquire().await?;
    //     Self::generic_exists(&mut *ex, stmt, args).await
    // }
    fetch_exists_fn!();

    async fn fetch_exists_plain<'a, A>(&'a mut self, stmt: &'a str) -> Result<bool>
    where
        A: IntoArguments<'a, Self::DB> + 'a + Default,
    {
        let mut ex = self.get_pool()?.acquire().await?;
        let args: PhantomData<SqliteArguments> = PhantomData::default();
        Self::generic_exists_plain(&mut *ex, stmt, args).await
    }
    // fetch_exists_plain_fn!();

    async fn fetch_option<'a, SE>(
        &'a mut self,
        stmt: &'a str,
        selection: &'a SE::Selection,
        args: SqliteArguments<'a>,
    ) -> Result<Option<SE>>
    where
        SE: SelectedEntity<Self::DB> + Send + Unpin,
    {
        let mut ex = self.get_pool()?.acquire().await?;
        Self::generic_fetch_option(&mut *ex, stmt, selection, args).await
    }

    async fn fetch_option_plain<'a, SE>(
        &'a mut self,
        stmt: &'a str,
        selection: &'a SE::Selection,
    ) -> Result<Option<SE>>
    where
        SE: SelectedEntity<Self::DB> + Send + Unpin,
    {
        let mut ex = self.get_pool()?.acquire().await?;
        let args: PhantomData<SqliteArguments> = PhantomData::default();
        Self::generic_fetch_option_plain(&mut *ex, stmt, selection, args).await
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
        let mut ex = self.get_pool()?.acquire().await?;
        Self::generic_fetch_all(&mut *ex, stmt, selection, args).await
    }

    async fn fetch_all_plain<'a, SE>(
        &'a mut self,
        stmt: &'a str,
        selection: &'a SE::Selection,
    ) -> Result<Vec<SE>>
    where
        SE: SelectedEntity<Self::DB> + Send + Unpin,
    {
        let mut ex = self.get_pool()?.acquire().await?;
        let args: PhantomData<SqliteArguments> = PhantomData::default();
        Self::generic_fetch_all_plain(&mut *ex, stmt, selection, args).await
    }

    async fn fetch_one_full<'a, SE>(
        &'a mut self,
        stmt: &'a str,
        args: SqliteArguments<'a>,
    ) -> Result<SE>
    where
        SE: SelectedEntity<Self::DB> + Send + Unpin,
    {
        let mut ex = self.get_pool()?.acquire().await?;
        Self::generic_fetch_one_full(&mut *ex, stmt, args).await
    }

    async fn fetch_one_full_plain<'a, SE>(&'a mut self, stmt: &'a str) -> Result<SE>
    where
        SE: SelectedEntity<Self::DB> + Send + Unpin,
    {
        let mut ex = self.get_pool()?.acquire().await?;
        let args: PhantomData<SqliteArguments> = PhantomData::default();
        Self::generic_fetch_one_full_plain(&mut *ex, stmt, args).await
    }

    async fn fetch_option_full<'a, SE>(
        &'a mut self,
        stmt: &'a str,
        args: <Self::DB as Database>::Arguments<'a>,
    ) -> Result<Option<SE>>
    where
        SE: SelectedEntity<Self::DB> + Send + Unpin,
    {
        let mut ex = self.get_pool()?.acquire().await?;
        Self::generic_fetch_option_full(&mut *ex, stmt, args).await
    }

    async fn fetch_option_full_plain<'a, SE>(&'a mut self, stmt: &'a str) -> Result<Option<SE>>
    where
        SE: SelectedEntity<Self::DB> + Send + Unpin,
    {
        let mut ex = self.get_pool()?.acquire().await?;
        let args: PhantomData<SqliteArguments> = PhantomData::default();
        Self::generic_fetch_option_full_plain(&mut *ex, stmt, args).await
    }

    async fn fetch_all_full<'a, SE>(
        &'a mut self,
        stmt: &'a str,
        args: <Self::DB as Database>::Arguments<'a>,
    ) -> Result<Vec<SE>>
    where
        SE: SelectedEntity<Self::DB> + Send + Unpin,
    {
        let mut ex = self.get_pool()?.acquire().await?;
        Self::generic_fetch_all_full(&mut *ex, stmt, args).await
    }

    async fn fetch_all_full_plain<'a, SE>(&'a mut self, stmt: &'a str) -> Result<Vec<SE>>
    where
        SE: SelectedEntity<Self::DB> + Send + Unpin,
    {
        let mut ex = self.get_pool()?.acquire().await?;
        let args: PhantomData<SqliteArguments> = PhantomData::default();
        Self::generic_fetch_all_full_plain(&mut *ex, stmt, args).await
    }
}
