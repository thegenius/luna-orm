#![allow(async_fn_in_trait)]
#![allow(dead_code)]
#![forbid(unsafe_code)]
mod error;
mod result;

pub mod database;
mod sql_api;

mod db;
mod dto;
mod sql_executor;
mod sql_generator;
pub mod sql_generator_container;

// pub use db::DB;
pub use dto::CountResult;
pub use error::TaitanOrmError;
pub use result::Result;
pub use sql_api::SqlApi;
pub use sql_executor::SqlExecutor;
pub use sql_generator::SqlGenerator;

pub use taitan_orm_macro::Schema;
pub mod traits {
    pub use taitan_orm_trait::{CountSql, Entity, Location, LocationExpr, Mutation, OrderBy, Schema, SelectedEntity, Selection, Unique};
    pub use taitan_orm_trait::validate_order_by;
    pub use taitan_orm_trait::pagination::Pagination;
    pub use taitan_orm_trait::page_info::PageInfo;
    pub use taitan_orm_trait::paged_list::PagedList;
    pub use taitan_orm_trait::ParsedTemplateSql;
    pub use taitan_orm_trait::TemplateValue;
    pub use taitan_orm_trait::TemplateRecord;
}