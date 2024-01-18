use luna_orm::prelude::*;
use luna_orm::LunaOrmResult;

#[derive(Entity, Clone, Debug)]
#[TableName = "user"]
pub struct UserEntity {
    #[PrimaryKey]
    id: i32,
    classes_id: i32,
    name: String,
    age: i32,
}

#[derive(Entity, Clone, Debug)]
#[TableName = "classes"]
pub struct ClassesEntity {
    #[PrimaryKey]
    id: i32,
    name: String,
}

#[derive(Selection, Debug, Default, Clone)]
pub struct UserSelection {
    id: bool,
    classes_id: bool,
    name: bool,
    age: bool,
}

#[derive(Selection, Debug, Default, Clone)]
pub struct ClassesSelection {
    id: bool,
    name: bool,
}

#[derive(Location, Clone, Debug)]
#[TableName = "user"]
#[UniqueIndex = "classes_id, name"]
#[UniqueIndex = "id"]
pub struct UserLocation {
    id: Option<LocationExpr<i32>>,
    classes_id: Option<LocationExpr<i32>>,
    name: Option<LocationExpr<String>>,
    age: Option<LocationExpr<i32>>,
}

#[derive(Location, Clone, Debug)]
#[TableName = "classes"]
pub struct ClassesLocation {
    id: Option<LocationExpr<i32>>,
    name: Option<LocationExpr<String>>,
}

#[derive(SelectedEntity, Debug, Clone, PartialEq, Eq)]
pub struct HelloSelectedEntity {
    id: Option<i32>,
    classes_id: Option<i32>,
    name: Option<String>,
    age: Option<i32>,
}
/*
async fn test_insert(
    db: &mut DB<SqliteDatabase>,
    id: i32,
    classes_id: i32,
    content: &str,
    age: i32,
) -> LunaOrmResult<()> {
    let entity = UserEntity {
        id,
        classes_id,
        name: content.to_string(),
        age,
    };
    let result = db.insert(&entity).await?;
    assert!(result);

    let primary = HelloPrimary { id };
    let selection = HelloSelection {
        id: true,
        content: true,
        age: true,
    };
    let result: Option<HelloSelectedEntity> = db.select(&primary, &selection).await?;
    let selected_entity = HelloSelectedEntity {
        id: Some(id),
        content: Some(content.to_string()),
        age,
    };
    assert_eq!(result, Some(selected_entity));
    Ok(())
}

#[tokio::test]
pub async fn test_database() -> LunaOrmResult<()> {
    let config = SqliteLocalConfig {
        work_dir: "./workspace".to_string(),
        db_file: "test.db".to_string(),
    };
    let mut db: DB<SqliteDatabase> = SqliteDatabase::build(config).await?.into();
    db.execute_plain("DROP TABLE IF EXISTS `article`").await?;
    db.execute_plain(
        "CREATE TABLE IF NOT EXISTS `student`(`id` INT PRIMARY KEY, `classes_id` INT, `age` INT, `name` VARCHAR(64))",
    )
    .await?;

    db.execute_plain(
        "CREATE TABLE IF NOT EXISTS `classes`(`id` INT PRIMARY KEY, `name` VARCHAR(64))",
    )
    .await
    .unwrap();

    test_insert(&mut db, 1, "test", Some(21)).await?;
    test_insert(&mut db, 2, "test", Some(22)).await?;
    test_insert(&mut db, 3, "test", Some(23)).await?;

    let selection = HelloSelection {
        id: true,
        content: true,
        age: true,
    };

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
        .search_paged(&location, Some(&order_by), &selection, &page)
        .await?;
    assert_eq!(result.page.total, 3);
    assert_eq!(result.data.len(), 1);

    return Ok(());
}
*/
