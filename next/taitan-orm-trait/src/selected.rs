use crate::selection::Selection;
use crate::NotImplementError;
use sqlx::Database;
use std::fmt::Debug;

pub trait SelectedEntity<DB: Database>: Debug + Default {
    type Selection: Selection;
    fn from_row(selection: &Self::Selection, row: DB::Row) -> Result<Self, sqlx::Error>
    where
        Self: Sized;

    fn from_row_full(row: DB::Row) -> Result<Self, sqlx::Error>
    where
        Self: Sized,
    {
        Err(sqlx::Error::Decode(
            NotImplementError("".to_string()).into(),
        ))
    }
}
