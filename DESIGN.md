# 整体原理设计
（1）构建struct -> SQL能力  
（2）构建struct -> input(arguments)能力  
（3）构建output(row) -> struct能力

## concept
1. Entity : 包含表的所有字段，表字段应该和Entity Field一一对应
2. Selected & Selection: 字段选择器&被选择字段的dto
3. Mutation: 除主键外的所有字段的Option，表明字段的更新
4. Primary: 主键
5. UniqueLocation: 唯一键
6. Location: Option<LocationExpr<Type>> 是where表达式的等价体


## 所有写入/更新/删除的操作
1. create方法是返回数据库生成的generated 或者 default 字段
如果不包含任何generated字段，返回的就是原始&entity
如果包含generated字段或者default字段，且不是主键，那么就是走一次主键查询
如果主键是auto_increment字段，则要求必须有另外的唯一键，走一次唯一键查询
```
async fn insert(entity) -> Result<bool>                # if not exists -> insert, if exists -> fails   
async fn upsert(entity) -> Result<bool>                # if not exists -> insert, if exists -> update  
async fn create<'a>(&'a entity) -> Result<&'a entity>  # if not exists -> insert, if exists -> fails  

async fn update(mutation, unique)   -> Result<bool> 
async fn change(mutation, location) -> Result<u64>
    
async fn delete(unique)   -> Result<bool>
async fn purify(location) -> Result<u64>
```

## 所有查询操作
1. 计数查询默认返回u64表示记录的行数  
特别地，因为如果使用unique计数，只可能返回0/1，因为常用所以抽象为  
exists -> bool

2. 查询操作的核心设计selection + selected，可以极大地避免dto爆炸问题。
理论上因为selected的所有字段都是option的，所有可以表达所有n个字段的2^n种dto组合  
select: 使用唯一索引查询，返回Option<SE>  
search: 使用条件查询，返回PagedList<SE>      
devour: 遍历所有记录，返回PagedList<SE>

3. 默认遍历执行分页的决策是有争议的，因为有很多高效的分页方案，不会默认执行count，offset也可能更加科学
所以是否暴露底层的不分页查询是一个后续的决策点
search_not_paged  
devour_not_paged
```
async fn exists(unique)  -> Result<bool>
async fn count(location) -> Result<u64>
async fn count_all(name) -> Result<u64>

async fn select<SE>(selection, unique)                                 -> Result<Option<SE>>
async fn search<SE>(selection, location, order_by_option, page_option) -> Result<Vec<SE>>
async fn devour<SE>(selection,           order_by_option, page_option) -> Result<Vec<SE>>

async fn search_paged<SE>(selection, location, order_by, page) -> Result<PagedList<SE>>
async fn devour_paged<SE>(selection,           order_by, page) -> Result<PagedList<SE>>
```

## 所有模板操作
1. 关于写入/唯一键更新操作，通常认为应该不需要再通过模板实现了
2. count可能还存在一些复杂的count需要使用模板
3. devour不需要再通过模板实现了
```
async fn change_by_template(template) -> Result<u64>
async fn purify_by_template(template) -> Result<u64>

async fn count_by_template(template)      -> Result<u64>
async fn select_by_template<SE>(template) -> Result<Option<SE>>
async fn search_by_template<SE>(template, page_option) -> Result<Vec<SE>>
async fn search_paged_by_template<SE>(template, page) -> Result<PagedList<SE>>

async fn procedure_by_template<SE>(template) -> SE
```

## SQL生成器
| SQL          | 语法模板                                                                      | 需要的信息                           |
|--------------|---------------------------------------------------------------------------|---------------------------------|
| insert       | INSERT INTO {table}({fields}) VALUES({})                                  | table + fields                  |
| create       | INSERT INTO {table}({fields}) VALUES({})                                  | table + fields                  |
| upsert       | INSERT INTO {table}({fields}) VALUES({}) ON DUPLICATE UPDATE SET {fields} | table + fields                  |
| update       | UPDATE {table} SET {set_fields} WHERE {loc_fields}                        | table + set_fields + loc_fields |
| change       | UPDATE {table} SET {set_fields} WHERE {loc_fields}                        | table + set_fields + loc_fields |
| delete       | DELETE FROM {table} WHERE {loc_fields}                                    | table + loc_fields              |
| purify       | DELETE FROM {table} WHERE {loc_fields}                                    | table + loc_fields              |
| search_all   | SELECT {sel_fields} FROM {table}                                          | table + sel_fields              |
| select       | SELECT {sel_fields} FROM {table} WHERE {loc_fields}                       | table + sel_fields + loc_fields |
| search       | SELECT {sel_fields} FROM {table} WHERE {loc_fields}                       | table + sel_fields + loc_fields |
| count        | SELECT COUNT(1) FROM {table} WHERE {loc_fields}                           | table + loc_fields              |
| search_paged | SELECT {sel_fields} FROM {table} WHERE {loc_fields}                       | table + sel_fields + loc_fields |


## input arguments生成器
```
<DB> get_insert_arguments(entity) -> Arguments<DB> 
<DB> get_upsert_arguments(entity) -> Arguments<DB>
<DB> get_update_arguments(mutation, primary) - > Arguments<DB> 
<DB> get_change_arguments(mutation, location) -> Arguments<DB> 
<DB> get_primary_arguments(primary)  -> Arguments<DB> 
<DB> get_location_arguments(location, order_by_option) -> Arguments<DB> 
```

## output struct生成器
```
<DB> get_insert_arguments(entity) -> Arguments<DB> 
<DB> get_upsert_arguments(entity) -> Arguments<DB>
<DB> get_update_arguments(mutation, primary) - > Arguments<DB> 
<DB> get_change_arguments(mutation, location) -> Arguments<DB> 
<DB> get_primary_arguments(primary)  -> Arguments<DB> 
<DB> get_location_arguments(location, order_by_option) -> Arguments<DB> 
```

   


    



    



    


## trait + macro
[1] 根据Schema生成所有需要的struct
entity
primary

（1）生成SQL的基本信息
 (2)
（2）struct转化到arguments  
（3）row转化到struct的能力  


## sql generator
从Entity转化成SQL的转化器
（1）insert 语句
（2）upsert 语句





## sql executor
1. trait SqlExecutor
get_pool() : 获取可执行的连接

查询执行函数
fetch_option_plain
fetch_vec_plain

fetch_option
fetch_vec

更新执行函数
execute
execute_plain

存储过程执行函数
execute_procedure

2. Command Executor
SqlExecutor的实现类，负责真是的SQL执行
（1）从Generator获取SQL
（2）把input转化成为arguments
（3）把output转化为struct

3. Transaction Executor
同Command Executor，同时负责好begin + commit





