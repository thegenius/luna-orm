use crate::error::LunaOrmError;
use crate::sql_executor::SqlExecutor;
use crate::sql_generator::SqlGenerator;
use crate::LunaOrmResult;

use async_trait::async_trait;
use luna_orm_trait::*;

#[async_trait]
pub trait CommandExecutor: SqlExecutor {
    type G: SqlGenerator + Sync;

    fn get_generator(&self) -> &Self::G;

    #[inline]
    async fn select<P, S, SE>(&mut self, primary: P, selection: S) -> LunaOrmResult<Option<SE>>
    where
        P: Primary + Send,
        S: Selection + Send,
        SE: SelectedEntity + Send + Unpin,
    {
        let sql = self.get_generator().get_select_sql(&selection, &primary);
        let args = primary.into_any_arguments();
        let result: Option<SE> = self.fetch_optional(&sql, args).await?;
        return Ok(result);
    }

    #[inline]
    async fn create<E>(&mut self, entity: E) -> LunaOrmResult<E>
    where
        E: Entity + Send + Clone,
    {
        let sql = self.get_generator().get_insert_sql(&entity);
        let entity_clone = entity.clone();
        let args = entity.into_insert_any_arguments();
        self.execute(&sql, args).await?;
        return Ok(entity_clone);
    }

    #[inline]
    async fn insert<E>(&mut self, entity: E) -> LunaOrmResult<bool>
    where
        E: Entity + Send + Clone,
    {
        let sql = self.get_generator().get_insert_sql(&entity);
        let args = entity.into_insert_any_arguments();
        let result = self.execute(&sql, args).await?;
        return Ok(result.rows_affected() > 0);
    }

    #[inline]
    async fn upsert<E>(&mut self, entity: E) -> LunaOrmResult<bool>
    where
        E: Entity + Send + Clone,
    {
        let sql = self.get_generator().get_upsert_sql(&entity);
        let args = entity.into_upsert_any_arguments();
        let result = self.execute(&sql, args).await?;
        return Ok(result.rows_affected() > 0);
    }

    #[inline]
    async fn update<E>(&mut self, entity: E) -> LunaOrmResult<bool>
    where
        E: Entity + Send + Clone,
    {
        let sql = self.get_generator().get_update_sql(&entity);
        let args = entity.into_update_any_arguments();
        let result = self.execute(&sql, args).await?;
        return Ok(result.rows_affected() > 0);
    }

    #[inline]
    async fn remove<P, E>(&mut self, primary: P) -> LunaOrmResult<E>
    where
        P: Primary + Send,
        E: Entity + Send + Clone,
    {
        todo!()
        /*
        let sql = self.get_generator().get_delete_sql(&primary);
        let args = primary.into_any_arguments();
        let result = self.execute(&sql, args).await?;
        return Ok(result.rows_affected() > 0);
        */
    }

    #[inline]
    async fn delete<P>(&mut self, primary: P) -> LunaOrmResult<bool>
    where
        P: Primary + Send,
    {
        let sql = self.get_generator().get_delete_sql(&primary);
        let args = primary.into_any_arguments();
        let result = self.execute(&sql, args).await?;
        return Ok(result.rows_affected() > 0);
    }

    #[inline]
    async fn search<EX, L, S, SE>(&mut self, location: L, selection: S) -> LunaOrmResult<Vec<SE>>
    where
        L: Location + Send,
        S: Selection + Send,
        SE: SelectedEntity + Send + Unpin,
    {
        let sql = self.get_generator().get_search_sql(&selection, &location);
        let args = location.into_any_arguments();
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
