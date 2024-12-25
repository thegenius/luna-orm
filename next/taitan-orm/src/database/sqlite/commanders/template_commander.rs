use crate::database::sqlite::SqliteLocalConfig;
use crate::sql_generator::DefaultSqlGenerator;
use crate::{CountResult, Result};
use crate::{SqlApi, SqlExecutor, SqlGenerator, TaitanOrmError};
use path_absolutize::Absolutize;
use std::fmt::Debug;
// use sqlx::error::BoxDynError;
use crate::sql_generator_container::SqlGeneratorContainer;
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

/**
async fn execute_by_template(template) -> Result<u64>

// selection location order_by pagination all should embed in template
async fn select_by_template<SE>(template) -> Result<Option<SE>>
async fn search_by_template<SE>(template) -> Result<Vec<SE>>
async fn search_paged_by_template<SE>(template) -> Result<PagedList<SE>>

async fn procedure_by_template<SE>(template) -> SE
*/
pub trait SqliteTemplateCommander: SqlExecutor<DB = Sqlite> + SqlGeneratorContainer {
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

    async fn select_by_template<SE>(&mut self, template: &dyn TemplateRecord) -> Result<Option<SE>>
    where
        SE: SelectedEntity<Self::DB> + Send + Unpin,
    {
        debug!(target: "taitan_orm", command = "select_by_template", template = ?template);
        let sql = template.get_sql(None);
        let sql = self.get_generator().post_process(sql);
        debug!(target: "taitan_orm", command = "select_by_template", sql = sql);
        let args = template.gen_template_arguments_sqlite()?;
        let result: Option<SE> = self
            .fetch_execute_option(&sql, args)
            .await?;
        debug!(target: "taitan_orm", command = "select_by_template", result = ?result);
        Ok(result)
    }

    // async fn search_by_template<SE>(template) -> Result<Vec<SE>>
    async fn search_by_template<SE>(
        &mut self,
        template: &dyn TemplateRecord,
    ) -> Result<Vec<SE>>
    where
        SE: SelectedEntity<Self::DB> + Send + Unpin,
    {
        debug!(target: "taitan_orm", command = "search_by_template", template = ?template);
        let sql = template.get_sql(None);
        let sql = self.get_generator().post_process(sql);
        debug!(target: "taitan_orm", command = "search_by_template", sql = sql);
        let args = template.gen_template_arguments_sqlite()?;
        let result: Vec<SE> = self.fetch_execute_all(&sql, args).await?;
        debug!(target: "taitan_orm", command = "search_by_template", result = ?result);
        Ok(result)
    }


    // async fn search_paged_by_template<SE>(
    //     &mut self,
    //     template: &dyn TemplateRecord,
    //     page: &Pagination,
    // ) -> Result<PagedList<Self::DB, SE>>
    // where
    //     SE: SelectedEntity<Self::DB> + Send + Unpin,
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
