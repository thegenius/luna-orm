
use sqlx::sqlx_macros;
use time::PrimitiveDateTime;
use uuid::Uuid;
use taitan_orm::Schema;
use taitan_orm_trait::Optional;

#[derive(Schema, Clone, Debug)]
#[table_name = "user"]
#[unique_key = "age"]
#[unique_key = "name, birthday"]
pub struct UserEntity {
    #[primary_key]
    #[auto_increment]
    id: Optional<i64>,

    pub request_id: Uuid,

    age: Optional<i32>,

    name: String,

    pub birthday: Optional<PrimitiveDateTime>,
}

#[sqlx_macros::test]
pub async fn entity_macro_spec() -> taitan_orm::Result<()> {




    Ok(())
}
