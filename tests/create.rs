use luna_orm::prelude::*;
use luna_orm::LunaOrmResult;
mod common;
use common::create_table;
use common::mutex::get_test_mutex;
use common::setup_database;
use common::setup_logger;
use sqlx::sqlx_macros;
use tracing::debug;

#[derive(Schema, Clone, Debug)]
#[TableName = "user"]
pub struct UserEntity {
    #[PrimaryKey]
    #[AutoIncrement]
    id: Option<i64>,
    age: Option<i32>,
    name: String,
}

#[sqlx_macros::test]
pub async fn test_create() -> LunaOrmResult<()> {
    let test_mutex = get_test_mutex();
    let test_lock = test_mutex.lock();
    setup_logger();
    let mut db = setup_database().await?;
    create_table(&mut db, "user" , 
        "create table if not exists `user`(`id` integer primary key autoincrement, `age` INT, `name` VARCHAR(60), create_time DATETIME default current_timestamp)" ).await?;
    let mut entity = UserEntity {
        id: None,
        age: Some(23),
        name: "test".to_string(),
    };

    // sqlx sqlite has bug #2099, it does not actually commit insert returning stmt
    let result = db.create(&mut entity).await?;
    assert_eq!(entity.id, Some(1));

    let db_clone = db.clone();
    let mut trx = db_clone.transaction().await?;
    let selection = UserSelection {
        id: true,
        age: true,
        name: true,
    };
    let entities: Vec<UserSelectedEntity> = trx.search_all(&selection).await?;
    trx.commit().await?;

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
