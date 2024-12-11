use crate::database::sqlite::SqliteLocalConfig;
use crate::sql_generator::DefaultSqlGenerator;
use crate::{CountResult, PageInfo, PagedList, Pagination, Result};
use crate::{LunaOrmError, SqlApi, SqlExecutor, SqlGenerator};
use path_absolutize::Absolutize;
use sqlx::pool::PoolConnection;
use sqlx::sqlite::{SqliteArguments, SqliteConnectOptions, SqliteJournalMode, SqliteSynchronous};
use sqlx::{Sqlite, SqlitePool};
use std::fs;
use std::marker::PhantomData;
use std::path::Path;
use taitan_orm_trait::{Entity, Location, Mutation, OrderBy, Primary, SelectedEntity, Selection};
use tracing::debug;
use crate::dto::CountSelection;

#[derive(Debug, Clone)]
pub struct SqliteCommander {
    sql_generator: DefaultSqlGenerator,
    pub(crate) sqlite_pool: SqlitePool,
}

impl SqliteCommander {
    async fn init_local(workspace_dir: &str, db_file: &str) -> Result<SqlitePool> {
        let workspace = Path::new(workspace_dir);
        let workspace_absolute = workspace
            .absolutize()
            .map_err(|_e| LunaOrmError::DatabaseInitFail("workdir absolute fail".to_string()))?;

        fs::create_dir_all(&workspace_absolute)
            .map_err(|_e| LunaOrmError::DatabaseInitFail("create dir fail".to_string()))?;
        let db_file_path = workspace_absolute.join(db_file);

        let options = SqliteConnectOptions::new()
            .filename(db_file_path.clone())
            .synchronous(SqliteSynchronous::Full)
            .journal_mode(SqliteJournalMode::Wal)
            .create_if_missing(true);
        let sqlite_pool = SqlitePool::connect_with(options)
            .await
            .map_err(|_e| LunaOrmError::DatabaseInitFail("create is missing fail".to_string()))?;
        Ok(sqlite_pool)
    }

    pub async fn build(config: SqliteLocalConfig<'_>) -> Result<Self> {
        let pool = SqliteCommander::init_local(&config.work_dir, &config.db_file).await?;
        let generator = DefaultSqlGenerator::new();
        let database = SqliteCommander {
            sql_generator: generator,
            sqlite_pool: pool,
        };
        Ok(database)
    }
}

impl SqlApi for SqliteCommander {
    type G = DefaultSqlGenerator;

    fn get_generator(&self) -> &Self::G {
        &self.sql_generator
    }

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

    async fn select<SE>(
        &self,
        primary: &dyn Primary,
        selection: &SE::Selection,
    ) -> Result<Option<SE>>
    where
        SE: SelectedEntity<Self::DB> + Send + Unpin,
    {
        debug!(target: "taitan_orm", command = "select", primary = ?primary, selection = ?selection);
        let sql = self.get_generator().get_select_sql(selection, primary);
        debug!(target: "taitan_orm", command = "select", sql = sql);
        let args: SqliteArguments<'_> = primary.gen_primary_arguments_sqlite()?;
        let result: Option<SE> = self.fetch_optional(&sql, selection, args).await?;
        debug!(target: "taitan_orm", command = "select", result = ?result);
        Ok(result)
    }

    async fn update<M: Mutation>(&mut self, mutation: &M, primary: &M::Primary) -> Result<bool> {
        debug!(target: "taitan_orm", command = "update", mutation = ?mutation, primary = ?primary);
        let sql = self.get_generator().get_update_sql(mutation, primary);
        debug!(target: "taitan_orm", command = "update", sql = sql);
        let args = mutation.gen_update_arguments_sqlite(primary)?;
        let result = self.execute::<SqliteArguments>(&sql, args).await?;
        debug!(target: "taitan_orm", command = "update", result = ?result);
        Ok(result > 0)
    }

    async fn delete(&mut self, primary: &dyn Primary) -> Result<bool> {
        debug!(target: "taitan_orm", command = "delete", primary = ?primary);
        let sql = self.get_generator().get_delete_sql(primary);
        debug!(target: "taitan_orm", command = "delete", sql = sql);
        let args = primary.gen_primary_arguments_sqlite()?;
        let result = self.execute::<SqliteArguments>(&sql, args).await?;
        debug!(target: "taitan_orm", command = "delete", result = ?result);
        Ok(result > 0)
    }

    async fn fetch<SE>(&mut self, selection: &SE::Selection) -> Result<Vec<SE>>
    where
        SE: SelectedEntity<Self::DB> + Send + Unpin
    {
        debug!(target: "taitan_orm", command = "search_all", selection = ?selection);
        let sql = self.get_generator().get_search_all_sql(selection);
        debug!(target: "taitan_orm", command = "search_all", sql = sql);
        let result: Vec<SE> = self.fetch_all_plain(&sql, selection).await?;
        debug!(target: "taitan_orm", command = "search_all", result = ?result);
        Ok(result)
    }


    async fn search<SE>(
        &mut self,
        selection: &SE::Selection,
        location: &dyn Location,
        order_by: Option<&dyn OrderBy>
    ) -> Result<Vec<SE>>
    where
        SE: SelectedEntity<Self::DB> + Send + Unpin
    {
        debug!(target: "taitan_orm", command = "search", location = ?location, order_by = ?order_by, selection = ?selection);
        let sql = self
            .get_generator()
            .get_search_sql(selection, location, order_by);
        debug!(target: "taitan_orm", command = "search", sql = sql);
        if order_by.is_some() {
            let order_by_fields = order_by.unwrap().get_order_by_fields();
            let valid_order_by = location.check_valid_order_by(order_by_fields);
            if !valid_order_by {
                return Err(LunaOrmError::OrderByFieldsError);
            }
        }
        let args = location.gen_location_arguments_sqlite()?;
        let result: Vec<SE> = self.fetch_all(&sql, selection, args).await?;
        debug!(target: "taitan_orm", command = "search", result = ?result);
        Ok(result)
    }

    async fn count(&mut self, location: &dyn Location) -> Result<usize> {
        debug!(target: "taitan_orm", command = "count", location = ?location);
        let args = location.gen_location_arguments_sqlite()?;
        let count_sql = self.get_generator().get_search_count_sql(location);
        debug!(target: "taitan_orm", command = "count", sql = count_sql);
        let record_count: Option<CountResult> = self.fetch_optional(&count_sql, &CountSelection::default(), args).await?;
        debug!(target: "taitan_orm", command = "count", result = ?record_count);
        if record_count.is_none() {
            Ok(0)
        } else {
            Ok(record_count.unwrap().count as usize)
        }
    }


    async fn search_paged<SE>(
        &mut self,
        selection: &SE::Selection,
        location: &dyn Location,
        page: &Pagination,
        order_by: Option<&dyn OrderBy>
    ) -> Result<PagedList<Self::DB, SE>>
    where
        SE: SelectedEntity<Self::DB> + Send + Unpin
    {
        debug!(target: "taitan_orm", command = "search_paged", location = ?location, order_by = ?order_by, selection = ?selection, page = ?page);
        if order_by.is_some() {
            let order_by_fields = order_by.unwrap().get_order_by_fields();
            let valid_order_by = location.check_valid_order_by(order_by_fields);
            if !valid_order_by {
                return Err(LunaOrmError::OrderByFieldsError);
            }
        }
        let args = location.gen_location_arguments_sqlite()?;
        let count_sql = self.get_generator().get_search_count_sql(location);
        debug!(target: "taitan_orm", command = "search_paged", count_sql = count_sql);
        let record_count: Option<CountResult> = self.fetch_optional(&count_sql, &CountSelection::default(), args).await?;
        if record_count.is_none() {
            return Ok(PagedList::empty(page.page_size, page.page_num));
        }
        let record_count: CountResult = record_count.unwrap();
        if record_count.count <= 0 {
            return Ok(PagedList::empty(page.page_size, page.page_num));
        }
        let record_count: i64 = record_count.count;

        let sql = self
            .get_generator()
            .get_paged_search_sql(selection, location, order_by, page);
        debug!(target: "taitan_orm", command = "search_paged", sql = sql);
        let args = location.gen_location_arguments_sqlite()?;
        let entity_list: Vec<SE> = self.fetch_all(&sql, selection, args).await?;
        let page_info = PageInfo {
            page_size: page.page_size,
            page_num: page.page_num,
            page_total: (record_count / page.page_size as i64) as usize,
            total: record_count as usize,
        };

        let result = PagedList {
            data: entity_list,
            page: page_info,
            _phantom: PhantomData,
        };
        debug!(target: "taitan_orm", command = "search_paged", result = ?result);
        Ok(result)
    }

    //
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
    //
    // async fn purify(&mut self, location: &dyn Location) -> Result<usize> {
    //     debug!(target: "taitan_orm", command = "purify", location = ?location);
    //     let sql = self.get_generator().get_purify_sql(location);
    //     debug!(target: "taitan_orm", command = "purify", sql = sql);
    //     let args = location.any_arguments();
    //     let result = self.execute(&sql, args).await?;
    //     debug!(target: "taitan_orm", command = "purify", result = ?result);
    //     return Ok(result.rows_affected() as usize);
    // }
    //
    // async fn change(
    //     &mut self,
    //     mutation: &dyn Mutation,
    //     location: &dyn Location,
    // ) -> Result<usize> {
    //     debug!(target: "taitan_orm", command = "change", mutation = ?mutation, location = ?location);
    //     let sql = self.get_generator().get_change_sql(mutation, location);
    //     debug!(target: "taitan_orm", command = "change", sql = sql);
    //     let mut args = mutation.any_arguments();
    //     let where_args = location.any_arguments();
    //     args = luna_merge_args(args, where_args);
    //     let result = self.execute(&sql, args).await?;
    //     debug!(target: "taitan_orm", command = "change", result = ?result);
    //     return Ok(result.rows_affected() as usize);
    // }
    //
    // async fn execute_by_template(&mut self, template: &dyn TemplateRecord) -> Result<usize> {
    //     debug!(target: "taitan_orm", command = "execute_by_template", template = ?template);
    //     let sql = template.get_sql(None);
    //     let sql = self.get_generator().post_process(sql);
    //     debug!(target: "taitan_orm", command = "execute_by_template", sql = sql);
    //     let args = template.any_arguments();
    //     let result = self.execute(&sql, args).await?;
    //     debug!(target: "taitan_orm", command = "execute_by_template", result = ?result);
    //     return Ok(result.rows_affected() as usize);
    // }
    //
    // async fn select_by_template<SE>(
    //     &mut self,
    //     template: &dyn TemplateRecord,
    // ) -> Result<Option<SE>>
    // where
    //     SE: SelectedEntity + Send + Unpin,
    // {
    //     debug!(target: "taitan_orm", command = "select_by_template", template = ?template);
    //     let sql = template.get_sql(None);
    //     let sql = self.get_generator().post_process(sql);
    //     debug!(target: "taitan_orm", command = "select_by_template", sql = sql);
    //     let args = template.any_arguments();
    //     let result: Option<SE> = self.fetch_optional(&sql, args).await?;
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
