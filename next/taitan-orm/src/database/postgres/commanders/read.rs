use crate::database::sqlite::SqliteLocalConfig;
use crate::sql_generator::DefaultSqlGenerator;
use crate::{count_all_fn, count_fn, devour_fn, devour_paged_fn, exists_fn, search_fn, search_paged_fn, select_fn, CountResult, Result};
use crate::{SqlApi, SqlExecutor, SqlGenerator, TaitanOrmError};
use path_absolutize::Absolutize;
use std::fmt::Debug;
// use sqlx::error::BoxDynError;
use crate::sql_generator_container::SqlGeneratorContainer;
use sqlx::postgres::{PgArguments};
use sqlx::{Database, MySql, Postgres, Sqlite, SqlitePool};
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
        page_total: (record_count + page.page_size - 1) / page.page_size, // ceil
        total: record_count,
    };

    PagedList {
        data,
        page: page_info,
        _phantom: PhantomData,
    }
}


pub trait PostgresReadCommander: SqlExecutor<DB = Postgres> + SqlGeneratorContainer {

    exists_fn!(PgArguments, Unique::gen_unique_arguments_postgres);

    count_fn!(PgArguments, Location::gen_location_arguments_postgres);

    count_all_fn!(PgArguments);

    select_fn!(Unique::gen_unique_arguments_postgres);

    search_fn!(Location::gen_location_arguments_postgres);

    search_paged_fn!(Location::gen_location_arguments_postgres);

    devour_fn!(Pagination::gen_page_arguments_postgres);

    devour_paged_fn!(Pagination::gen_page_arguments_postgres);
}
