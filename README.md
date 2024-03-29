
# :warning: LUNA-ORM is under rapid development, api may change, you should not use it in PRODUCTION env.
For now, just have a basic taste, waiting for the version 1.0  
At that time, the api will be stable, and backward compatible will be promised.  

  
![Building](https://github.com/thegenius/luna-orm/actions/workflows/rust.yml/badge.svg)
[![Version](https://img.shields.io/badge/crates-0.3.6-green)](https://crates.io/crates/luna-orm)

# LUNA-ORM

**LUNA-ORM** is an async orm framework based on SQLx. Built with :heart:
-  **Intuitive** : Simple API, the most simple orm in this world.
-  **Time Saving** : Most useful API is implemented by default, no need to waste your life.
-  **Smooth Transaction** : Transaction is almost same as normal.
-  **Template SQL** : You can execute your own sql with no pain.
-  **Dynamic Parameters** : Handle complex dynamic sql with default.
-  **Truly Asynchronous** : Based on SQLx, luna-orm is fully async.
-  **Error Soundly** : Every error has its meaning. 

## ROADMAP
- **0.1 API Skeleton** :white_check_mark:
- **0.2 Transaction** transaction support :white_check_mark:
- **0.3 Static Template** static tempalte sql and tracing  :white_check_mark:
- **0.4 Relationship**: support relationship :hammer: (I'm working hard on this, and finally maybe something like the core part of GraphQL)
- **0.5 Dynamic Template**: support dynamic template :pushpin:
- **0.6 Error**: :pushpin:
- **0.7 Performance**: benchmark and optimize :pushpin:
- **0.7 Correctness**: code coverage and mocking :pushpin:
- **0.9 Stablization and Doc**: stablize the api, macro and error :pushpin:
- **1.0 Fisrt stable version**: :pushpin:
- **2.0 Ecosystem**: :paperclip:

## INSTALL
```toml
luna-orm = { version = "0.3.6" }
  
```

## Intuitive
Everything should just works as you want.


### Create a database instance.
```rust
use luna_orm::prelude::*;
use luna_orm::LunaOrmResult;

#[tokio::main]
pub async fn main() -> LunaOrmResult<()> {

  // 1. example use sqlite with local file mode
  let config = SqliteLocalConfig::new("./workspace", "test.db");

  // 2. create a DB instance.
  let mut db: DB<SqliteDatabase> = SqliteDatabase::build(config).await.unwrap().into();

  // optional: you may need to create the table for the first time.
  // db.execute_plain(
  //      "CREATE TABLE IF NOT EXISTS `user`(`id` INT PRIMARY KEY, `age` INT, `name` VARCHAR(64))",
  // )
  
  Ok(())
}
 
```
### Insert an entity

```rust
// 1. Declare an Entity, Derive macro Entity, give a TableName
#[derive(Entity, Clone, Debug)]
#[TableName = "user"]
pub struct HelloEntity {
    #[PrimaryKey]
    id: i32,
    name: String,
    age: Option<i32>,
}


// 2. create an entity.
let entity = HelloEntity {
      id: 1,
      name: "Allen".to_string(),
      age: Some(23)
}; 
// 3. insert it, this is so intuitive, you don't need to warry about anything, it jsut works.
let result = db.insert(&entity).await?;
```

## Time Saving
Almost 90% command has been implemented by default, this may saving your time.i

## Concept
![](https://github.com/thegenius/luna-orm/blob/main/docs/concept.png)

### If you want to insert
```rust
  // insert an entity if not exists.
  async fn insert(&mut self, entity: &dyn Entity) -> LunaOrmResult<bool>;
  
  // insert is not exists, and update if already exists.
  async fn upsert(&mut self, entity: &dyn Entity) -> LunaOrmResult<bool>;

  // insert an entity if not exists, and return the inserted entity.
  async fn create<'a>(&mut self, entity: &'a dyn Entity) -> LunaOrmResult<&'a dyn Entity>;
```
### If you want to update
```rust
  // update one record by primary
  async fn update(&mut self, mutation: &dyn Mutation, primary: &dyn Primary) -> LunaOrmResult<bool> 

  // update many records by location
  async fn change(&mut self,mutation: &dyn Mutation, location: &dyn Location) -> LunaOrmResult<usize>;
  
```


### If you want to delete
```rust

  // delete one record by primary
  async fn delete(&mut self, primary: &dyn Primary) -> LunaOrmResult<bool>;

  // delete many records by location
  async fn purify(&mut self, location: &dyn Location) -> LunaOrmResult<usize>;
  
```

### If you want to select
```rust
  // fetch one entity by primary and select fields by selection 
  async fn select<SE>(
    &mut self, 
    primary: &dyn Primary, 
    selection: &dyn Selection
  ) -> LunaOrmResult<Option<SE>>
     where
        SE: SelectedEntity + Send + Unpin; 

  // fetch many entity by location&order and select fields by selection
   async fn search<SE>(
        &mut self,
        location: &dyn Location,
        order_by: Option<&dyn OrderBy>,
        selection: &dyn Selection,
    ) -> LunaOrmResult<Vec<SE>>
    where
        SE: SelectedEntity + Send + Unpin;

  // fetch paged entity with pagination
   async fn search_paged<SE>(
        &mut self,
        location: &dyn Location,
        order_by: Option<&dyn OrderBy>,
        selection: &dyn Selection,
        page: &Pagination,
    ) -> LunaOrmResult<PagedList<SE>>
    where
        SE: SelectedEntity + Send + Unpin;
        
```




## Smooth Transaction
```rust
  let db: DB<SqliteDatabase> = SqliteDatabase::build(config).await.unwrap().into();
  // 1. start a transaction by the simple async api.
  let mut trx = db.transaction().await.unwrap();

  // 2. just do every thing you want,
  // every command is just same as normal.  
  trx.insert(...).await?;
  trx.select(...).await?;
  trx.delete(...).await?;

  // 3. last thing is just commit, if you forget, trx will rollback by default. 
  trx.commit().await?;
  
```

## Template SQL
```rust
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

let template = HelloTemplate {
    id: 2,
    content: "template".to_string(),
};
// template just works as you want, #{} is the variable 
db.execute_by_template(&template).await?;


// if you want to execute paged template,
// you should give a TemplateCountSql, the `as count` is important.
let select_template = HelloSelectTemplate { id: 0 };
let page = Pagination {
        page_size: 1,
        page_num: 1,
};
let result: PagedList<HelloSelectedEntity> =
    db.search_paged_by_template(&select_template, &page).await?;


  
```

## Dynamic Parameters

## Truly Asynchronous

## Error Sound


## MSRV: Minimum Supported Rust Version
1.75.0  
LUNA-ORM use async trait.

## Safety
This lib uses #![forbid(unsafe_code)]

## LICENSE
Apache 2.0
