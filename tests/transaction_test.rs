use luna_orm::prelude::*;
use luna_orm::LunaOrmResult;

#[derive(Selection, Default, Clone)]
pub struct HelloSelection {
    id: Option<bool>,
    content: Option<bool>,
}

#[derive(Primary, Default, Clone)]
#[TableName = "article"]
pub struct HelloPrimary {
    id: i32,
}

#[derive(SelectedEntity, Debug, Clone, PartialEq, Eq)]
pub struct HelloSelectedEntity {
    id: Option<i32>,
    content: Option<String>,
}

#[derive(Entity, Clone, Debug)]
#[TableName = "article"]
pub struct HelloEntity {
    #[PrimaryKey]
    id: i32,
    content: String,
}

#[derive(Mutation, Clone, Debug)]
pub struct HelloMutation {
    content: String,
}

#[derive(Location, Clone, Debug)]
#[TableName = "article"]
pub struct HelloLocation {
    id: Option<LocationExpr<i32>>,
    content: Option<LocationExpr<String>>,
}

async fn build_db() -> Database {
    let config = SqliteLocalConfig {
        work_dir: "./workspace".to_string(),
        db_file: "test.db".to_string(),
    };
    let config = DatabaseConfig::SqliteLocal(config);
    let db = Database::build(config).await.unwrap();
    let mut trx = db.transaction().await.unwrap();
    trx.query("DROP TABLE IF EXISTS `article`").await.unwrap();
    trx.query("CREATE TABLE IF NOT EXISTS `article`(`id` INT PRIMARY KEY, `content` VARCHAR(64))")
        .await
        .unwrap();
    trx.query("DELETE FROM `article`").await.unwrap();
    trx.commit().await.unwrap();
    return db;
}

#[tokio::test]
pub async fn test_transaction() -> LunaOrmResult<()> {
    let db = build_db().await;
    let mut trx = db.transaction().await?;

    let entity = HelloEntity {
        id: 23,
        content: "test".to_string(),
    };
    let result = trx.insert(entity).await?;
    assert_eq!(result, true);

    let primary = HelloPrimary { id: 23 };
    let selection = HelloSelection {
        id: None,
        content: Some(true),
    };
    trx.commit().await?;

    let mut trx = db.transaction().await?;

    let result: Option<HelloSelectedEntity> = trx.select(primary, selection).await?;
    let selected_entity = HelloSelectedEntity {
        id: None,
        content: Some("test".to_string()),
    };
    assert_eq!(result, Some(selected_entity));

    return Ok(());
}

#[tokio::test]
pub async fn test_transaction_rollback() -> LunaOrmResult<()> {
    let db = build_db().await;
    let mut trx = db.transaction().await?;

    let entity = HelloEntity {
        id: 23,
        content: "test".to_string(),
    };
    let result = trx.insert(entity).await?;
    assert_eq!(result, true);

    let primary = HelloPrimary { id: 23 };
    let selection = HelloSelection {
        id: None,
        content: Some(true),
    };
    trx.rollback().await?;

    let mut trx = db.transaction().await?;
    let result: Option<HelloSelectedEntity> = trx.select(primary, selection).await?;
    let selected_entity = HelloSelectedEntity {
        id: None,
        content: Some("test".to_string()),
    };
    assert_eq!(result, None);

    return Ok(());
}

async fn expect_rollback_transaction<'a>(mut trx: Transaction<'a>) -> LunaOrmResult<()> {
    let entity = HelloEntity {
        id: 23,
        content: "test".to_string(),
    };
    let result = trx.insert(entity).await?;
    assert_eq!(result, true);
    return Ok(());
}

async fn expect_commit_transaction<'a>(mut trx: Transaction<'a>) -> LunaOrmResult<()> {
    let entity = HelloEntity {
        id: 23,
        content: "test".to_string(),
    };
    let result = trx.insert(entity).await?;
    assert_eq!(result, true);
    trx.commit().await?;
    return Ok(());
}

#[tokio::test]
pub async fn test_transaction_rollback2() -> LunaOrmResult<()> {
    let db = build_db().await;
    let mut trx = db.transaction().await?;
    let _ = expect_rollback_transaction(trx).await;

    let primary = HelloPrimary { id: 23 };
    let selection = HelloSelection {
        id: None,
        content: Some(true),
    };
    let result: Option<HelloSelectedEntity> = db.select(primary, selection).await?;
    assert_eq!(result, None);

    return Ok(());
}

#[tokio::test]
pub async fn test_transaction_commit() -> LunaOrmResult<()> {
    let db = build_db().await;
    let mut trx = db.transaction().await?;
    let _ = expect_commit_transaction(trx).await;

    let primary = HelloPrimary { id: 23 };
    let selection = HelloSelection {
        id: None,
        content: Some(true),
    };
    let result: Option<HelloSelectedEntity> = db.select(primary, selection).await?;

    let selected_entity = HelloSelectedEntity {
        id: None,
        content: Some("test".to_string()),
    };
    assert_eq!(result, Some(selected_entity));

    return Ok(());
}
