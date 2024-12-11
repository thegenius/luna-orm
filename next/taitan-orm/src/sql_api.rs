use sqlx::pool::PoolConnection;
use sqlx::Connection;
use std::fmt::Debug;
use tracing::debug;
// use tracing::debug;
use crate::result::Result;
use crate::{SqlExecutor, SqlGenerator};
use taitan_orm_trait::{Entity, Primary, SelectedEntity};
pub trait SqlApi: SqlExecutor + Debug {
    type G: SqlGenerator + Sync + Debug;
    fn get_generator(&self) -> &Self::G;

    // async fn get_connection(&self) -> Result<PoolConnection<Self::DB>>;
    // IntoArguments<'_, <Self as SqlExecutor>::DB>` is not implemented for `SqliteArguments<'_>
    // sqlx的IntoArguments还足够通用
    async fn insert(&mut self, entity: &dyn Entity) -> Result<bool>;

    async fn select<SE>(
        &self,
        primary: &dyn Primary,
        selection: &SE::Selection,
    ) -> Result<Option<SE>>
    where
        SE: SelectedEntity<Self::DB> + Send + Unpin;
}
