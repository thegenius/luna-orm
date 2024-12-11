
use sqlx::{Connection, Database};
use std::fmt::Debug;
use tracing::debug;
use crate::result::Result;
use crate::{SqlExecutor, SqlGenerator};
use taitan_orm_trait::{Entity, Location, Mutation, OrderBy, Primary, SelectedEntity, TemplateRecord};
use taitan_orm_trait::paged_list::PagedList;
use taitan_orm_trait::pagination::Pagination;

pub trait SqlApi: SqlExecutor + Debug {
    type G: SqlGenerator + Sync + Debug;
    fn get_generator(&self) -> &Self::G;

    async fn insert(&mut self, entity: &dyn Entity) -> Result<bool>;

    async fn upsert(&mut self, entity: &dyn Entity) -> Result<bool>;

    async fn update<M: Mutation>(&mut self, mutation: &M, primary: &M::Primary) -> Result<bool>;

    async fn delete(&mut self, primary: &dyn Primary) -> Result<bool>;


    /**
    查询语义设计：
    返回值语义：
    option: 返回至多1个元素
    vec:    返回元素数组，数组可能为空
    page:  返回元素分页数组

    行级筛选语义：
    select: 使用唯一键筛选，对应返回值option
    search: 使用索引筛选，对应返回值vec/paged
    devour: 不使用条件筛选，对应返回值vec/paged

    字段级筛选语义：
    full：筛选所有字段
    non-full：筛选特定字段
    selection trait具备返回full selection的方法

    select_full(unique) -> option<SE>
    select(selection, unique) -> option<SE>

    search_vec(selection, location)
    search_page(selection, location, page)
    search_full_vec(location)
    search_full_page(location, page)

    devour_vec(selection)
    devour_page(selection, page)
    devour_full_vec()
    devour_full_page(page)

    */


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

    async fn purify(&mut self, location: &dyn Location) -> Result<usize>;

    async fn change<M: Mutation>(&mut self, mutation: &M, location: &M::Location) -> Result<bool>;

    async fn execute_by_template(&mut self, template: &dyn TemplateRecord) -> Result<usize>;

    // async fn select_by_template<SE>(
    //     &mut self,
    //     template: &dyn TemplateRecord,
    // ) -> Result<Option<SE>>
    // where
    //     SE: SelectedEntity<Self::DB> + Send + Unpin;
}
