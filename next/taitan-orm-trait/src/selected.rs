use std::fmt::Debug;
use sqlx::Database;
use crate::selection::Selection;

pub trait SelectedEntity<DB: Database>: Debug {
    type Selection: Selection;
    fn from_row(selection: &Self::Selection, row: DB::Row) -> Result<Self, sqlx::Error>
    where
        Self: Sized;
}