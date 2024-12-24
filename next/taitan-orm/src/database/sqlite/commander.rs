use crate::database::sqlite::SqliteLocalConfig;
use crate::sql_generator::DefaultSqlGenerator;
use crate::{CountResult, Result};
use crate::{SqlApi, SqlExecutor, SqlGenerator, TaitanOrmError};
use path_absolutize::Absolutize;
use std::fmt::Debug;
// use sqlx::error::BoxDynError;
use sqlx::sqlite::{SqliteArguments, SqliteConnectOptions, SqliteJournalMode, SqliteSynchronous};
use sqlx::{Database, Sqlite, SqlitePool};
use std::fs;
use std::marker::PhantomData;
use std::path::Path;
use taitan_orm_trait::page_info::PageInfo;
use taitan_orm_trait::paged_list::PagedList;
use taitan_orm_trait::pagination::Pagination;
use taitan_orm_trait::{
    Entity, Location, Mutation, OrderBy, SelectedEntity, Selection, TemplateRecord, Unique,
};
use tracing::debug;

fn build_paged_list<DB: Database, SE>(
    data: Vec<SE>,
    record_count: u64,
    page: &Pagination,
) -> PagedList<DB, SE>
where
    SE: SelectedEntity<DB> + Send + Unpin,
{
    let page_info = PageInfo {
        page_size: page.page_size,
        page_num: page.page_num,
        page_total: (record_count / page.page_size as u64),
        total: record_count,
    };

    PagedList {
        data,
        page: page_info,
        _phantom: PhantomData,
    }
}

pub trait SqliteCommander: SqlExecutor<DB = Sqlite> {
    type G: SqlGenerator + Sync + Debug;

    fn get_generator(&mut self) -> &Self::G;
    async fn insert(&mut self, entity: &dyn Entity) -> Result<bool> {
        debug!(target: "taitan_orm", command = "insert",  entity = ?entity);
        let sql = self.get_generator().get_insert_sql(entity);
        debug!(target: "taitan_orm", command = "insert", sql = sql);
        let args = entity.gen_insert_arguments_sqlite()?;
        let result = self.execute::<SqliteArguments>(&sql, args).await?;
        debug!(target: "taitan_orm", command = "insert", result = ?result);
        Ok(result > 0)
    }

    async fn upsert(&mut self, entity: &dyn Entity) -> Result<bool> {
        debug!(target: "taitan_orm", command = "upsert", entity = ?entity);
        let sql = self.get_generator().get_upsert_sql(entity);
        debug!(target: "taitan_orm", command = "upsert", sql = sql);
        let args = entity.gen_upsert_arguments_sqlite()?;
        let result = self.execute::<SqliteArguments>(&sql, args).await?;
        debug!(target: "taitan_orm", command = "upsert", result = ?result);
        Ok(result > 0)
    }

    async fn update<M: Mutation>(
        &mut self,
        mutation: &M,
        unique: &dyn Unique<Mutation = M>,
    ) -> Result<bool> {
        debug!(target: "taitan_orm", command = "update", mutation = ?mutation, primary = ?unique);
        let sql = self.get_generator().get_unique_update_sql(mutation, unique);
        debug!(target: "taitan_orm", command = "update", sql = sql);
        let args = unique.gen_update_arguments_sqlite(mutation)?;
        let result = self.execute::<SqliteArguments>(&sql, args).await?;
        debug!(target: "taitan_orm", command = "update", result = ?result);
        Ok(result > 0)
    }

    async fn change<M: Mutation>(&mut self, mutation: &M, location: &M::Location) -> Result<u64> {
        debug!(target: "taitan_orm", command = "change", mutation = ?mutation, location = ?location);
        let sql = self.get_generator().get_change_sql(mutation, location);
        debug!(target: "taitan_orm", command = "change", sql = sql);
        let args = mutation.gen_change_arguments_sqlite(location)?;
        let result = self.execute::<SqliteArguments>(&sql, args).await?;
        debug!(target: "taitan_orm", command = "change", result = ?result);
        Ok(result)
    }

    async fn delete<M: Mutation>(&mut self, unique: &dyn Unique<Mutation = M>) -> Result<bool> {
        debug!(target: "taitan_orm", command = "delete", primary = ?unique);
        let sql = self.get_generator().get_delete_sql(unique);
        debug!(target: "taitan_orm", command = "delete", sql = sql);
        let args = unique.gen_unique_arguments_sqlite()?;
        let result = self.execute::<SqliteArguments>(&sql, args).await?;
        debug!(target: "taitan_orm", command = "delete", result = ?result);
        Ok(result > 0)
    }

    async fn purify(&mut self, location: &dyn Location) -> Result<u64> {
        debug!(target: "taitan_orm", command = "purify", location = ?location);
        let sql = self.get_generator().get_purify_sql(location);
        debug!(target: "taitan_orm", command = "purify", sql = sql);
        let args = location.gen_location_arguments_sqlite()?;
        let result = self.execute::<SqliteArguments>(&sql, args).await?;
        debug!(target: "taitan_orm", command = "purify", result = ?result);
        Ok(result)
    }

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
        let args: SqliteArguments<'_> = unique.gen_unique_arguments_sqlite()?;
        let result: Option<SE> = self.fetch_optional(&sql, selection, args).await?;
        debug!(target: "taitan_orm", command = "select", result = ?result);
        Ok(result)
    }

    async fn search<SE>(
        &mut self,
        selection: &SE::Selection,
        location: &dyn Location,
        order_by: &dyn OrderBy,
    ) -> Result<Vec<SE>>
    where
        SE: SelectedEntity<Self::DB> + Send + Unpin,
    {
        debug!(target: "taitan_orm", command = "search", location = ?location, order_by = ?order_by, selection = ?selection);
        let sql = self
            .get_generator()
            .get_search_sql(selection, location, Some(order_by));
        debug!(target: "taitan_orm", command = "search", sql = sql);
        let args = location.gen_location_arguments_sqlite()?;
        let result: Vec<SE> = self.fetch_all(&sql, selection, args).await?;
        debug!(target: "taitan_orm", command = "search", result = ?result);
        Ok(result)
    }

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
                .get_search_paged_sql(selection, location, &Some(order_by), page);
        debug!(target: "taitan_orm", command = "search_paged", sql = sql);
        let args = location.gen_location_arguments_sqlite()?;
        let entity_list: Vec<SE> = self.fetch_all(&sql, selection, args).await?;
        let result = build_paged_list(entity_list, record_count, page);
        debug!(target: "taitan_orm", command = "search_paged", result = ?result);
        Ok(result)
    }

    async fn devour<SE>(
        &mut self,
        selection: &SE::Selection,
        order_by: &dyn OrderBy,
    ) -> Result<Vec<SE>>
    where
        SE: SelectedEntity<Self::DB> + Send + Unpin,
    {
        debug!(target: "taitan_orm", command = "devour", selection = ?selection);
        let sql = self.get_generator().get_devour_sql(selection, order_by);
        debug!(target: "taitan_orm", command = "devour", sql = sql);
        let result: Vec<SE> = self.fetch_all_plain(&sql, selection).await?;
        debug!(target: "taitan_orm", command = "devour", result = ?result);
        Ok(result)
    }

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
        let record_count = self.count_table(selection.get_table_name()).await?;
        if record_count <= 0 {
            return Ok(PagedList::empty(page.page_size, page.page_num));
        }

        debug!(target: "taitan_orm", command = "devour_paged", selection = ?selection);
        let sql = self.get_generator().get_devour_sql(selection, order_by);
        debug!(target: "taitan_orm", command = "devour_paged", sql = sql);
        let entity_list: Vec<SE> = self.fetch_all_plain(&sql, selection).await?;
        let result = build_paged_list(entity_list, record_count, page);
        debug!(target: "taitan_orm", command = "devour_paged", result = ?result);
        Ok(result)
    }

    async fn count(&mut self, location: &dyn Location) -> Result<u64> {
        debug!(target: "taitan_orm", command = "count", location = ?location);
        let args = location.gen_location_arguments_sqlite()?;
        let count_sql = self.get_generator().get_count_sql(location);
        debug!(target: "taitan_orm", command = "count", sql = count_sql);
        let record_count: CountResult = self.fetch_execute(&count_sql, args).await?;
        debug!(target: "taitan_orm", command = "count", result = ?record_count);
        Ok(record_count.count)
    }

    async fn count_table(&mut self, table_name: &str) -> Result<u64> {
        debug!(target: "taitan_orm", command = "count", table_name = ?table_name);
        let count_sql = self.get_generator().get_count_table_sql(table_name);
        debug!(target: "taitan_orm", command = "count", sql = count_sql);
        let record_count: CountResult = self.fetch_execute_plain(&count_sql).await?;
        debug!(target: "taitan_orm", command = "count", result = ?record_count);
        Ok(record_count.count)
    }

    // async fn search_joined<SE>(
    //     &mut self,
    //     joined_conds: JoinedConditions,
    //     locations: Vec<&dyn Location>,
    //     order_by: Option<&dyn OrderBy>,
    //     selections: Vec<&dyn Selection>,
    //     page: &Pagination,
    // ) -> String
    // //-> Result<PagedList<SE>>
    // //where
    // //    SE: SelectedEntity + Send + Unpin,
    // {
    //     self.get_generator().get_page_joined_search_sql(
    //         &joined_conds,
    //         &locations,
    //         order_by,
    //         &selections,
    //         page,
    //     )
    // }

    async fn execute_by_template(&mut self, template: &dyn TemplateRecord) -> Result<usize> {
        debug!(target: "taitan_orm", command = "execute_by_template", template = ?template);
        let sql = template.get_sql(None);
        let sql = self.get_generator().post_process(sql);
        debug!(target: "taitan_orm", command = "execute_by_template", sql = sql);
        let args = template.gen_template_arguments_sqlite()?;
        let result = self.execute::<SqliteArguments>(&sql, args).await?;
        debug!(target: "taitan_orm", command = "execute_by_template", result = ?result);
        Ok(result as usize)
    }

    // async fn select_by_template<SE>(
    //     &mut self,
    //     template: &dyn TemplateRecord,
    // ) -> Result<Option<SE>>
    // where
    //     SE: SelectedEntity<Self::DB> + Send + Unpin
    // {
    //     debug!(target: "taitan_orm", command = "select_by_template", template = ?template);
    //     let sql = template.get_sql(None);
    //     let sql = self.get_generator().post_process(sql);
    //     debug!(target: "taitan_orm", command = "select_by_template", sql = sql);
    //     let args = template.gen_template_arguments_sqlite()?;
    //     let result: Option<SE> = self.fetch_optional::<SqliteArguments>(&sql,   args).await?;
    //     debug!(target: "taitan_orm", command = "select_by_template", result = ?result);
    //     return Ok(result);
    // }

    // async fn search_by_template<SE>(
    //     &mut self,
    //     template: &dyn TemplateRecord,
    // ) -> Result<Vec<SE>>
    // where
    //     SE: SelectedEntity + Send + Unpin,
    // {
    //     debug!(target: "taitan_orm", command = "search_by_template", template = ?template);
    //     let sql = template.get_sql(None);
    //     let sql = self.get_generator().post_process(sql);
    //     debug!(target: "taitan_orm", command = "search_by_template", sql = sql);
    //     let args = template.any_arguments();
    //     let result: Vec<SE> = self.fetch_all(&sql, args).await?;
    //     debug!(target: "taitan_orm", command = "search_by_template", result = ?result);
    //     return Ok(result);
    // }
    //
    // async fn search_paged_by_template<SE>(
    //     &mut self,
    //     template: &dyn TemplateRecord,
    //     page: &Pagination,
    // ) -> Result<PagedList<SE>>
    // where
    //     SE: SelectedEntity + Send + Unpin,
    // {
    //     debug!(target: "taitan_orm", command = "search_paged_by_template", template = ?template, page = ?page);
    //     let count_sql = template.get_count_sql();
    //     let record_count: Option<RecordCount>;
    //     let args = template.any_arguments();
    //     match count_sql {
    //         CountSql::Empty => {
    //             return Err(LunaOrmError::PagedTemplateHasNoCountSql);
    //         }
    //         CountSql::PlainSql(sql) => {
    //             let sql = self.get_generator().post_process(sql);
    //             debug!(target: "taitan_orm", command = "search_paged_by_template", count_sql = sql);
    //             record_count = self.fetch_optional_plain(&sql).await?;
    //         }
    //         CountSql::VariabledSql(sql) => {
    //             let sql = self.get_generator().post_process(sql);
    //             debug!(target: "taitan_orm", command = "search_paged_by_template", count_sql = sql);
    //             record_count = self.fetch_optional(&sql, args).await?;
    //         }
    //     }
    //     if record_count.is_none() {
    //         return Ok(PagedList::empty(page.page_size, page.page_num));
    //     }
    //     let record_count: RecordCount = record_count.unwrap();
    //     if record_count.count <= 0 {
    //         return Ok(PagedList::empty(page.page_size, page.page_num));
    //     }
    //     let record_count: i64 = record_count.count;
    //
    //     let sql = template.get_sql(Some(page));
    //     let sql = self.get_generator().post_process(sql);
    //     debug!(target: "taitan_orm", command = "search_paged_by_template", sql = sql);
    //     let args = template.any_arguments();
    //     let entity_list: Vec<SE> = self.fetch_all(&sql, args).await?;
    //
    //     let page_info = PageInfo {
    //         page_size: page.page_size,
    //         page_num: page.page_num,
    //         page_total: (record_count / page.page_size as i64) as usize,
    //         total: record_count as usize,
    //     };
    //     let result = PagedList {
    //         data: entity_list,
    //         page: page_info,
    //     };
    //     debug!(target: "taitan_orm", command = "search_paged_by_template", result = ?result);
    //     Ok(result)
    // }
}
