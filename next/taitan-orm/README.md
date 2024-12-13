trait: 从struct中通过宏生成基本的信息，相当于其他语言的反射，只不过是编译态的反射
sql generator: 通过trait反射回来的基本信息，生成必要的SQL语句
sql executor: 把struct转化为arguments, 把row转化为SelectedEntity

sql api: 操作API
\[database\]_commander: 实现sql api
\[database\]_transaction_commander: 实现sql api，但是传入事务，而不是传入连接 


## API Design

```rust
async fn insert(entity: &dyn Entity) -> Result<bool>;
async fn upsert(entity: &dyn Entity) -> Result<bool>;
async fn update<M: Mutation>(mutation: &M, unique: &M::Primary) -> Result<bool>;
async fn change<M: Mutation>(mutation: &M, location: &M::Location) -> Result<bool>;
async fn delete(unique: &dyn Unique) -> Result<bool>;
async fn purify(location: &dyn Location) -> Result<usize>;
```

```rust
async fn select<SE>(
    selection: &SE::Selection, 
    unique: &dyn Unique
) -> Result<Option<SE>>;

async fn search<SE>(
    selection: &SE::Selection, 
    location: &dyn Location, 
    order_by: &dyn OrderBy
) -> Result<Vec<SE>>;

async fn search_paged<SE>(
    selection: &SE::Selection,
    location: &dyn Location,
    order_by: &dyn OrderBy,
    page: &Pagination,
) -> Result<PagedList<Self::DB, SE>>;

async fn devour<SE>(
    selection: &SE::Selection, 
    order_by: &dyn OrderBy
) -> Result<Vec<SE>>;

async fn devour_paged<SE>(
    selection: &SE::Selection, 
    order_by: &dyn OrderBy, 
    page: &Pagination
) -> Result<PagedList<Self::DB, SE>>;

async fn count(location: &dyn Location) -> Result<u64>;

async fn count_table(table_name: &str) -> Result<u64>;

```