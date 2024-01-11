use crate::error::LunaOrmError;
use crate::sql_executor::SqlExecutor;
use crate::sql_generator::SqlGenerator;
use crate::LunaOrmResult;

use luna_orm_macro::timed;
use luna_orm_trait::*;
use std::fmt::Debug;
use tracing::{debug, instrument};

pub trait CommandExecutor: SqlExecutor + Debug {
    type G: SqlGenerator + Sync + Debug;

    fn get_generator(&self) -> &Self::G;

    async fn select<SE>(
        &mut self,
        primary: &dyn Primary,
        selection: &dyn Selection,
    ) -> LunaOrmResult<Option<SE>>
    where
        SE: SelectedEntity + Send + Unpin,
    {
        debug!(target: "luna_orm", command = "select", primary = ?primary, selection = ?selection);
        let sql = self.get_generator().get_select_sql(selection, primary);
        debug!(target: "luna_orm", command = "select", sql = sql);
        let args = primary.any_arguments();
        let result: Option<SE> = self.fetch_optional(&sql, args).await?;
        debug!(target: "luna_orm", command = "select", result = ?result);
        return Ok(result);
    }

    async fn create<'a>(&mut self, entity: &'a mut dyn Entity) -> LunaOrmResult<&'a dyn Entity> {
        debug!(target: "luna_orm", command = "insert",  entity = ?entity);
        let sql = self.get_generator().get_insert_sql(entity);
        debug!(target: "luna_orm", command = "insert", sql = sql);
        let args = entity.any_arguments_of_insert();
        self.execute(&sql, args).await?;
        debug!(target: "luna_orm", command = "insert", result = ?entity);
        return Ok(entity);
    }

    #[timed]
    async fn insert(&mut self, entity: &dyn Entity) -> LunaOrmResult<bool> {
        debug!(target: "luna_orm", command = "insert",  entity = ?entity);
        let sql = self.get_generator().get_insert_sql(entity);
        debug!(target: "luna_orm", command = "insert", sql = sql);
        let args = entity.any_arguments_of_insert();
        let result = self.execute(&sql, args).await?;
        debug!(target: "luna_orm", command = "insert", result = ?result);
        return Ok(result.rows_affected() > 0);
    }

    async fn upsert(&mut self, entity: &dyn Entity) -> LunaOrmResult<bool> {
        debug!(target: "luna_orm", command = "upsert", entity = ?entity);
        let sql = self.get_generator().get_upsert_sql(entity);
        debug!(target: "luna_orm", command = "upsert", sql = sql);
        let args = entity.any_arguments_of_upsert();
        let result = self.execute(&sql, args).await?;
        debug!(target: "luna_orm", command = "upsert", result = ?result);
        return Ok(result.rows_affected() > 0);
    }

    async fn update(
        &mut self,
        mutation: &dyn Mutation,
        primary: &dyn Primary,
    ) -> LunaOrmResult<bool> {
        debug!(target: "luna_orm", command = "update", mutation = ?mutation, primary = ?primary);
        let sql = self.get_generator().get_update_sql(mutation, primary);
        debug!(target: "luna_orm", command = "update", sql = sql);
        let mut args = mutation.any_arguments();
        let where_args = primary.any_arguments();
        args = luna_merge_args(args, where_args);
        let result = self.execute(&sql, args).await?;
        debug!(target: "luna_orm", command = "update", result = ?result);
        return Ok(result.rows_affected() > 0);
    }

    async fn delete(&mut self, primary: &dyn Primary) -> LunaOrmResult<bool> {
        debug!(target: "luna_orm", command = "delete", primary = ?primary);
        let sql = self.get_generator().get_delete_sql(primary);
        debug!(target: "luna_orm", command = "delete", sql = sql);
        let args = primary.any_arguments();
        let result = self.execute(&sql, args).await?;
        debug!(target: "luna_orm", command = "delete", result = ?result);
        return Ok(result.rows_affected() > 0);
    }

    async fn search_all<SE>(&mut self, selection: &dyn Selection) -> LunaOrmResult<Vec<SE>>
    where
        SE: SelectedEntity + Send + Unpin,
    {
        debug!(target: "luna_orm", command = "search_all", selection = ?selection);
        let sql = self.get_generator().get_search_all_sql(selection);
        debug!(target: "luna_orm", command = "search_all", sql = sql);
        let result: Vec<SE> = self.fetch_all_plain(&sql).await?;
        debug!(target: "luna_orm", command = "search_all", result = ?result);
        return Ok(result);
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
        debug!(target: "luna_orm", command = "search", location = ?location, order_by = ?order_by, selection = ?selection);
        let sql = self
            .get_generator()
            .get_search_sql(selection, location, order_by);
        debug!(target: "luna_orm", command = "search", sql = sql);
        if order_by.is_some() {
            let order_by_fields = order_by.unwrap().get_order_by_fields();
            let valid_order_by = location.check_valid_order_by(order_by_fields);
            if !valid_order_by {
                return Err(LunaOrmError::OrderByFieldsError);
            }
        }
        let args = location.any_arguments();
        let result: Vec<SE> = self.fetch_all(&sql, args).await?;
        debug!(target: "luna_orm", command = "search", result = ?result);
        return Ok(result);
    }

    async fn count(&mut self, location: &dyn Location) -> LunaOrmResult<usize> {
        debug!(target: "luna_orm", command = "count", location = ?location);
        let args = location.any_arguments();
        let count_sql = self.get_generator().get_search_count_sql(location);
        debug!(target: "luna_orm", command = "count", sql = count_sql);
        let record_count: Option<RecordCount> = self.fetch_optional(&count_sql, args).await?;
        debug!(target: "luna_orm", command = "count", result = ?record_count);
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
        debug!(target: "luna_orm", command = "search_paged", location = ?location, order_by = ?order_by, selection = ?selection, page = ?page);
        if order_by.is_some() {
            let order_by_fields = order_by.unwrap().get_order_by_fields();
            let valid_order_by = location.check_valid_order_by(order_by_fields);
            if !valid_order_by {
                return Err(LunaOrmError::OrderByFieldsError);
            }
        }
        let args = location.any_arguments();
        let count_sql = self.get_generator().get_search_count_sql(location);
        debug!(target: "luna_orm", command = "search_paged", count_sql = count_sql);
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
        debug!(target: "luna_orm", command = "search_paged", sql = sql);
        let args = location.any_arguments();
        let entity_list: Vec<SE> = self.fetch_all(&sql, args).await?;
        let page_info = PageInfo {
            page_size: page.page_size,
            page_num: page.page_num,
            page_total: (record_count / page.page_size as i64) as usize,
            total: record_count as usize,
        };

        let result = PagedList {
            data: entity_list,
            page: page_info,
        };
        debug!(target: "luna_orm", command = "search_paged", result = ?result);
        Ok(result)
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
        debug!(target: "luna_orm", command = "purify", location = ?location);
        let sql = self.get_generator().get_purify_sql(location);
        debug!(target: "luna_orm", command = "purify", sql = sql);
        let args = location.any_arguments();
        let result = self.execute(&sql, args).await?;
        debug!(target: "luna_orm", command = "purify", result = ?result);
        return Ok(result.rows_affected() as usize);
    }

    async fn change(
        &mut self,
        mutation: &dyn Mutation,
        location: &dyn Location,
    ) -> LunaOrmResult<usize> {
        debug!(target: "luna_orm", command = "change", mutation = ?mutation, location = ?location);
        let sql = self.get_generator().get_change_sql(mutation, location);
        debug!(target: "luna_orm", command = "change", sql = sql);
        let mut args = mutation.any_arguments();
        let where_args = location.any_arguments();
        args = luna_merge_args(args, where_args);
        let result = self.execute(&sql, args).await?;
        debug!(target: "luna_orm", command = "change", result = ?result);
        return Ok(result.rows_affected() as usize);
    }

    async fn execute_by_template(&mut self, template: &dyn TemplateRecord) -> LunaOrmResult<usize> {
        debug!(target: "luna_orm", command = "execute_by_template", template = ?template);
        let sql = template.get_sql(None);
        let sql = self.get_generator().post_process(sql);
        debug!(target: "luna_orm", command = "execute_by_template", sql = sql);
        let args = template.any_arguments();
        let result = self.execute(&sql, args).await?;
        debug!(target: "luna_orm", command = "execute_by_template", result = ?result);
        return Ok(result.rows_affected() as usize);
    }

    async fn select_by_template<SE>(
        &mut self,
        template: &dyn TemplateRecord,
    ) -> LunaOrmResult<Option<SE>>
    where
        SE: SelectedEntity + Send + Unpin,
    {
        debug!(target: "luna_orm", command = "select_by_template", template = ?template);
        let sql = template.get_sql(None);
        let sql = self.get_generator().post_process(sql);
        debug!(target: "luna_orm", command = "select_by_template", sql = sql);
        let args = template.any_arguments();
        let result: Option<SE> = self.fetch_optional(&sql, args).await?;
        debug!(target: "luna_orm", command = "select_by_template", result = ?result);
        return Ok(result);
    }
    async fn search_by_template<SE>(
        &mut self,
        template: &dyn TemplateRecord,
    ) -> LunaOrmResult<Vec<SE>>
    where
        SE: SelectedEntity + Send + Unpin,
    {
        debug!(target: "luna_orm", command = "search_by_template", template = ?template);
        let sql = template.get_sql(None);
        let sql = self.get_generator().post_process(sql);
        debug!(target: "luna_orm", command = "search_by_template", sql = sql);
        let args = template.any_arguments();
        let result: Vec<SE> = self.fetch_all(&sql, args).await?;
        debug!(target: "luna_orm", command = "search_by_template", result = ?result);
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
        debug!(target: "luna_orm", command = "search_paged_by_template", template = ?template, page = ?page);
        let count_sql = template.get_count_sql();
        let record_count: Option<RecordCount>;
        let args = template.any_arguments();
        match count_sql {
            CountSql::Empty => {
                return Err(LunaOrmError::PagedTemplateHasNoCountSql);
            }
            CountSql::PlainSql(sql) => {
                let sql = self.get_generator().post_process(sql);
                debug!(target: "luna_orm", command = "search_paged_by_template", count_sql = sql);
                record_count = self.fetch_optional_plain(&sql).await?;
            }
            CountSql::VariabledSql(sql) => {
                let sql = self.get_generator().post_process(sql);
                debug!(target: "luna_orm", command = "search_paged_by_template", count_sql = sql);
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
        debug!(target: "luna_orm", command = "search_paged_by_template", sql = sql);
        let args = template.any_arguments();
        let entity_list: Vec<SE> = self.fetch_all(&sql, args).await?;

        let page_info = PageInfo {
            page_size: page.page_size,
            page_num: page.page_num,
            page_total: (record_count / page.page_size as i64) as usize,
            total: record_count as usize,
        };
        let result = PagedList {
            data: entity_list,
            page: page_info,
        };
        debug!(target: "luna_orm", command = "search_paged_by_template", result = ?result);
        Ok(result)
    }
}
