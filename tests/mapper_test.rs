use async_trait::async_trait;
/*
use luna_orm_macro::{Entity, Location, Mutation, Primary, SelectedEntity, Selection};
use luna_orm_trait::merge_any_arguments;
use luna_orm_trait::GenericDaoMapper;
use luna_orm_trait::{
    CmpOperator, Entity, Location, LocationExpr, Mutation, Primary, SelectedEntity, Selection,
};
*/
use luna_orm::prelude::*;
use path_absolutize::*;
use sqlx::any::AnyConnectOptions;
use sqlx::database::{Database, HasArguments};
use sqlx::mysql::{MySqlPool, MySqlRow};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqliteRow};
use sqlx::Connection;
use sqlx::Error;
use sqlx::Pool;
use sqlx::{Any, AnyPool};
use sqlx::{FromRow, IntoArguments};
use std::fs;
use std::marker::PhantomData;
use std::path::{Path, PathBuf};
use std::str::FromStr;

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

pub struct HelloMapperImpl {
    pool: AnyPool,
}

impl HelloMapperImpl {
    pub fn new(pool: AnyPool) -> Self {
        Self { pool }
    }
}

impl GenericDaoMapper for HelloMapperImpl {
    type P = HelloPrimary;
    type E = HelloEntity;
    type S = HelloSelection;
    type SE = HelloSelectedEntity;
    type L = HelloLocation;
    type M = HelloMutation;

    fn get_pool<'a>(&'a self) -> &'a AnyPool {
        return &self.pool;
    }
}

#[tokio::test]
pub async fn test_mapper() {
    let workspace = Path::new("./workspace");
    let workspace_absolute = workspace.absolutize().unwrap();
    fs::create_dir_all(&workspace_absolute).unwrap();
    let db_file_path = workspace_absolute.join("test.db");
    {
        let options = SqliteConnectOptions::new()
            .filename(db_file_path.clone())
            .create_if_missing(true);
        let _ = SqlitePool::connect_with(options).await.unwrap();
    }

    sqlx::any::install_default_drivers();
    let url = format!("sqlite:{}", db_file_path.to_str().unwrap());
    let any_options = AnyConnectOptions::from_str(&url).unwrap();
    let any_pool = AnyPool::connect_with(any_options).await.unwrap();
    let _ = sqlx::query("DROP TABLE IF EXISTS `article`")
        .execute(&any_pool)
        .await;
    let _ = sqlx::query(
        "CREATE TABLE IF NOT EXISTS `article`(`id` INT PRIMARY KEY, `content` VARCHAR(64))",
    )
    .execute(&any_pool)
    .await;

    let mapper = HelloMapperImpl::new(any_pool);

    let entity = HelloEntity {
        id: 23,
        content: "test article".to_string(),
    };
    let success = mapper.try_insert(entity).await.unwrap();
    assert_eq!(success, true);

    let selection = HelloSelection {
        id: None,
        content: Some(true),
    };
    let selected_fields = selection.get_selected_fields();
    assert_eq!(selected_fields, vec!["content".to_string()]);

    let primary = HelloPrimary { id: 23 };
    let result = mapper.try_select(primary.clone(), selection.clone()).await;
    dbg!(&result);
    assert!(result.is_ok());

    let props = HelloSelectedEntity {
        id: None,
        content: Some("test article".to_string()),
    };
    assert_eq!(result.unwrap(), Some(props));

    let result = mapper
        .try_update(HelloEntity {
            id: 23,
            content: "test article2".to_string(),
        })
        .await;
    assert_eq!(result.unwrap(), true);

    let result = mapper.try_select(primary.clone(), selection.clone()).await;
    let props = HelloSelectedEntity {
        id: None,
        content: Some("test article2".to_string()),
    };
    assert_eq!(result.unwrap(), Some(props));

    let upsert_entity = HelloEntity {
        id: 23,
        content: "upsert article".to_string(),
    };
    let result = mapper.try_upsert(upsert_entity).await;
    assert_eq!(result.unwrap(), true);

    let result = mapper.try_select(primary.clone(), selection.clone()).await;
    let props = HelloSelectedEntity {
        id: None,
        content: Some("upsert article".to_string()),
    };
    assert_eq!(result.unwrap(), Some(props));

    let result = mapper.try_delete(primary.clone()).await;
    assert_eq!(result.unwrap(), true);
    let result = mapper.try_select(primary.clone(), selection.clone()).await;
    assert_eq!(result.unwrap(), None);

    let entity1 = HelloEntity {
        id: 1,
        content: "test1".to_string(),
    };
    let entity2 = HelloEntity {
        id: 2,
        content: "test2".to_string(),
    };
    mapper.try_insert(entity1).await.unwrap();
    mapper.try_insert(entity2).await.unwrap();

    let location = HelloLocation {
        id: Some(LocationExpr {
            val: 1,
            cmp: CmpOperator::GreaterOrEq,
        }),
        content: None,
    };
    let entity_list_result = mapper.try_search(location.clone(), selection.clone()).await;
    assert!(entity_list_result.is_ok());
    let list = entity_list_result.unwrap();
    assert_eq!(list.len(), 2);

    let mutation = HelloMutation {
        content: "mutation".to_string(),
    };
    let result = mapper.try_change(location, mutation).await;
    assert_eq!(result.unwrap(), 2);
}
