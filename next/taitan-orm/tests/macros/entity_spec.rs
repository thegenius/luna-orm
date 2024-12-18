
use sqlx::sqlx_macros;
use time::PrimitiveDateTime;
use uuid::Uuid;
use taitan_orm_macro::Schema;

#[derive(Schema, Clone, Debug)]
#[TableName = "user"]
pub struct UserEntity {
    #[PrimaryKey]
    #[AutoIncrement]
    id: Option<i64>,

    pub request_id: Uuid,

    age: Option<i32>,
    
    name: String,

    pub birthday: Option<PrimitiveDateTime>,
}

#[sqlx_macros::test]
pub async fn entity_macro_spec() -> taitan_orm::Result<()> {




    Ok(())
}
