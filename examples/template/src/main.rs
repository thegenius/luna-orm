use luna_orm::prelude::*;
use luna_orm::LunaOrmResult;

#[derive(Schema, Clone, Debug)]
#[TableName = "user"]
pub struct UserEntity {
    #[PrimaryKey]
    id: i32,
    name: String,
    age: Option<i32>,
}

#[derive(TemplateRecord)]
#[TemplateSql = "select id, name, age FROM `user` where id > #{id}"]
#[TemplateCountSql = "select count(*) as count FROM `user` where id > #{id}"]
pub struct HelloSelectTemplate {
    id: i32,
}

#[tokio::main]
async fn main() -> LunaOrmResult<()> {
    let config = SqliteLocalConfig {
        work_dir: "./workspace".to_string(),
        db_file: "test.db".to_string(),
    };

    let mut db: DB<SqliteDatabase> = SqliteDatabase::build(config).await.unwrap().into();

    let result = db.execute_plain("DROP TABLE IF EXISTS `user`").await?;
    let result = db.execute_plain(
        "CREATE TABLE IF NOT EXISTS `user`(`id` INT PRIMARY KEY, `age` INT, `name` VARCHAR(64))",
    ).await?;

    // 1. insert entity
    let entity1 = UserEntity {
        id: 1,
        name: "Allen".to_string(),
        age: Some(23),
    };

    let entity2 = UserEntity {
        id: 2,
        name: "Bob".to_string(),
        age: Some(23),
    };

    let entity3 = UserEntity {
        id: 3,
        name: "Carter".to_string(),
        age: Some(23),
    };
    let _ = db.insert(&entity1).await?;
    let _ = db.insert(&entity2).await?;
    let _ = db.insert(&entity3).await?;

    let template = HelloSelectTemplate { id: 0 };
    let page = Pagination {
        page_size: 1,
        page_num: 1,
    };
    let entities: PagedList<UserSelectedEntity> =
        db.search_paged_by_template(&template, &page).await?;
    assert_eq!(entities.page.total, 3);
    assert_eq!(entities.data.get(0).unwrap().name, Some("Bob".to_string()));

    Ok(())
}
