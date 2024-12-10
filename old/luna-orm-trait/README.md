## entity
| type | auto-increment | nullable      | default   |is option|
|------|----------------|---------------|-----------|-|
| 主键   | N              | must not null | not allow |✗|
| 主键   | Y              | must not null | not allow |✓|
| 唯一键  | not allow      | must not null | Y         |✓|
| 唯一键  | not allow      | must not null | N         |✗|
| 非键   | Y              | -             | -         |✓|
| 非键   | -              | -             | Y         |✓|
| 非键   | -              | Y             | -         |✓|
| 非键   | N              | N             | N         |✗|

## 表结构主信息
\#[TableName]  
\#[FieldName]

## 索引信息宏
\#[PrimaryKey]
\#[UniqueKey]
\#[Index]

## input转化层
1. insert(entity)：所有非空字段都应该转化到arguments里面  
2. upsert(entity)：所有非空字段都转化，非primary主键的部分需要转2遍用来 on duplicate update set
3. update(mutation, primary): 所有mutation的