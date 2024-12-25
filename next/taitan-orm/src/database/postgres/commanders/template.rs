use crate::database::sqlite::SqliteLocalConfig;
use crate::sql_generator::DefaultSqlGenerator;
use crate::{execute_by_template_fn, fetch_all_by_template_fn, fetch_one_by_template_fn, fetch_option_by_template_fn, fetch_paged_by_template_fn, CountResult, Result};
use crate::{SqlApi, SqlExecutor, SqlGenerator, TaitanOrmError};
use path_absolutize::Absolutize;
use std::fmt::Debug;
// use sqlx::error::BoxDynError;
use crate::sql_generator_container::SqlGeneratorContainer;
use sqlx::sqlite::{SqliteArguments, SqliteConnectOptions, SqliteJournalMode, SqliteSynchronous};
use sqlx::{Database, MySql, Postgres, Sqlite, SqlitePool};
use std::fs;
use std::marker::PhantomData;
use std::path::Path;
use sqlx::mysql::MySqlArguments;
use sqlx::postgres::PgArguments;
use taitan_orm_trait::page_info::PageInfo;
use taitan_orm_trait::paged_list::PagedList;
use taitan_orm_trait::pagination::Pagination;
use taitan_orm_trait::{
    Entity, Location, Mutation, OrderBy, SelectedEntity, Selection, TemplateRecord, Unique,
};
use tracing::debug;

pub trait PostgresTemplateCommander: SqlExecutor<DB = Postgres> + SqlGeneratorContainer {

    execute_by_template_fn!(PgArguments, TemplateRecord::gen_template_arguments_postgres);

    fetch_one_by_template_fn!(TemplateRecord::gen_template_arguments_postgres);

    fetch_option_by_template_fn!(TemplateRecord::gen_template_arguments_postgres);

    fetch_all_by_template_fn!(TemplateRecord::gen_template_arguments_postgres);

    fetch_paged_by_template_fn!(TemplateRecord::gen_template_count_arguments_postgres, TemplateRecord::gen_template_arguments_postgres);
}
