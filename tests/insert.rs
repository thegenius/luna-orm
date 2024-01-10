use luna_orm::prelude::*;
use luna_orm::LunaOrmResult;
mod common;
use common::create_table;
use common::setup_database;
use common::setup_logger;
use tracing::debug;

#[derive(Schema, Clone, Debug)]
#[TableName = "user"]
pub struct UserEntity {
    #[PrimaryKey]
    #[AutoIncrement]
    id: Option<i32>,
    age: Option<i32>,
    name: String,
}

#[tokio::test]
pub async fn test_insert_normal() -> LunaOrmResult<()> {
    setup_logger();
    let mut db = setup_database().await?;
    create_table(&mut db, "user" , 
        "create table if not exists `user`(`id` integer primary key autoincrement, `age` INT, `name` VARCHAR(60), create_time DATETIME default current_timestamp)" ).await?;
    let entity = UserEntity {
        id: None,
        age: Some(23),
        name: "test".to_string(),
    };

    let sql = db.get_generator().get_insert_sql(&entity);
    let result = db.insert(&entity).await?;
    assert_eq!(result, true);

    let selection = UserSelection {
        id: true,
        age: true,
        name: true,
    };
    let entities: Vec<UserSelectedEntity> = db.search_all(&selection).await?;
    dbg!(&entities);

    let location = UserLocation {
        id: None,
        age: Some(LocationExpr {
            val: 23,
            cmp: CmpOperator::Eq,
        }),
        name: None,
    };
    let selected: Vec<UserSelectedEntity> = db.search(&location, None, &selection).await?;
    let sql = db
        .get_generator()
        .get_search_sql(&selection, &location, None);
    dbg!(&sql);
    assert_eq!(selected.len(), 1);
    let selected_one = selected.first().unwrap();
    assert_eq!(selected_one.id, Some(1));
    assert_eq!(selected_one.age, Some(23));
    assert_eq!(selected_one.name, Some("test".to_string()));

    Ok(())
}
