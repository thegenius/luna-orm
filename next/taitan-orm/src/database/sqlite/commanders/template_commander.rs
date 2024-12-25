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
async fn procedure_by_template<SE>(template) -> SE

// selection location order_by pagination all should embed in template
async fn select_by_template<SE>(template) -> Result<Option<SE>>
async fn search_by_template<SE>(template) -> Result<Vec<SE>>
async fn search_paged_by_template<SE>(template) -> Result<PagedList<SE>>
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

    async fn procedure_by_template<SE>(&mut self, template: &dyn TemplateRecord) -> Result<SE>
    where
        SE: SelectedEntity<Self::DB> + Send + Unpin,
    {
        debug!(target: "taitan_orm", command = "procedure_by_template", template = ?template);
        let sql = template.get_sql(None);
        let sql = self.get_generator().post_process(sql);
        debug!(target: "taitan_orm", command = "procedure_by_template", sql = sql);
        let args = template.gen_template_arguments_sqlite()?;
        let result: SE = self.fetch_execute(&sql, args).await?;
        debug!(target: "taitan_orm", command = "procedure_by_template", result = ?result);
        Ok(result)
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
        let result: Option<SE> = self.fetch_execute_option(&sql, args).await?;
        debug!(target: "taitan_orm", command = "select_by_template", result = ?result);
        Ok(result)
    }

    async fn search_by_template<SE>(&mut self, template: &dyn TemplateRecord) -> Result<Vec<SE>>
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

    async fn search_paged_by_template<SE>(
        &mut self,
        template: &dyn TemplateRecord,
    ) -> Result<PagedList<Self::DB, SE>>
    where
        SE: SelectedEntity<Self::DB> + Send + Unpin,
    {
        debug!(target: "taitan_orm", command = "search_paged_by_template", template = ?template);
        let count_sql = template
            .get_count_sql()
            .ok_or(TaitanOrmError::TemplatePagedNotHasCountSql)?;
        let count_sql = self.get_generator().post_process(count_sql);
        debug!(target: "taitan_orm", command = "search_paged_by_template", count_sql = count_sql);
        let page = template
            .get_pagination()
            .ok_or(TaitanOrmError::TemplatePageFieldNotFound)?;

        let count_args = template.gen_template_count_arguments_sqlite()?;
        let count_result_opt: Option<CountResult> =
            self.fetch_execute_option(&count_sql, count_args).await?;
        let record_count = count_result_opt.unwrap_or_default().count;
        if record_count <= 0 {
            return Ok(PagedList::empty(page.page_size, page.page_num));
        }

        let sql = template.get_sql(Some(page));
        let sql = self.get_generator().post_process(sql);
        debug!(target: "taitan_orm", command = "search_paged_by_template", sql = sql);
        let args = template.gen_template_arguments_sqlite()?;
        let entity_list: Vec<SE> = self.fetch_execute_all(&sql, args).await?;

        let page_info = PageInfo {
            page_size: page.page_size,
            page_num: page.page_num,
            page_total: record_count / page.page_size,
            total: record_count,
        };
        let result = PagedList {
            data: entity_list,
            page: page_info,
            _phantom: PhantomData,
        };
        debug!(target: "taitan_orm", command = "search_paged_by_template", result = ?result);
        Ok(result)
    }
}
