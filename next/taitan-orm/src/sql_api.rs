use crate::result::Result;
use crate::{SqlExecutor, SqlGenerator};
use sqlx::{Connection, Database};
use std::fmt::Debug;
use taitan_orm_trait::paged_list::PagedList;
use taitan_orm_trait::pagination::Pagination;
use taitan_orm_trait::{
    Entity, Location, Mutation, OrderBy, SelectedEntity, TemplateRecord, Unique,
};
use tracing::debug;

pub trait SqlApi: SqlExecutor + Debug {
    type G: SqlGenerator + Sync + Debug;
    fn get_generator(&self) -> &Self::G;

    async fn insert(&mut self, entity: &dyn Entity) -> Result<bool>;
    async fn upsert(&mut self, entity: &dyn Entity) -> Result<bool>;
    async fn update<M: Mutation>(&mut self, mutation: &M, unique: &M::Primary) -> Result<bool>;
    async fn change<M: Mutation>(&mut self, mutation: &M, location: &M::Location) -> Result<bool>;
    async fn delete(&mut self, unique: &dyn Unique) -> Result<bool>;
    async fn purify(&mut self, location: &dyn Location) -> Result<usize>;

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
    selection具备返回all_fields的方法

    最终查询api设计：
    select(selection, unique) -> option<SE>

    search(selection, location, order_by)
    search_page(selection, location, page, order_by)

    devour(selection, order_by)
    devour_page(selection, page, order_by)
    */

    /**
    根据主键查询1行数据
    */
    async fn select<SE>(
        &self,
        selection: &SE::Selection,
        unique: &dyn Unique
    ) -> Result<Option<SE>>
    where
        SE: SelectedEntity<Self::DB> + Send + Unpin;

    async fn search<SE, O>(
        &mut self,
        selection: &SE::Selection,
        location: &dyn Location,
        order_by: &O,
    ) -> Result<Vec<SE>>
    where
        SE: SelectedEntity<Self::DB> + Send + Unpin, O: OrderBy;

    async fn search_paged<SE, O>(
        &mut self,
        selection: &SE::Selection,
        location: &dyn Location,
        page: &Pagination,
        order_by: &O,
    ) -> Result<PagedList<Self::DB, SE>>
    where
        SE: SelectedEntity<Self::DB> + Send + Unpin, O: OrderBy;

    /**
    根据表中所有数据
    */
    async fn fetch<SE>(&mut self, selection: &SE::Selection) -> Result<Vec<SE>>
    where
        SE: SelectedEntity<Self::DB> + Send + Unpin;



    async fn count(&mut self, location: &dyn Location) -> Result<usize>;



    async fn execute_by_template(&mut self, template: &dyn TemplateRecord) -> Result<usize>;

    // async fn select_by_template<SE>(
    //     &mut self,
    //     template: &dyn TemplateRecord,
    // ) -> Result<Option<SE>>
    // where
    //     SE: SelectedEntity<Self::DB> + Send + Unpin;
}
