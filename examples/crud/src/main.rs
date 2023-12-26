use luna_orm::prelude::*;
use luna_orm::LunaOrmResult;

#[derive(Entity, Clone, Debug)]
#[TableName = "user"]
pub struct HelloEntity {
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

    let result = db.execute_plain(
        "CREATE TABLE IF NOT EXISTS `user`(`id` INT PRIMARY KEY, `age` INT, `name` VARCHAR(64))",
    ).await?;

    let entity = HelloEntity {
        id: 1,
        name: "Allen".to_string(),
        age: Some(23),
    };

    let result = db.insert(&entity).await?;

    println!("result: {}", result);
    Ok(())
}
