use crate::sql_executor2::SqlExecutorNew;
use crate::sql_generator2::SqlGenerator;

use crate::LunaOrmResult;
use luna_orm_trait::schema_trait::{
    EntityNew, LocationNew, MutationNew, PrimaryNew, SelectedEntityNew, UpdateCommand,
};
use sqlx::error::BoxDynError;
use sqlx::Database;
use std::fmt::Debug;

pub trait CommandExecutorNew: SqlExecutorNew + Debug {
    type G: SqlGenerator + Sync + Debug;

    fn get_generator(&self) -> &Self::G;

    fn gen_insert_arguments<'a>(
        &'a self,
        entity: &'a dyn EntityNew,
    ) -> Result<<Self::DB as Database>::Arguments<'_>, BoxDynError>;
    fn gen_upsert_arguments<'a>(
        &'a self,
        entity: &'a dyn EntityNew,
    ) -> Result<<Self::DB as Database>::Arguments<'a>, BoxDynError>;
    fn gen_update_arguments<'a>(
        &'a self,
        update_command: &'a dyn UpdateCommand,
    ) -> Result<<Self::DB as Database>::Arguments<'a>, BoxDynError>;

    fn gen_primary_arguments<'a>(
        &'a self,
        primary: &'a dyn PrimaryNew,
    ) -> Result<<Self::DB as Database>::Arguments<'a>, BoxDynError>;
    fn gen_location_arguments<'a>(
        &'a self,
        location: &'a dyn LocationNew,
    ) -> Result<<Self::DB as Database>::Arguments<'a>, BoxDynError>;
    // fn gen_selected_entity<DB: Database, SL: Selection, SE: SelectedEntityNew<DB>>(
    //     &self,
    //     selection: &SL,
    //     row: Self::DB::Row,
    // ) -> Result<SE, BoxDynError>;

    // async fn select<SE>(
    //     &mut self,
    //     primary: &dyn PrimaryNew,
    //     selection: &dyn Selection,
    // ) -> LunaOrmResult<Option<SE>>
    // where
    //     SE: SelectedEntityNew<Self::DB> + Send + Unpin,
    // {
    //     let pool = self.new_get_pool()?;
    //     let mut conn = pool.acquire().await?;
    //     debug!(target: "luna_orm", command = "select", primary = ?primary, selection = ?selection);
    //     let sql = self.get_generator().get_select_sql(selection, primary);
    //     debug!(target: "luna_orm", command = "select", sql = sql);
    //     let args: <Self::DB as Database>::Arguments<'_> = self.gen_primary_arguments(primary)?;
    //     let result: Option<SE> = self.new_fetch_optional(&mut *conn, &sql, selection, args).await?;
    //     debug!(target: "luna_orm", command = "select", result = ?result);
    //     return Ok(result);
    // }

    async fn select<SE>(
        &self,
        primary: &dyn PrimaryNew,
        selection: &SE::Selection,
    ) -> LunaOrmResult<Option<SE>>
    where
        SE: SelectedEntityNew<Self::DB> + Send + Unpin;
}
