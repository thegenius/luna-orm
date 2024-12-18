
use sqlx::sqlx_macros;
use taitan_orm_macro::Schema;

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
