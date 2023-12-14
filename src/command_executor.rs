use crate::error::LunaOrmError;
use crate::sql_executor::SqlExecutor;
use crate::sql_generator::SqlGenerator;
use crate::LunaOrmResult;

use crate::transaction::Transaction;
use async_trait::async_trait;
use luna_orm_trait::*;

pub trait PrimarySync: Primary + Sync {}
pub trait SelectionSync: Selection + Sync {}
pub trait EntitySync: Entity + Sync {}

#[async_trait]
pub trait CommandExecutor: SqlExecutor {
    type G: SqlGenerator + Sync;

    fn get_generator(&self) -> &Self::G;

    /*
    async fn try_select<SE>(
        &mut self,
        primary: &dyn PrimarySync,
        selection: &dyn SelectionSync,
    ) -> LunaOrmResult<Option<SE>>
    where
        SE: SelectedEntity + Send + Unpin,
    {
        let sql = self.get_generator().get_select_sql(selection, primary);
        let args = primary.any_arguments();
        let result: Option<SE> = self.fetch_optional(&sql, args).await?;
        return Ok(result);
    }

    async fn try_create(&mut self, entity: &dyn EntitySync) -> LunaOrmResult<&dyn EntitySync> {
        let sql = self.get_generator().get_insert_sql(entity);
        let args = entity.any_arguments_of_insert();
        self.execute(&sql, args).await?;
        return Ok(entity);
    }
    */

    #[inline]
    async fn select<P, S, SE>(&mut self, primary: &P, selection: &S) -> LunaOrmResult<Option<SE>>
    where
        P: Primary + Sync,
        S: Selection + Sync,
        SE: SelectedEntity + Send + Unpin,
    {
        let sql = self.get_generator().get_select_sql(selection, primary);
        let args = primary.any_arguments();
        let result: Option<SE> = self.fetch_optional(&sql, args).await?;
        return Ok(result);
    }

    #[inline]
    async fn create<'a, E>(&mut self, entity: &'a E) -> LunaOrmResult<&'a E>
    where
        E: Entity + Sync,
    {
        let sql = self.get_generator().get_insert_sql(entity);
        let args = entity.any_arguments_of_insert();
        self.execute(&sql, args).await?;
        return Ok(entity);
    }

    #[inline]
    async fn insert<E>(&mut self, entity: &E) -> LunaOrmResult<bool>
    where
        E: Entity + Sync,
    {
        let sql = self.get_generator().get_insert_sql(entity);
        let args = entity.any_arguments_of_insert();
        let result = self.execute(&sql, args).await?;
        return Ok(result.rows_affected() > 0);
    }

    #[inline]
    async fn upsert<E>(&mut self, entity: &E) -> LunaOrmResult<bool>
    where
        E: Entity + Sync,
    {
        let sql = self.get_generator().get_upsert_sql(entity);
        let args = entity.any_arguments_of_upsert();
        let result = self.execute(&sql, args).await?;
        return Ok(result.rows_affected() > 0);
    }

    #[inline]
    async fn update<E>(&mut self, entity: &E) -> LunaOrmResult<bool>
    where
        E: Entity + Sync,
    {
        let sql = self.get_generator().get_update_sql(entity);
        let args = entity.any_arguments_of_update();
        let result = self.execute(&sql, args).await?;
        return Ok(result.rows_affected() > 0);
    }

    /*
    #[inline]
    async fn remove<P, S, SE>(&mut self, primary: P, selection: S) -> LunaOrmResult<Option<SE>>
    where
        P: Primary + Send + Clone,
        S: Selection + Send,
        SE: SelectedEntity + Send + Unpin,
    {
        let primary_cloned = primary.clone();
        let selected_entity: Option<SE> = self.select(primary_cloned, selection).await?;
        let sql = self.get_generator().get_delete_sql(&primary);
        let args = primary.into_any_arguments();
        let result = self.execute(&sql, args).await?;
        if result.rows_affected() > 0 {
            return Ok(selected_entity);
        } else {
            return Ok(None);
        }
    }
    */

    #[inline]
    async fn delete<P>(&mut self, primary: &P) -> LunaOrmResult<bool>
    where
        P: Primary + Sync,
    {
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
        location: L,
        selection: S,
        page: &Pagination,
    ) -> LunaOrmResult<PagedList<SE>>
    where
        L: Location + Send,
        S: Selection + Send,
        SE: SelectedEntity + Send + Unpin,
    {
        let sql = self
            .get_generator()
            .get_paged_search_sql(&selection, &location, page);
        let args = location.into_any_arguments();
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
    async fn purify<EX, L>(&mut self, location: L) -> LunaOrmResult<usize>
    where
        L: Location + Send,
    {
        let sql = self.get_generator().get_purify_sql(&location);
        let args = location.into_any_arguments();
        let result = self.execute(&sql, args).await?;
        return Ok(result.rows_affected() as usize);
    }

    #[inline]
    async fn change<EX, L, M>(&mut self, location: L, mutation: M) -> LunaOrmResult<usize>
    where
        L: Location + Send,
        M: Mutation + Send,
    {
        let sql = self.get_generator().get_change_sql(&mutation, &location);
        let mut args = mutation.into_any_arguments();
        let where_args = location.into_any_arguments();
        args = merge_any_arguments(args, where_args);
        let result = self.execute(&sql, args).await?;
        return Ok(result.rows_affected() as usize);
    }
}
