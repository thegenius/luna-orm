use thiserror::Error;

#[derive(Error, Debug)]
pub enum LunaOrmError {
    #[error("databse init fail with args: `{0}`")]
    DatabaseInitFail(String),

    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),

    #[error(transparent)]
    BoxDynError(#[from] Box<dyn std::error::Error + 'static + Send + Sync>),

    #[error("deserialize entity from row  error")]
    FromRowToEntityError,

    #[error("invalid order by fields")]
    OrderByFieldsError,

    #[error("method not implement error")]
    NotImplement,

    #[error("paged template sql can't execute with no count sql")]
    PagedTemplateHasNoCountSql,

    #[error("dynamic request parse error: {0}")]
    DynamicRequestParseError(String),
}
