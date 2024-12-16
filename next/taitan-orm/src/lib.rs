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

// pub use db::DB;
pub use dto::CountResult;
pub use error::TaitanOrmError;
pub use result::Result;
pub use sql_api::SqlApi;
pub use sql_executor::SqlExecutor;
pub use sql_generator::SqlGenerator;
