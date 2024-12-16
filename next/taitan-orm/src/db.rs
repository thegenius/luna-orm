// use crate::database::sqlite::{SqliteCommander, SqliteDatabase};
// use crate::{SqlApi, SqlExecutor};
// use std::ops::{Deref, DerefMut};
// use sqlx::Database;
// use taitan_orm_trait::{Entity, Location, Mutation, OrderBy, SelectedEntity, TemplateRecord, Unique};
// use taitan_orm_trait::paged_list::PagedList;
// use taitan_orm_trait::pagination::Pagination;
//
// #[derive(Debug, Clone)]
// enum DatabaseEnum{
//     Sqlite(SqliteDatabase),
// }
//
//
// impl SqlApi for DatabaseEnum {
//     async fn insert(&mut self, entity: &dyn Entity) -> crate::Result<bool> {
//         match self { DatabaseEnum::Sqlite(sqlite) => {sqlite.insert(entity)} }
//     }
//
//     async fn upsert(&mut self, entity: &dyn Entity) -> crate::Result<bool> {
//         todo!()
//     }
//
//     async fn update<M: Mutation>(&mut self, mutation: &M, unique: &M::Primary) -> crate::Result<bool> {
//         todo!()
//     }
//
//     async fn change<M: Mutation>(&mut self, mutation: &M, location: &M::Location) -> crate::Result<u64> {
//         todo!()
//     }
//
//     async fn delete(&mut self, unique: &dyn Unique) -> crate::Result<bool> {
//         todo!()
//     }
//
//     async fn purify(&mut self, location: &dyn Location) -> crate::Result<u64> {
//         todo!()
//     }
//
//     async fn select<DB: Database, SE>(&mut self, selection: &SE::Selection, unique: &dyn Unique) -> crate::Result<Option<SE>>
//     where
//         SE: SelectedEntity<DB> + Send + Unpin
//     {
//         todo!()
//     }
//
//     async fn search<DB: Database, SE>(&mut self, selection: &SE::Selection, location: &dyn Location, order_by: &dyn OrderBy) -> crate::Result<Vec<SE>>
//     where
//         SE: SelectedEntity<DB> + Send + Unpin
//     {
//         todo!()
//     }
//
//     async fn search_paged<DB: Database, SE>(&mut self, selection: &SE::Selection, location: &dyn Location, order_by: &dyn OrderBy, page: &Pagination) -> crate::Result<PagedList<DB, SE>>
//     where
//         SE: SelectedEntity<DB> + Send + Unpin
//     {
//         todo!()
//     }
//
//     async fn devour<DB: Database, SE>(&mut self, selection: &SE::Selection, order_by: &dyn OrderBy) -> crate::Result<Vec<SE>>
//     where
//         SE: SelectedEntity<DB> + Send + Unpin
//     {
//         todo!()
//     }
//
//     async fn devour_paged<DB: Database, SE>(&mut self, selection: &SE::Selection, order_by: &dyn OrderBy, page: &Pagination) -> crate::Result<PagedList<DB, SE>>
//     where
//         SE: SelectedEntity<DB> + Send + Unpin
//     {
//         todo!()
//     }
//
//     async fn count(&mut self, location: &dyn Location) -> crate::Result<u64> {
//         todo!()
//     }
//
//     async fn count_table(&mut self, table_name: &str) -> crate::Result<u64> {
//         todo!()
//     }
//
//     async fn execute_by_template(&mut self, template: &dyn TemplateRecord) -> crate::Result<usize> {
//         todo!()
//     }
// }
