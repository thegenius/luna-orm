use crate::error::LunaOrmError;
use crate::sql_executor::SqlExecutor;
use crate::sql_generator::SqlGenerator;
use crate::LunaOrmResult;

use luna_orm_macro::timed;
use luna_orm_trait::*;
use std::fmt::Debug;
use sqlx::{Database, Sqlite};
use sqlx::sqlite::SqliteArguments;
use tracing::{debug, instrument};
use luna_orm_trait::schema_trait::{SchemaNew, SelectedEntityNew};
use crate::sql_executor2::SqlExecutorNew;

pub trait CommandExecutorNew: SqlExecutorNew + Debug {

    type DB: Database;
    type G: SqlGenerator + Sync + Debug;

    fn get_generator(&self) -> &Self::G;

    async fn select<SE>(
        &mut self,
        primary: &dyn Primary,
        selection: &dyn Selection,
    ) -> LunaOrmResult<Option<SE>>
    where
        SE: SelectedEntityNew<Self::DB> + Send + Unpin,
    {
        let pool = self.new_get_pool()?;
        let mut conn = pool.acquire().await?;
        debug!(target: "luna_orm", command = "select", primary = ?primary, selection = ?selection);
        let sql = self.get_generator().get_select_sql(selection, primary);
        debug!(target: "luna_orm", command = "select", sql = sql);
        let args = primary.any_arguments();
        let args: SqliteArguments = <User as SchemaNew<Sqlite>>::gen_insert_arguments(user).unwrap();

        let result: Option<SE> = self.new_fetch_optional(&mut *conn, &sql, args).await?;
        debug!(target: "luna_orm", command = "select", result = ?result);
        return Ok(result);
    }

}
