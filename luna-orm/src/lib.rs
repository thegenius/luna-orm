//!
//! # LUNA-ORM
//! luna-orm is build for time saving
//!
//! **LUNA-ORM** is an async orm framework based on SQLx. Built with :heart:
//! -  **Intuitive** : Simple API, the most simple orm in this world.
//! -  **Time Saving** : Most useful API is implemented by default, no need to waste your life.
//! -  **Smooth Transaction** : Transaction is almost same as normal.
//! -  **Template SQL** : You can execute your own sql with no pain.
//! -  **Dynamic Parameters** : Handle complex dynamic sql with default.
//! -  **Truly Asynchronous** : Based on SQLx, luna-orm is fully async.
//! -  **Error Soundly** : Every error has its meaning.
//!
//!

#![allow(dead_code)]
#![allow(async_fn_in_trait)]
#![forbid(unsafe_code)]
//#![feature(trait_upcasting)]
//#![allow(incomplete_features)]

mod command_executor;
// mod command_executor2;

mod database;
mod error;
mod mapper;
mod sql_executor;
mod sql_generator;

mod transaction;
mod sql_executor2;

pub type LunaOrmResult<T> = std::result::Result<T, error::LunaOrmError>;

pub mod prelude {
    pub use luna_orm_trait::input_generator::InputGenerator;
    pub use crate::sql_executor2::SqlExecutorNew;
    pub use crate::command_executor::CommandExecutor;
    pub use crate::database::*;
    pub use crate::error::*;
    pub use crate::sql_executor::*;
    pub use crate::sql_generator::*;
    pub use crate::transaction::Transaction;
    pub use luna_orm_macro::*;
    pub use luna_orm_trait::*;
    pub use sqlx::any::AnyArguments;
    pub use sqlx::any::AnyRow;
    pub use sqlx::Arguments;
    pub use sqlx::Row;
}
