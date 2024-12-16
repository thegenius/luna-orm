use sqlx::sqlx_macros;
use taitan_orm_macro::Schema;
use taitan_orm_trait::{Entity, Schema};
use sqlx::mysql::MySqlArguments;
use sqlx::sqlite::SqliteArguments;
use sqlx::postgres::PgArguments;
use sqlx::error::BoxDynError;
use taitan_orm_trait::NotImplementError;

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