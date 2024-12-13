use crate::selection::Selection;
use sqlx::Database;
use std::fmt::Debug;
use crate::NotImplementError;

pub trait SelectedEntity<DB: Database>: Debug {
    type Selection: Selection;
    fn from_row(selection: &Self::Selection, row: DB::Row) -> Result<Self, sqlx::Error>
    where
        Self: Sized;

    fn from_row_full(row: DB::Row) -> Result<Self, sqlx::Error>
    where
        Self: Sized {
        Err(sqlx::Error::Decode(NotImplementError("".to_string()).into() ))
    }
}
