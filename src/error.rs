use thiserror::Error;

#[derive(Error, Debug)]
pub enum LunaOrmError {
    #[error("databse init fail with args: `{0}`")]
    DatabaseInitFail(String),

    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),

    #[error("method not implement error")]
    NotImplement,
}
