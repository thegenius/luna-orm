use sqlx::error::BoxDynError;
use sqlx::mysql::MySqlArguments;
use sqlx::postgres::PgArguments;
use sqlx::sqlite::SqliteArguments;
use sqlx::sqlx_macros;
use sqlx::Arguments;
use taitan_orm_macro::Schema;
use taitan_orm_trait::NotImplementError;
use taitan_orm_trait::{Entity, Unique, Location, Schema, LocationExpr, Mutation, Selection, SelectedEntity};
use sqlx::{Sqlite, MySql, Postgres};
use sqlx::Database;
use sqlx::Row;

#[derive(Schema, Clone, Debug)]
#[TableName = "user"]
pub struct UserEntity {
    age: Option<i32>,

    #[PrimaryKey]
    #[AutoIncrement]
    id: Option<i64>,

    name: String,
}

#[sqlx_macros::test]
pub async fn entity_macro_spec() -> taitan_orm::Result<()> {
    Ok(())
}
