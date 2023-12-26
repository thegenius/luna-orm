use luna_orm::prelude::*;
use luna_orm::LunaOrmResult;

#[tokio::main]
async fn main() -> LunaOrmResult<()> {
    // 2. create a DB instance.
    let mut db: DB<MysqlDatabase> = MysqlDatabase::build("localhost/test", "user", "passwod")
        .await?
        .into();

    // 3. optional: you may need to create the table for the first time.
    let result = db.execute_plain(
        "CREATE TABLE IF NOT EXISTS `user`(`id` INT PRIMARY KEY, `age` INT, `name` VARCHAR(64))",
    ).await?;

    println!("result: {}", result.rows_affected());
    Ok(())
}
