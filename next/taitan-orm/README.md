trait: 从struct中通过宏生成基本的信息，相当于其他语言的反射，只不过是编译态的反射
sql generator: 通过trait反射回来的基本信息，生成必要的SQL语句
sql executor: 把struct转化为arguments, 把row转化为SelectedEntity

sql api: 操作API
\[database\]_commander: 实现sql api
\[database\]_transaction_commander: 实现sql api，但是传入事务，而不是传入连接 