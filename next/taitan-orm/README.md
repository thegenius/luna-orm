trait: 从struct中通过宏生成基本的信息，相当于其他语言的反射，只不过是编译态的反射
sql generator: 通过trait反射回来的基本信息，生成必要的SQL语句
sql executor: 把struct转化为arguments, 把row转化为SelectedEntity

sql api: 操作API
\[database\]_commander: 实现sql api
\[database\]_transaction_commander: 实现sql api，但是传入事务，而不是传入连接 


## API Design

```rust
async fn insert(&mut self, entity: &dyn Entity) -> Result<bool>;
async fn upsert(&mut self, entity: &dyn Entity) -> Result<bool>;
async fn update<M: Mutation>(&mut self, mutation: &M, unique: &M::Primary) -> Result<bool>;
async fn change<M: Mutation>(&mut self, mutation: &M, location: &M::Location) -> Result<bool>;
async fn delete(&mut self, unique: &dyn Unique) -> Result<bool>;
async fn purify(&mut self, location: &dyn Location) -> Result<usize>;
```