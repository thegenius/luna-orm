use crate::error::LunaOrmError;
use crate::sql_executor::SqlExecutor;
use crate::sql_generator::SqlGenerator;
use crate::LunaOrmResult;

use async_trait::async_trait;
use luna_orm_trait::*;

#[async_trait]
pub trait CommandExecutor: SqlExecutor {
    type G: SqlGenerator + Sync;

    fn get_generator(&self) -> &Self::G;

    async fn select<SE>(
        &mut self,
        primary: &dyn Primary,
        selection: &dyn Selection,
    ) -> LunaOrmResult<Option<SE>>
    where
        SE: SelectedEntity + Send + Unpin,
    {
        let sql = self.get_generator().get_select_sql(selection, primary);
        let args = primary.any_arguments();
        let result: Option<SE> = self.fetch_optional(&sql, args).await?;
        return Ok(result);
    }

    async fn create<'a>(&mut self, entity: &'a dyn Entity) -> LunaOrmResult<&'a dyn Entity> {
        let sql = self.get_generator().get_insert_sql(entity);
        let args = entity.any_arguments_of_insert();
        self.execute(&sql, args).await?;
        return Ok(entity);
    }

    async fn insert(&mut self, entity: &dyn Entity) -> LunaOrmResult<bool> {
        let sql = self.get_generator().get_insert_sql(entity);
        let args = entity.any_arguments_of_insert();
        let result = self.execute(&sql, args).await?;
        return Ok(result.rows_affected() > 0);
    }

    async fn upsert(&mut self, entity: &dyn Entity) -> LunaOrmResult<bool> {
        let sql = self.get_generator().get_upsert_sql(entity);
        let args = entity.any_arguments_of_upsert();
        let result = self.execute(&sql, args).await?;
        return Ok(result.rows_affected() > 0);
    }

    async fn update(
        &mut self,
        mutation: &dyn Mutation,
        primary: &dyn Primary,
    ) -> LunaOrmResult<bool> {
        let sql = self.get_generator().get_update_sql(mutation, primary);
        let mut args = mutation.any_arguments();
        let where_args = primary.any_arguments();
        args = luna_merge_args(args, where_args);
        let result = self.execute(&sql, args).await?;
        return Ok(result.rows_affected() > 0);
    }

    async fn delete(&mut self, primary: &dyn Primary) -> LunaOrmResult<bool> {
        let sql = self.get_generator().get_delete_sql(primary);
        let args = primary.any_arguments();
        let result = self.execute(&sql, args).await?;
        return Ok(result.rows_affected() > 0);
    }

    async fn search<SE>(
        &mut self,
        location: &dyn Location,
        order_by: Option<&dyn OrderBy>,
        selection: &dyn Selection,
    ) -> LunaOrmResult<Vec<SE>>
    where
        SE: SelectedEntity + Send + Unpin,
    {
        let sql = self
            .get_generator()
            .get_search_sql(selection, location, order_by);
        if order_by.is_some() {
            let order_by_fields = order_by.unwrap().get_order_by_fields();
            let valid_order_by = location.check_valid_order_by(order_by_fields);
            if !valid_order_by {
                return Err(LunaOrmError::OrderByFieldsError);
            }
        }
        let args = location.any_arguments();
        let result: Vec<SE> = self.fetch_all(&sql, args).await?;
        return Ok(result);
    }

    async fn count(&mut self, location: &dyn Location) -> LunaOrmResult<usize> {
        let args = location.any_arguments();
        let count_sql = self.get_generator().get_search_count_sql(location);
        let record_count: Option<RecordCount> = self.fetch_optional(&count_sql, args).await?;
        if record_count.is_none() {
            return Ok(0);
        } else {
            return Ok(record_count.unwrap().count as usize);
        }
    }

    async fn search_paged<SE>(
        &mut self,
        location: &dyn Location,
        order_by: Option<&dyn OrderBy>,
        selection: &dyn Selection,
        page: &Pagination,
    ) -> LunaOrmResult<PagedList<SE>>
    where
        SE: SelectedEntity + Send + Unpin,
    {
        if order_by.is_some() {
            let order_by_fields = order_by.unwrap().get_order_by_fields();
            let valid_order_by = location.check_valid_order_by(order_by_fields);
            if !valid_order_by {
                return Err(LunaOrmError::OrderByFieldsError);
            }
        }
        let args = location.any_arguments();
        let count_sql = self.get_generator().get_search_count_sql(location);
        let record_count: Option<RecordCount> = self.fetch_optional(&count_sql, args).await?;
        if record_count.is_none() {
            return Ok(PagedList::empty(page.page_size, page.page_num));
        }
        let record_count: RecordCount = record_count.unwrap();
        if record_count.count <= 0 {
            return Ok(PagedList::empty(page.page_size, page.page_num));
        }
        let record_count: i64 = record_count.count;

        let sql = self
            .get_generator()
            .get_paged_search_sql(selection, location, order_by, page);
        let args = location.any_arguments();
        let entity_list: Vec<SE> = self.fetch_all(&sql, args).await?;
        let page_info = PageInfo {
            page_size: page.page_size,
            page_num: page.page_num,
            page_total: (record_count / page.page_size as i64) as usize,
            total: record_count as usize,
        };
        return Ok(PagedList {
            data: entity_list,
            page: page_info,
        });
    }

    async fn search_joined<SE>(
        &mut self,
        joined_conds: JoinedConditions,
        locations: Vec<&dyn Location>,
        order_by: Option<&dyn OrderBy>,
        selections: Vec<&dyn Selection>,
        page: &Pagination,
    ) -> String
//-> LunaOrmResult<PagedList<SE>>
    //where
    //    SE: SelectedEntity + Send + Unpin,
    {
        self.get_generator().get_page_joined_search_sql(
            &joined_conds,
            &locations,
            order_by,
            &selections,
            page,
        )
    }

    async fn purify(&mut self, location: &dyn Location) -> LunaOrmResult<usize> {
        let sql = self.get_generator().get_purify_sql(location);
        let args = location.any_arguments();
        let result = self.execute(&sql, args).await?;
        return Ok(result.rows_affected() as usize);
    }

    async fn change(
        &mut self,
        mutation: &dyn Mutation,
        location: &dyn Location,
    ) -> LunaOrmResult<usize> {
        let sql = self.get_generator().get_change_sql(mutation, location);
        let mut args = mutation.any_arguments();
        let where_args = location.any_arguments();
        args = luna_merge_args(args, where_args);
        let result = self.execute(&sql, args).await?;
        return Ok(result.rows_affected() as usize);
    }

    async fn execute_by_template(&mut self, template: &dyn TemplateRecord) -> LunaOrmResult<usize> {
        let sql = template.get_sql(None);
        let sql = self.get_generator().post_process(sql);
        let args = template.any_arguments();
        let result = self.execute(&sql, args).await?;
        return Ok(result.rows_affected() as usize);
    }

    async fn select_by_template<SE>(
        &mut self,
        template: &dyn TemplateRecord,
    ) -> LunaOrmResult<Option<SE>>
    where
        SE: SelectedEntity + Send + Unpin,
    {
        let sql = template.get_sql(None);
        let sql = self.get_generator().post_process(sql);
        let args = template.any_arguments();
        let result: Option<SE> = self.fetch_optional(&sql, args).await?;
        return Ok(result);
    }
    async fn search_by_template<SE>(
        &mut self,
        template: &dyn TemplateRecord,
    ) -> LunaOrmResult<Vec<SE>>
    where
        SE: SelectedEntity + Send + Unpin,
    {
        let sql = template.get_sql(None);
        let sql = self.get_generator().post_process(sql);
        let args = template.any_arguments();
        let result: Vec<SE> = self.fetch_all(&sql, args).await?;
        return Ok(result);
    }

    async fn search_paged_by_template<SE>(
        &mut self,
        template: &dyn TemplateRecord,
        page: &Pagination,
    ) -> LunaOrmResult<PagedList<SE>>
    where
        SE: SelectedEntity + Send + Unpin,
    {
        let count_sql = template.get_count_sql();
        let mut record_count: Option<RecordCount>;
        let args = template.any_arguments();
        match count_sql {
            CountSql::Empty => {
                return Err(LunaOrmError::PagedTemplateHasNoCountSql);
            }
            CountSql::PlainSql(sql) => {
                let sql = self.get_generator().post_process(sql);
                record_count = self.fetch_optional_plain(&sql).await?;
            }
            CountSql::VariabledSql(sql) => {
                let sql = self.get_generator().post_process(sql);
                record_count = self.fetch_optional(&sql, args).await?;
            }
        }
        if record_count.is_none() {
            return Ok(PagedList::empty(page.page_size, page.page_num));
        }
        let record_count: RecordCount = record_count.unwrap();
        if record_count.count <= 0 {
            return Ok(PagedList::empty(page.page_size, page.page_num));
        }
        let record_count: i64 = record_count.count;

        let sql = template.get_sql(Some(page));
        let sql = self.get_generator().post_process(sql);
        let args = template.any_arguments();
        let entity_list: Vec<SE> = self.fetch_all(&sql, args).await?;

        let page_info = PageInfo {
            page_size: page.page_size,
            page_num: page.page_num,
            page_total: (record_count / page.page_size as i64) as usize,
            total: record_count as usize,
        };
        return Ok(PagedList {
            data: entity_list,
            page: page_info,
        });
    }
}
