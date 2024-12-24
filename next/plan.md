
# API设计，把API设计逐渐固定下来
1. unique还没有和primary统一
2. template相关接口还没有
3. template中如何支持Option
4. transaction支持能够更加优雅

# 多数据库支持
sqlite: YES
mysql: No
postgres: No

# 文档

# 正确性测试

# 性能测试


# 额外优化
是否支持Cow<'a, str>作为entity的字符串字段，目前因为查询返回值一定不能用&str
