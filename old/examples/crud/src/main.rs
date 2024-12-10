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
    let entity = UserEntity {
        id: 1,
        name: "Allen".to_string(),
        age: Some(23),
    };
    let result = db.insert(&entity).await?;

    // 2. update
    let mutation = UserMutation {
        name: None,
        age: Some(24),
    };
    let primary = UserPrimary { id: 1 };
    let result = db.update(&mutation, &primary).await?;

    // 3. select
    let selection = UserSelection {
        id: false,
        name: true,
        age: true,
    };
    let entity: Option<UserSelectedEntity> = db.select(&primary, &selection).await?;
    let expect_entity = UserSelectedEntity {
        id: None,
        name: Some("Allen".to_string()),
        age: Some(24),
    };
    assert_eq!(entity, Some(expect_entity));

    // 4. delete
    let _ = db.delete(&primary).await?;
    let entity: Option<UserSelectedEntity> = db.select(&primary, &selection).await?;
    assert_eq!(entity, None);

    println!("result: {}", result);
    Ok(())
}
