use std::marker::PhantomData;
use crate::dto::PageInfo;
use sqlx::Database;
use taitan_orm_trait::SelectedEntity;

#[derive(Clone, Debug)]
pub struct PagedList<DB, T>
where
    DB: Database,
    T: SelectedEntity<DB>,
{
    pub data: Vec<T>,
    pub page: PageInfo,
    pub _phantom: PhantomData<DB>
}

impl<DB, T> PagedList<DB, T>
where
    DB: Database,
    T: SelectedEntity<DB>,
{
    pub fn empty(page_size: usize, page_num: usize) -> Self {
        Self {
            page: PageInfo::empty(page_size, page_num),
            data: Vec::new(),
            _phantom: PhantomData
        }
    }
}
