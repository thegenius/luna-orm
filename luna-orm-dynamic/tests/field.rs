use luna_orm::prelude::*;
use luna_orm::LunaOrmResult;
use luna_types::constraint::supported::Constraint;
use luna_types::constraint::named::NamedConstraint;
use luna_types::field::supports::integer::Integer;
use luna_types::constraint::supports::integer::IntegerConstraint;
use luna_types::constraint::supports::integer::IntegerConstraintBuilder;
use luna_types::field::valid::ValidField;
use luna_types::field::supported::Field;
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

    let int_constraint: IntegerConstraint<i32> =
        IntegerConstraintBuilder::default()
            .min(10)
            .max(30)
            .build()
            .unwrap()
            .into();
    let int_cons = Constraint::Int(int_constraint);

    let val_id: Field = Field::Int( Integer::<i32>::from(20i32));
    let val_age: Field = Field::Int(Integer::<i32>::from(21i32));
    // let valid_id: ValidField<'_> = ValidField::from_valid(val_id, int_cons.clone()).unwrap();
    // let valid_age: ValidField<'_> = ValidField::from_valid(val_age, int_cons).unwrap();

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
