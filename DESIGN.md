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


## 所有希望支持的操作
```
async fn insert(entity) -> primary  # if not exists -> insert, if exists -> fails   
async fn create(entity) -> primary  # if not exists -> insert, if exists -> return primary  
async fn upsert(entity) -> primary  # if not exists -> insert, if exists -> update  

async fn update(mutation, primary) -> bool 
async fn change(mutation, location) -> usize
    
async fn delete(primary) -> bool
async fn purify(location) -> usize

async fn search_all<SE>(selection) -> Vec<SE>
async fn select<SE>(selection, primary) -> Option<SE>  
async fn search<SE>(selection, location, order_by_option) -> Vec<SE>
async fn count(location) -> usize
async fn search_paged<SE>(selection, location, page, order_by_option) -> PagedList<SE>
    
async fn execute_by_template(template) -> usize
async fn select_by_template<SE>(template) -> Option<SE>
async fn search_by_template<SE>(template) -> Vec<SE>
async fn search_paged_by_template<SE>(template, page) -> PagedList<SE>
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





