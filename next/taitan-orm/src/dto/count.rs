use sqlx::{Error, MySql, Postgres, Row, Sqlite};
use sqlx::mysql::MySqlRow;
use sqlx::postgres::PgRow;
use sqlx::sqlite::SqliteRow;
use taitan_orm_trait::{SelectedEntity, Selection};


#[derive(Clone, Debug, Default)]
pub struct EmptySelection {
}

impl Selection for EmptySelection {}

#[derive(Clone, Debug)]
pub struct CountResult {
    pub count: i64,
}

impl SelectedEntity<Sqlite> for CountResult {

    type Selection = EmptySelection;

    fn from_row(_selection: &Self::Selection, row: SqliteRow) -> Result<Self, Error>
    where
        Self: Sized
    {
        let count: i64 = row.try_get("count")?;
        Ok(Self { count })
    }
}

impl SelectedEntity<MySql> for CountResult {

    type Selection = EmptySelection;

    fn from_row(_selection: &Self::Selection, row: MySqlRow) -> Result<Self, Error>
    where
        Self: Sized
    {
        let count: i64 = row.try_get("count")?;
        Ok(Self { count })
    }
}

impl SelectedEntity<Postgres> for CountResult {

    type Selection = EmptySelection;

    fn from_row(_selection: &Self::Selection, row: PgRow) -> Result<Self, Error>
    where
        Self: Sized
    {
        let count: i64 = row.try_get("count")?;
        Ok(Self { count })
    }
}