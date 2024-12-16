pub enum CountSql {
    Empty,
    PlainSql(String),
    VariabledSql(String),
}
