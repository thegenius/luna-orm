use luna_orm::prelude::*;
use luna_orm::LunaOrmResult;

#[derive(Selection, Default, Clone)]
pub struct HelloSelection {
    id: Option<bool>,
    content: Option<bool>,
}

#[derive(OrderBy, Clone)]
pub enum HelloOrderBy {
    Id,
    IdContent,
    ContentId,
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
#[UniqueIndex = "id, content"]
#[UniqueIndex = "id"]
pub struct HelloLocation {
    id: Option<LocationExpr<i32>>,
    content: Option<LocationExpr<String>>,
}

#[tokio::test]
pub async fn test_database() -> LunaOrmResult<()> {
    let config = SqliteLocalConfig {
        work_dir: "./workspace".to_string(),
        db_file: "test.db".to_string(),
    };
    let mut db: SqliteDatabase = SqliteDatabase::build(config).await.unwrap();
    db.execute_plain("DROP TABLE IF EXISTS `article`")
        .await
        .unwrap();
    db.execute_plain(
        "CREATE TABLE IF NOT EXISTS `article`(`id` INT PRIMARY KEY, `content` VARCHAR(64))",
    )
    .await
    .unwrap();

    let entity = HelloEntity {
        id: 23,
        content: "test".to_string(),
    };
    let result = db.insert(&entity).await?;
    assert!(result);

    let primary = HelloPrimary { id: 23 };
    let selection = HelloSelection {
        id: None,
        content: Some(true),
    };
    let result: Option<HelloSelectedEntity> = db.select(&primary, &selection).await?;
    let selected_entity = HelloSelectedEntity {
        id: None,
        content: Some("test".to_string()),
    };
    assert_eq!(result, Some(selected_entity));

    let entity = HelloEntity {
        id: 24,
        content: "test2".to_string(),
    };
    let result = db.insert(&entity).await?;
    assert!(result);

    let entity = HelloEntity {
        id: 25,
        content: "test3".to_string(),
    };
    let result = db.insert(&entity).await?;
    assert!(result);

    let location: HelloLocation = HelloLocation {
        id: Some(LocationExpr::new(CmpOperator::GreaterThan, 0)),
        content: None,
    };
    let page = Pagination {
        page_size: 1,
        page_num: 0,
    };
    let order_by = HelloOrderBy::ContentId;
    let result: PagedList<HelloSelectedEntity> = db
        .search_paged(&location, &order_by, &selection, &page)
        .await?;
    assert_eq!(result.page.total, 3);
    assert_eq!(result.data.len(), 1);

    return Ok(());
}
