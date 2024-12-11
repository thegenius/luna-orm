
use sqlx::{Connection, Database};
use std::fmt::Debug;
use crate::result::Result;
use crate::{PagedList, Pagination, SqlExecutor, SqlGenerator};
use taitan_orm_trait::{Entity, Location, Mutation, OrderBy, Primary, SelectedEntity};
pub trait SqlApi: SqlExecutor + Debug {
    type G: SqlGenerator + Sync + Debug;
    fn get_generator(&self) -> &Self::G;

    async fn insert(&mut self, entity: &dyn Entity) -> Result<bool>;

    async fn upsert(&mut self, entity: &dyn Entity) -> Result<bool>;

    async fn update<M: Mutation>(&mut self, mutation: &M, primary: &M::Primary) -> Result<bool>;

    async fn delete(&mut self, primary: &dyn Primary) -> Result<bool>;

    /**
    根据主键查询1行数据
    */
    async fn select<SE>(
        &self,
        primary: &dyn Primary,
        selection: &SE::Selection,
    ) -> Result<Option<SE>>
    where
        SE: SelectedEntity<Self::DB> + Send + Unpin;

    /**
    根据表中所有数据
    */
    async fn fetch<SE>(&mut self, selection: &SE::Selection) -> Result<Vec<SE>>
    where
        SE: SelectedEntity<Self::DB> + Send + Unpin;

    async fn search<SE>(
        &mut self,
        selection: &SE::Selection,
        location: &dyn Location,
        order_by: Option<&dyn OrderBy>,
    ) -> Result<Vec<SE>>
    where
        SE: SelectedEntity<Self::DB> + Send + Unpin;

    async fn count(&mut self, location: &dyn Location) -> Result<usize>;

    async fn search_paged<SE>(
        &mut self,
        selection: &SE::Selection,
        location: &dyn Location,
        page: &Pagination,
        order_by: Option<&dyn OrderBy>,
    ) -> Result<PagedList<Self::DB, SE>>
    where
        SE: SelectedEntity<Self::DB> + Send + Unpin;
}
