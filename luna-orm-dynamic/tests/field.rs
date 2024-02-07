use luna_orm::prelude::*;
use luna_orm::LunaOrmResult;
use luna_types::CachedConstraint;
use luna_types::Integer;
use luna_types::IntegerConstraint;
use luna_types::IntegerConstraintBuilder;
use luna_types::ValidField;
use sqlx::sqlx_macros;

#[sqlx_macros::test]
async fn test_insert_field() {
    let config = SqliteLocalConfig::new("./workspace", "test.db");
    let mut db: SqliteDatabase = SqliteDatabase::build(config).await.unwrap();
    let mut db: DB<SqliteDatabase> = DB(db);
    db.execute_plain("DROP TABLE IF EXISTS `article`")
        .await
        .unwrap();
    db.execute_plain(
        "CREATE TABLE IF NOT EXISTS `article`(`id` INT PRIMARY KEY, `age` INT, `content` VARCHAR(64))",
    )
    .await
    .unwrap();

    let int_constraint: CachedConstraint<IntegerConstraint<i32>> =
        IntegerConstraintBuilder::default()
            .min(10)
            .max(30)
            .build()
            .unwrap()
            .into();
    let val_id: Integer<i32> = Integer::<i32>::from_valid(20i32, &int_constraint).unwrap();
    let val_age: Integer<i32> = Integer::<i32>::from_valid(21i32, &int_constraint).unwrap();
    let mut args: AnyArguments = AnyArguments::default();
    luna_add_arg(&mut args, &val_id);
    luna_add_arg(&mut args, &val_age);

    let result = db
        .execute("insert into `article`(`id`, `age`) values(?, ?)", args)
        .await;
    let row: JsonResult = db
        .fetch_one_plain("select id, age from article")
        .await
        .unwrap();
    assert_eq!(r#"{"age":21,"id":20}"#, row.data);
}
