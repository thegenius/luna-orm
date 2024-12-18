
use sqlx::sqlx_macros;
use taitan_orm_macro::Schema;

use sqlx::Arguments; // 大量使用args.add，没法去掉
use sqlx::Row; // 因为使用了try_get

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
