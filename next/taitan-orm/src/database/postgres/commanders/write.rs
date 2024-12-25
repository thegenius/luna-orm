use crate::database::sqlite::SqliteLocalConfig;
use crate::sql_generator::DefaultSqlGenerator;
use crate::{change_fn, delete_fn, insert_fn, purify_fn, update_fn, upsert_fn, CountResult, Result};
use crate::{SqlApi, SqlExecutor, SqlGenerator, TaitanOrmError};
use path_absolutize::Absolutize;
use std::fmt::Debug;
// use sqlx::error::BoxDynError;
use crate::sql_generator_container::SqlGeneratorContainer;
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

pub trait PostgresWriteCommander: SqlExecutor<DB = Postgres> + SqlGeneratorContainer {

    insert_fn!(PgArguments, Entity::gen_insert_arguments_postgres);

    upsert_fn!(PgArguments, Entity::gen_upsert_arguments_postgres);

    update_fn!(PgArguments, Unique::gen_update_arguments_postgres);

    change_fn!(PgArguments, Mutation::gen_change_arguments_postgres);

    delete_fn!(PgArguments, Unique::gen_unique_arguments_postgres);

    purify_fn!(PgArguments, Location::gen_location_arguments_postgres);
}