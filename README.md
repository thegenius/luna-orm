# LUNA-ORM

LUNA-ORM is an async orm framework based on SQLx.
- ** Intuitive ** : Elegant API, the most simple orm in this world.
- ** Saving Lives ** : Most useful api is default, no need to waste your life.
- ** Smooth Transaction ** : Transaction is almost same as normal.
- ** Template SQL ** : You can execute your own sql with no pain.
- ** Dynamic Parameters ** : Handle complex dynamic sql with default.
- ** Truly Asynchronous ** : Based on SQLx, luna-orm is fully async.
- ** Error Sound ** : Every error has its meaning. 

## INSTALL
```toml
luna-orm = { version = "0.3" }
  
```

## Intuitive
Everything should just works as you want.

```rust
use luna_orm::preclude::*;

#[derive(Entity, Clone, Debug)]
#[TableName = "user"]
pub struct HelloEntity {
    #[PrimaryKey]
    id: i32,
    name: String,
    age: Option<i32>,
}

#[tokio::main]
pub async fn main() -> LunaOrmResult<()> {

  // 1. example use sqlite with local file mode
  let config = SqliteLocalConfig {
        work_dir: "./workspace".to_string(),
        db_file: "test.db".to_string(),
    };

  // optional: you may need to create the table for the first time.
  // db.execute_plain(
  //      "CREATE TABLE IF NOT EXISTS `article`(`id` INT PRIMARY KEY, `age` INT, `name` VARCHAR(64))",
  // )
  
  // 2. create a DB instance.
  let mut db: DB<SqliteDatabase> = SqliteDatabase::build(config).await.unwrap().into();

  // 3. create an entity and juse insert it, that's it!
  let entity = HelloEntity {
      id: 1,
      name: "Allen".to_string(),
      age: Some(23)
  }; 
  let result = db.insert(entity).await?;
  
  Ok(())
}

  
```



## Saving Lives
Almost 90% command has been implemented by default, this may saving your lives.

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

