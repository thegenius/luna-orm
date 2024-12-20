use thiserror::Error;

#[derive(Error, Debug)]
pub enum TaitanOrmError {
    #[error("databse init fail with args: `{0}`")]
    DatabaseInitFail(String),

    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),

    #[error(transparent)]
    BoxDynError(#[from] Box<dyn std::error::Error + 'static + Send + Sync>),

    #[error(transparent)]
    BoxDynError2(#[from] Box<dyn std::error::Error>),

    #[error("deserialize entity from row  error")]
    FromRowToEntityError,

    #[error("invalid order by fields")]
    OrderByFieldsError,

    #[error("method not implement error: {0}")]
    NotImplement(String),

    #[error("paged template sql can't execute with no count sql")]
    PagedTemplateHasNoCountSql,

    #[error("dynamic request parse error: {0}")]
    DynamicRequestParseError(String),
}
