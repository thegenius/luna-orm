use sqlx::any::AnyArguments;
use sqlx::Database;
use sqlx::Encode;
use sqlx::Error as SqlxError;
use sqlx::Row;
use sqlx::Type;

pub trait Primary {
    const TABLE_NAME: &'static str;
    const PRIMARY_FIELD_NAMES: &'static [&'static str];

    fn get_primary_arguments<'p>(&self) -> AnyArguments<'p>;
}

pub trait Location {
    const TABLE_NAME: &'static str;
    fn get_located_fields(&self) -> Vec<(&str, &str)>;
    fn get_arguments<'q, DB, T>(&self) -> Vec<T>
    where
        DB: Database,
        T: Encode<'q, DB> + Type<DB>;
}

pub trait Selection {
    fn get_selected_fields(&self) -> Vec<&str>;
}

pub trait SelectedEntity {
    fn from_row<DB, R>(row: R) -> Result<Self, SqlxError>
    where
        DB: Database,
        R: Row<Database = DB>,
        Self: Sized;
}

pub trait Mutation {
    const TABLE_NAME: &'static str;
    fn get_body_fields(&self) -> Vec<&str>;

    fn get_arguments<'q, DB, T>(&self) -> Vec<T>
    where
        DB: Database,
        T: Encode<'q, DB> + Type<DB>;
}

pub trait Entity {
    const TABLE_NAME: &'static str;
    const PRIMARY_FIELD_NAMES: &'static [&'static str];
    fn get_body_fields(&self) -> Vec<&str>;

    fn get_arguments<'q, DB, T>(&self) -> Vec<T>
    where
        DB: Database,
        T: Encode<'q, DB> + Type<DB>;

    fn from_row<DB, R>(row: R) -> Result<Self, SqlxError>
    where
        DB: Database,
        R: Row<Database = DB>,
        Self: Sized;
}
