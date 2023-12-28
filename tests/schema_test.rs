use luna_orm::prelude::*;
use luna_orm::LunaOrmResult;

#[derive(OrderBy, Clone)]
pub enum HelloOrderBy {
    Id,
    IdContent,
    ContentId,
}

#[derive(Schema, Clone, Debug)]
#[TableName = "article"]
#[UniqueIndex = "id, content"]
#[UniqueIndex = "id"]
pub struct HelloEntity {
    #[PrimaryKey]
    id: i32,
    content: String,
    age: Option<i32>,
}

/*
#[derive(Location, Clone, Debug)]
#[TableName = "article"]
#[UniqueIndex = "id, content"]
#[UniqueIndex = "id"]
pub struct HelloLocation {
    id: Option<LocationExpr<i32>>,
    content: Option<LocationExpr<String>>,
}
*/

#[derive(TemplateRecord)]
#[TemplateSql = "update article set content = #{content} where id = #{id}"]
pub struct HelloTemplate {
    id: i32,
    content: String,
}

#[derive(TemplateRecord)]
#[TemplateSql = "select * FROM article where id > #{id}"]
#[TemplateCountSql = "select count(*) as count FROM article where id > #{id}"]
pub struct HelloSelectTemplate {
    id: i32,
}

async fn test_insert(
    db: &mut DB<SqliteDatabase>,
    id: i32,
    content: &str,
    age: Option<i32>,
) -> LunaOrmResult<()> {
    let entity = HelloEntity {
        id,
        content: content.to_string(),
        age,
    };
    let result = db.insert(&entity).await?;
    assert!(result);

    let primary = ArticlePrimary { id };
    //let primary2 = TestttPrimary { id };
    let selection = ArticleSelection {
        id: true,
        content: true,
        age: true,
    };
    let result: Option<ArticleSelectedEntity> = db.select(&primary, &selection).await?;
    let selected_entity = ArticleSelectedEntity {
        id: Some(id),
        content: Some(content.to_string()),
        age,
    };
    assert_eq!(result, Some(selected_entity));
    Ok(())
}

async fn test_upsert(
    db: &mut DB<SqliteDatabase>,
    id: i32,
    content: &str,
    age: Option<i32>,
) -> LunaOrmResult<()> {
    let entity = HelloEntity {
        id,
        content: content.to_string(),
        age,
    };
    let result = db.upsert(&entity).await?;
    assert!(result);

    let primary = ArticlePrimary { id };
    let selection = ArticleSelection {
        id: true,
        content: true,
        age: true,
    };
    let result: Option<ArticleSelectedEntity> = db.select(&primary, &selection).await?;
    let selected_entity = ArticleSelectedEntity {
        id: Some(id),
        content: Some(content.to_string()),
        age,
    };
    assert_eq!(result, Some(selected_entity));
    Ok(())
}

async fn test_update(
    db: &mut DB<SqliteDatabase>,
    id: i32,
    content: &str,
    age: i32,
) -> LunaOrmResult<()> {
    let entity = HelloEntity {
        id,
        content: content.to_string(),
        age: Some(age),
    };
    let result = db.upsert(&entity).await?;
    assert!(result);
    let mutation = ArticleMutation {
        content: None,
        age: Some(age + 1),
    };

    let primary = ArticlePrimary { id };
    let result = db.update(&mutation, &primary).await?;
    assert!(result);

    let selection = ArticleSelection {
        id: true,
        content: true,
        age: true,
    };
    let result: Option<ArticleSelectedEntity> = db.select(&primary, &selection).await?;
    let selected_entity = ArticleSelectedEntity {
        id: Some(id),
        content: Some(content.to_string()),
        age: Some(age + 1),
    };
    assert_eq!(result, Some(selected_entity));
    Ok(())
}

async fn test_execute_template(db: &mut DB<SqliteDatabase>) -> LunaOrmResult<()> {
    test_upsert(db, 2, "test", Some(22)).await?;
    let template = HelloTemplate {
        id: 2,
        content: "template".to_string(),
    };
    db.execute_by_template(&template).await?;

    let select_template = HelloSelectTemplate { id: 0 };
    let page = Pagination {
        page_size: 1,
        page_num: 1,
    };
    let result: PagedList<ArticleSelectedEntity> =
        db.search_paged_by_template(&select_template, &page).await?;
    assert_eq!(result.page.total, 3);
    let result_list = result.data;
    let selected_entity = ArticleSelectedEntity {
        id: Some(2),
        content: Some("template".to_string()),
        age: Some(22),
    };
    assert_eq!(result_list, vec![selected_entity]);

    Ok(())
}

#[tokio::test]
pub async fn test_database() -> LunaOrmResult<()> {
    let config = SqliteLocalConfig {
        work_dir: "./workspace".to_string(),
        db_file: "test.db".to_string(),
    };
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
    test_insert(&mut db, 1, "test", Some(21)).await?;
    test_upsert(&mut db, 2, "test", Some(22)).await?;
    test_insert(&mut db, 3, "test", Some(23)).await?;

    let selection = ArticleSelection {
        id: true,
        content: true,
        age: true,
    };

    let location: ArticleLocation = ArticleLocation {
        id: Some(LocationExpr::new(CmpOperator::GreaterThan, 0)),
        content: None,
        age: None,
    };
    let page = Pagination {
        page_size: 1,
        page_num: 0,
    };
    let order_by = HelloOrderBy::ContentId;
    let result: PagedList<ArticleSelectedEntity> = db
        .search_paged(&location, Some(&order_by), &selection, &page)
        .await?;
    assert_eq!(result.page.total, 3);
    assert_eq!(result.data.len(), 1);

    test_execute_template(&mut db).await?;

    test_update(&mut db, 4, "test", 24).await?;
    test_update(&mut db, 5, "test", 25).await?;
    return Ok(());
}
