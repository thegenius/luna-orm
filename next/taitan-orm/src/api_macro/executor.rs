#[macro_export]
macro_rules! execute_fn {
    () => {
        async fn execute<'a, A>(&'a mut self, stmt: &'a str, args: A) -> Result<u64>
        where
            A: IntoArguments<'a, Self::DB> + 'a
        {
            let mut ex = self.get_connection().await?;
            Self::generic_execute(&mut *ex, stmt, args).await
        }
    };
}


#[macro_export]
macro_rules! execute_plain_fn {
    () => {
        async fn execute_plain<'a>(&'a mut self, stmt: &'a str) -> crate::Result<u64> {
            let args: PhantomData<SqliteArguments> = PhantomData::default();
            let mut ex = self.get_pool()?.acquire().await?;
            Self::generic_execute_plain(&mut *ex, stmt, args).await
        }
    };
}

#[macro_export]
macro_rules! fetch_exists_fn {
    () => {
        async fn fetch_exists<'a>(
            &'a mut self,
            stmt: &'a str,
            args: <Self::DB as Database>::Arguments<'a>,
        ) -> Result<bool> {
            let mut ex = self.get_pool()?.acquire().await?;
            Self::generic_exists(&mut *ex, stmt, args).await
        }
    };
}

#[macro_export]
macro_rules! fetch_exists_plain_fn {
    () => {
        async fn fetch_exists_plain<'a, A>(&'a mut self, stmt: &'a str) -> Result<bool>
        where
            A: IntoArguments<'a, crate::result::Result::DB> + 'a + Default,
        {
            let mut ex = self.get_pool()?.acquire().await?;
            let args: PhantomData<SqliteArguments> = PhantomData::default();
            Self::generic_exists_plain(&mut *ex, stmt, args).await
        }
    };
}

#[macro_export]
macro_rules! fetch_option_fn {
    () => {
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
    };
}

#[macro_export]
macro_rules! fetch_option_plain_fn {
    () => {
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
    };
}

#[macro_export]
macro_rules! fetch_all_fn {
    () => {
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
    };
}

#[macro_export]
macro_rules! fetch_all_plain_fn {
    () => {
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
    };
}

#[macro_export]
macro_rules! fetch_one_full_fn {
    () => {
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
    };
}

#[macro_export]
macro_rules! fetch_one_full_plain_fn {
    () => {
        async fn fetch_one_full_plain<'a, SE>(&'a mut self, stmt: &'a str) -> Result<SE>
        where
            SE: SelectedEntity<Self::DB> + Send + Unpin,
        {
            let mut ex = self.get_pool()?.acquire().await?;
            let args: PhantomData<SqliteArguments> = PhantomData::default();
            Self::generic_fetch_one_full_plain(&mut *ex, stmt, args).await
        }

    };
}

#[macro_export]
macro_rules! fetch_option_full_fn {
    () => {
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
    };
}

#[macro_export]
macro_rules! fetch_option_full_plain_fn {
    () => {
        async fn fetch_option_full_plain<'a, SE>(&'a mut self, stmt: &'a str) -> Result<Option<SE>>
        where
            SE: SelectedEntity<Self::DB> + Send + Unpin,
        {
            let mut ex = self.get_pool()?.acquire().await?;
            let args: PhantomData<SqliteArguments> = PhantomData::default();
            Self::generic_fetch_option_full_plain(&mut *ex, stmt, args).await
        }
    };
}

#[macro_export]
macro_rules! fetch_all_full_fn {
    () => {
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
    };
}

#[macro_export]
macro_rules! fetch_all_full_plain_fn {
    () => {
        async fn fetch_all_full_plain<'a, SE>(&'a mut self, stmt: &'a str) -> Result<Vec<SE>>
        where
            SE: SelectedEntity<Self::DB> + Send + Unpin,
        {
            let mut ex = self.get_pool()?.acquire().await?;
            let args: PhantomData<SqliteArguments> = PhantomData::default();
            Self::generic_fetch_all_full_plain(&mut *ex, stmt, args).await
        }
    };
}



















