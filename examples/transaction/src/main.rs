use luna_orm::prelude::*;
use luna_orm::LunaOrmResult;

#[derive(Schema, Clone, Debug)]
#[TableName = "user"]
pub struct UserEntity {
    #[PrimaryKey]
    id: i32,
    name: String,
    cash: Option<i32>,
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
        "CREATE TABLE IF NOT EXISTS `user`(`id` INT PRIMARY KEY, `cash` INT, `name` VARCHAR(64))",
    ).await?;

    // 1. insert entity
    let entity = UserEntity {
        id: 1,
        name: "Allen".to_string(),
        cash: Some(100),
    };
    db.insert(&entity).await?;
    let entity = UserEntity {
        id: 2,
        name: "Bob".to_string(),
        cash: Some(0),
    };
    db.insert(&entity).await?;

    // 2. execute transaction commands
    let mut trx = db.transaction().await?;
    let mutation1 = UserMutation {
        name: None,
        cash: Some(50),
    };
    let primary1 = UserPrimary { id: 1 };
    let mutation2 = UserMutation {
        name: None,
        cash: Some(50),
    };
    let primary2 = UserPrimary { id: 2 };
    trx.update(&mutation1, &primary1).await?;
    trx.update(&mutation2, &primary2).await?;
    trx.commit().await?;

    // 3. check
    let selection = UserSelection {
        id: false,
        name: true,
        cash: true,
    };
    let entity1: Option<UserSelectedEntity> = db.select(&primary1, &selection).await?;
    let entity2: Option<UserSelectedEntity> = db.select(&primary2, &selection).await?;
    assert_eq!(entity1.unwrap().cash, Some(50));
    assert_eq!(entity2.unwrap().cash, Some(50));

    Ok(())
}
