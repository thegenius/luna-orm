use luna_orm::prelude::*;

#[derive(Entity, Debug, Clone)]
#[TableName = "article"]
pub struct ArticleEntity {
    #[PrimaryKey]
    id: i32,
}
