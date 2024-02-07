use luna_orm::prelude::*;
use luna_orm::LunaOrmResult;
use luna_types::CachedConstraint;
use luna_types::Integer;
use luna_types::IntegerConstraint;
use luna_types::IntegerConstraintBuilder;
use luna_types::ValidField;
use luna_types::{ConstraintType, Record, RecordConstraint};
use serde_json::Value;
use sqlx::sqlx_macros;

#[sqlx_macros::test]
async fn test_insert_record() {
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

    let json_str = r#"{"age":21,"id":20}"#;
    let json_value: Value = serde_json::from_str(json_str).unwrap();

    let constraint_str = r#"[ 
    { "type": "smallint", "name": "id", "constraint": { "min": 10, "max": 20 } },
    { "type": "smallint", "name": "age", "constraint": { "min": 10, "max": 30 } } ]"#;
    let constraint: RecordConstraint = serde_json::from_str(constraint_str).unwrap();
    dbg!(&constraint);

    let record: Record = Record::from_json(&json_value, &constraint).unwrap();
    dbg!(&record);
    let args = record.into_any_arguments();

    let result = db
        .execute("insert into `article`(`id`, `age`) values(?, ?)", args)
        .await;
    let row: JsonResult = db
        .fetch_one_plain("select id, age from article")
        .await
        .unwrap();
    assert_eq!(r#"{"age":21,"id":20}"#, row.data);
}
