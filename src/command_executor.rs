use crate::error::LunaOrmError;
use crate::sql_executor::SqlExecutor;
use crate::sql_generator::SqlGenerator;
use crate::LunaOrmResult;

use crate::transaction::Transaction;
use async_trait::async_trait;
use luna_orm_trait::*;

#[async_trait]
pub trait CommandExecutor: SqlExecutor {
    type G: SqlGenerator + Sync;

    fn get_generator(&self) -> &Self::G;

    async fn select<SE>(
        &mut self,
        primary: &dyn Primary,
        selection: &dyn Selection,
    ) -> LunaOrmResult<Option<SE>>
    where
        SE: SelectedEntity + Send + Unpin,
    {
        let sql = self.get_generator().get_select_sql(selection, primary);
        let args = primary.any_arguments();
        let result: Option<SE> = self.fetch_optional(&sql, args).await?;
        return Ok(result);
    }

    async fn create<'a>(&mut self, entity: &'a dyn Entity) -> LunaOrmResult<&'a dyn Entity> {
        let sql = self.get_generator().get_insert_sql(entity);
        let args = entity.any_arguments_of_insert();
        self.execute(&sql, args).await?;
        return Ok(entity);
    }

    async fn insert(&mut self, entity: &dyn Entity) -> LunaOrmResult<bool> {
        let sql = self.get_generator().get_insert_sql(entity);
        let args = entity.any_arguments_of_insert();
        let result = self.execute(&sql, args).await?;
        return Ok(result.rows_affected() > 0);
    }

    async fn upsert(&mut self, entity: &dyn Entity) -> LunaOrmResult<bool> {
        let sql = self.get_generator().get_upsert_sql(entity);
        let args = entity.any_arguments_of_upsert();
        let result = self.execute(&sql, args).await?;
        return Ok(result.rows_affected() > 0);
    }

    async fn update(&mut self, entity: &dyn Entity) -> LunaOrmResult<bool> {
        let sql = self.get_generator().get_update_sql(entity);
        let args = entity.any_arguments_of_update();
        let result = self.execute(&sql, args).await?;
        return Ok(result.rows_affected() > 0);
    }

    async fn delete(&mut self, primary: &dyn Primary) -> LunaOrmResult<bool> {
        let sql = self.get_generator().get_delete_sql(primary);
        let args = primary.any_arguments();
        let result = self.execute(&sql, args).await?;
        return Ok(result.rows_affected() > 0);
    }

    #[inline]
    async fn search<EX, L, S, SE>(&mut self, location: &L, selection: &S) -> LunaOrmResult<Vec<SE>>
    where
        L: Location + Sync,
        S: Selection + Sync,
        SE: SelectedEntity + Send + Unpin,
    {
        let sql = self.get_generator().get_search_sql(selection, location);
        let args = location.any_arguments();
        let result: Vec<SE> = self.fetch_all(&sql, args).await?;
        return Ok(result);
    }

    #[inline]
    async fn search_paged<EX, L, S, SE>(
        &mut self,
        location: &L,
        selection: &S,
        page: &Pagination,
    ) -> LunaOrmResult<PagedList<SE>>
    where
        L: Location + Sync,
        S: Selection + Sync,
        SE: SelectedEntity + Send + Unpin,
    {
        let sql = self
            .get_generator()
            .get_paged_search_sql(selection, location, page);
        let args = location.any_arguments();
        let entity_list: Vec<SE> = self.fetch_all(&sql, args).await?;
        let page_info = PageInfo {
            page_size: 10,
            page_num: 10,
            page_total: 10,
            total: 100,
        };
        return Ok(PagedList {
            data: entity_list,
            page: page_info,
        });
    }

    #[inline]
    async fn purify<EX, L>(&mut self, location: &L) -> LunaOrmResult<usize>
    where
        L: Location + Sync,
    {
        let sql = self.get_generator().get_purify_sql(location);
        let args = location.any_arguments();
        let result = self.execute(&sql, args).await?;
        return Ok(result.rows_affected() as usize);
    }

    #[inline]
    async fn change<EX, L, M>(&mut self, location: &L, mutation: &M) -> LunaOrmResult<usize>
    where
        L: Location + Sync,
        M: Mutation + Sync,
    {
        let sql = self.get_generator().get_change_sql(mutation, location);
        let mut args = mutation.any_arguments();
        let where_args = location.any_arguments();
        args = merge_any_arguments(args, where_args);
        let result = self.execute(&sql, args).await?;
        return Ok(result.rows_affected() as usize);
    }
}
