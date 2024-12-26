#[macro_export]
macro_rules! exists_fn {
    ($args_type:ty, $gen_args_fn:path) => {
        async fn exists<M: Mutation>(&mut self, unique: &dyn Unique<Mutation = M>) -> Result<bool> {
            debug!(target: "taitan_orm", command = "exists", unique = ?unique);
            let sql = self.get_generator().get_exists_sql(unique);
            debug!(target: "taitan_orm", command = "exists", sql = sql);
            let args = ($gen_args_fn)(unique)?;
            let result: bool = self.fetch_exists(&sql, args).await?;
            debug!(target: "taitan_orm", command = "exists", result = ?result);
            Ok(result)
        }
    };
}

#[macro_export]
macro_rules! count_fn {
    ($args_type:ty, $gen_args_fn:path) => {
        async fn count(&mut self, location: &dyn Location) -> Result<u64> {
            debug!(target: "taitan_orm", command = "count", location = ?location);
            let args = ($gen_args_fn)(location)?;
            let count_sql = self.get_generator().get_count_sql(location);
            debug!(target: "taitan_orm", command = "count", sql = count_sql);
            let record_count: CountResult = self.fetch_one_full(&count_sql, args).await?;
            debug!(target: "taitan_orm", command = "count", result = ?record_count);
            Ok(record_count.count)
        }
    };
}

#[macro_export]
macro_rules! count_all_fn {
    ($args_type:ty) => {
        async fn count_all(&mut self, table_name: &str) -> Result<u64> {
            debug!(target: "taitan_orm", command = "count", table_name = ?table_name);
            let count_sql = self.get_generator().get_count_table_sql(table_name);
            debug!(target: "taitan_orm", command = "count", sql = count_sql);
            let record_count: CountResult = self.fetch_one_full_plain(&count_sql).await?;
            debug!(target: "taitan_orm", command = "count", result = ?record_count);
            Ok(record_count.count)
        }
    };
}

#[macro_export]
macro_rules! select_fn {
    ($gen_args_fn:path) => {
        async fn select<SE, M>(
            &mut self,
            selection: &SE::Selection,
            unique: &dyn Unique<Mutation = M>,
        ) -> Result<Option<SE>>
        where
            M: Mutation,
            SE: SelectedEntity<Self::DB> + Send + Unpin,
        {
            debug!(target: "taitan_orm", command = "select", primary = ?unique, selection = ?selection);
            let sql = self.get_generator().get_select_sql(selection, unique);
            debug!(target: "taitan_orm", command = "select", sql = sql);
            let args = ($gen_args_fn)(unique)?;
            let result: Option<SE> = self.fetch_option(&sql, selection, args).await?;
            debug!(target: "taitan_orm", command = "select", result = ?result);
            Ok(result)
        }
    };
}

#[macro_export]
macro_rules! search_fn {
    ($gen_args_fn:path) => {
        async fn search<SE>(
            &mut self,
            selection: &SE::Selection,
            location: &dyn Location,
            order_by: &Option<&dyn OrderBy>,
            page: &Option<&Pagination>
        ) -> Result<Vec<SE>>
        where
            SE: SelectedEntity<Self::DB> + Send + Unpin,
        {
            debug!(target: "taitan_orm", command = "search", location = ?location, order_by = ?order_by, selection = ?selection);
            let sql = self
                .get_generator()
                .get_search_paged_sql(selection, &Some(location), order_by, page);
            debug!(target: "taitan_orm", command = "search", sql = sql);
            let args = ($gen_args_fn)(location)?;
            let result: Vec<SE> = self.fetch_all(&sql, selection, args).await?;
            debug!(target: "taitan_orm", command = "search", result = ?result);
            Ok(result)
        }
    };
}

#[macro_export]
macro_rules! search_paged_fn {
    ($gen_args_fn:path) => {
        async fn search_paged<SE>(
            &mut self,
            selection: &SE::Selection,
            location: &dyn Location,
            order_by: &dyn OrderBy,
            page: &Pagination,
        ) -> Result<PagedList<Self::DB, SE>>
        where
            SE: SelectedEntity<Self::DB> + Send + Unpin,
        {
            debug!(target: "taitan_orm", command = "search_paged", location = ?location, order_by = ?order_by, selection = ?selection, page = ?page);
            let record_count = self.count(location).await?;
            if record_count <= 0 {
                return Ok(PagedList::empty(page.page_size, page.page_num));
            }

            let sql =
                self.get_generator()
                    .get_search_paged_sql(selection, &Some(location), &Some(order_by), &Some(&page));
            debug!(target: "taitan_orm", command = "search_paged", sql = sql);
            let args = ($gen_args_fn)(location)?;
            let entity_list: Vec<SE> = self.fetch_all(&sql, selection, args).await?;
            let result = build_paged_list(entity_list, record_count, page);
            debug!(target: "taitan_orm", command = "search_paged", result = ?result);
            Ok(result)
        }
    };
}

#[macro_export]
macro_rules! devour_fn {
    ($gen_args_fn:path) => {
        async fn devour<SE>(
            &mut self,
            selection: &SE::Selection,
            order_by: &Option<&dyn OrderBy>,
            page: &Option<&Pagination>,
        ) -> Result<Vec<SE>>
        where
            SE: SelectedEntity<Self::DB> + Send + Unpin,
        {
            debug!(target: "taitan_orm", command = "devour", selection = ?selection);
            let sql = self.get_generator().get_search_paged_sql(selection, &None, order_by, page);
            debug!(target: "taitan_orm", command = "devour", sql = sql);
            match page {
                None => {
                    let result: Vec<SE> = self.fetch_all_plain(&sql, selection).await?;
                    debug!(target: "taitan_orm", command = "devour", result = ?result);
                    Ok(result)
                },
                Some(page) => {
                    let args = ($gen_args_fn)(page)?;
                    let result: Vec<SE> = self.fetch_all(&sql, selection, args).await?;
                    debug!(target: "taitan_orm", command = "devour", result = ?result);
                    Ok(result)
                }
            }
        }
    };
}

#[macro_export]
macro_rules! devour_paged_fn {
    ($gen_args_fn:path) => {
        async fn devour_paged<SE>(
            &mut self,
            selection: &SE::Selection,
            order_by: &dyn OrderBy,
            page: &Pagination,
        ) -> Result<PagedList<Self::DB, SE>>
        where
            SE: SelectedEntity<Self::DB> + Send + Unpin,
        {
            debug!(target: "taitan_orm", command = "devour_paged", order_by = ?order_by, selection = ?selection, page = ?page);
            let record_count = self.count_all(selection.get_table_name()).await?;
            if record_count <= 0 {
                return Ok(PagedList::empty(page.page_size, page.page_num));
            }

            debug!(target: "taitan_orm", command = "devour_paged", selection = ?selection);
            let sql = self.get_generator().get_search_paged_sql(selection, &None, &Some(order_by), &Some(page));
            debug!(target: "taitan_orm", command = "devour_paged", sql = sql);
            let args = ($gen_args_fn)(page)?;
            let entity_list: Vec<SE> = self.fetch_all(&sql, selection, args).await?;
            let result = build_paged_list(entity_list, record_count, page);
            debug!(target: "taitan_orm", command = "devour_paged", result = ?result);
            Ok(result)
        }
    };
}


