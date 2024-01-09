use luna_orm::prelude::*;
use luna_orm::LunaOrmResult;

pub async fn setup_database() -> LunaOrmResult<DB<SqliteDatabase>> {
    let config = SqliteLocalConfig {
        work_dir: "./workspace".to_string(),
        db_file: "test.db".to_string(),
    };
    let db: SqliteDatabase = SqliteDatabase::build(config).await.unwrap();
    let mut db: DB<SqliteDatabase> = DB(db);
    db.execute_plain("DROP TABLE IF EXISTS `article`")
        .await
        .unwrap();
    db.execute_plain(
        "CREATE TABLE IF NOT EXISTS `article`(`id` INT PRIMARY KEY, `age` INT, `content` VARCHAR(64))",
    ).await?;
    Ok(db)
}

pub async fn create_table<T>(
    db: &mut DB<T>,
    table_name: &str,
    create_table_stmt: &str,
) -> LunaOrmResult<bool>
where
    T: Database,
{
    let drop_stmt = format!("DROP TABLE IF EXISTS `{}`", table_name);
    db.execute_plain(&drop_stmt).await?;
    db.execute_plain(create_table_stmt).await?;
    Ok(true)
}
