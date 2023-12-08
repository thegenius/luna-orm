#![allow(dead_code)]

mod database;
mod error;
mod mapper;
mod sql_generator;
mod transaction;

pub type LunaOrmResult<T> = std::result::Result<T, error::LunaOrmError>;

pub mod prelude {
    pub use crate::database::*;
    pub use crate::mapper::GenericDaoMapper;
    pub use crate::mapper::GenericDaoMapperImpl;
    pub use crate::transaction::Transaction;
    pub use luna_orm_macro::*;
    pub use luna_orm_trait::*;
    pub use sqlx::any::AnyArguments;
    pub use sqlx::any::AnyRow;
    pub use sqlx::Arguments;
    pub use sqlx::Row;
}
