use std::fmt::Debug;
// use tracing::debug;
use taitan_orm_trait::{ Primary, SelectedEntity};
use crate::{SqlExecutor, SqlGenerator};
use crate::result::Result;
pub trait SqlApi: SqlExecutor + Debug {
    type G: SqlGenerator + Sync + Debug;
    fn get_generator(&self) -> &Self::G;

    async fn select<SE>(
        &self,
        primary: &dyn Primary,
        selection: &SE::Selection,
    ) -> Result<Option<SE>>
    where
        SE: SelectedEntity<Self::DB> + Send + Unpin;

    // async fn insert(&mut self, entity: &dyn Entity) -> Result<bool> {
    //     debug!(target: "luna_orm", command = "insert",  entity = ?entity);
    //     let sql = self.get_generator().get_insert_sql(entity);
    //     debug!(target: "luna_orm", command = "insert", sql = sql);
    //     let args = entity.any_arguments_of_insert();
    //     let result = self.execute(&sql, args).await?;
    //     debug!(target: "luna_orm", command = "insert", result = ?result);
    //     return Ok(result.rows_affected() > 0);
    // }
}
