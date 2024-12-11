use crate::selection::Selection;
use sqlx::Database;
use std::fmt::Debug;

pub trait SelectedEntity<DB: Database>: Debug {
    type Selection: Selection;
    fn from_row(selection: &Self::Selection, row: DB::Row) -> Result<Self, sqlx::Error>
    where
        Self: Sized;
}
