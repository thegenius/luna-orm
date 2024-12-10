use crate::{Entity, Location, Mutation, Primary, SelectedEntity, Selection, SqlxError};
use sqlx_core::any::{AnyArguments, AnyRow};
use sqlx_core::database::Database;
use sqlx_core::error::BoxDynError;
use sqlx_core::row::Row;
use std::fmt::Debug;
use sqlx::mysql::MySqlArguments;
use sqlx::postgres::PgArguments;
use sqlx::sqlite::SqliteArguments;
use sqlx_core::arguments::Arguments;

#[derive(Debug)]
struct NotImplementError(String);
impl std::error::Error for NotImplementError {}
impl std::fmt::Display for NotImplementError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "method {} is not implements", self.0)
    }
}


pub trait EntityNew: Sync + Debug {
    fn get_table_name(&self) -> &str;

    fn get_insert_fields(&self) -> Vec<String>;

    fn get_upsert_set_fields(&self) -> Vec<String>;

    fn get_auto_increment_field(&self) -> Option<&str>;

    fn set_auto_increment_field(&mut self, value: Option<i64>) -> bool;

    fn gen_insert_arguments_sqlite(&self) -> Result<SqliteArguments<'_>, BoxDynError> {
        Err(NotImplementError("gen_insert_arguments_sqlite".to_string()).into())
    }
    fn gen_upsert_arguments_sqlite(&self) -> Result<SqliteArguments<'_>, BoxDynError> {
        Err(NotImplementError("gen_upsert_arguments_sqlite".to_string()).into())
    }
    fn gen_insert_arguments_mysql(&self) -> Result<MySqlArguments, BoxDynError> {
        Err(NotImplementError("gen_insert_arguments_mysql".to_string()).into())
    }
    fn gen_upsert_arguments_mysql(&self) -> Result<MySqlArguments, BoxDynError> {
        Err(NotImplementError("gen_upsert_arguments_mysql".to_string()).into())
    }

    fn gen_insert_arguments_postgres(&self) -> Result<PgArguments, BoxDynError> {
        Err(NotImplementError("gen_insert_arguments_postgres".to_string()).into())
    }
    fn gen_upsert_arguments_postgres(&self) -> Result<PgArguments, BoxDynError> {
        Err(NotImplementError("gen_upsert_arguments_postgres".to_string()).into())
    }
}


pub trait PrimaryNew: Sync + Debug {
    type Mutation: MutationNew;
    fn get_table_name(&self) -> &'static str;

    fn get_primary_field_names(&self) -> &'static [&'static str];

    fn gen_primary_arguments_sqlite(&self) -> Result<SqliteArguments<'_>, BoxDynError> {
        Err(NotImplementError("gen_primary_arguments_sqlite".to_string()).into())
    }
    fn gen_primary_arguments_mysql(&self) -> Result<MySqlArguments, BoxDynError> {
        Err(NotImplementError("gen_primary_arguments_mysql".to_string()).into())
    }
    fn gen_primary_arguments_postgres(&self) -> Result<PgArguments, BoxDynError> {
        Err(NotImplementError("gen_primary_arguments_postgres".to_string()).into())
    }

    fn gen_update_arguments_sqlite<'a>(&'a self, mutation: &'a Self::Mutation) -> Result<SqliteArguments<'a>, BoxDynError> {
        Err(NotImplementError("gen_update_arguments_sqlite".to_string()).into())
    }
    fn gen_update_arguments_mysql(&self, mutation: &Self::Mutation) -> Result<MySqlArguments, BoxDynError> {
        Err(NotImplementError("gen_update_arguments_mysql".to_string()).into())
    }
    fn gen_update_arguments_postgres(&self, mutation: &Self::Mutation) -> Result<PgArguments, BoxDynError> {
        Err(NotImplementError("gen_update_arguments_postgres".to_string()).into())
    }
}

pub trait MutationNew: Sync + Debug {

    fn get_fields_name(&self) -> Vec<String>;
}

pub trait LocationNew: Sync + Debug {
    type Mutation: MutationNew;
    fn get_table_name(&self) -> &'static str;

    fn get_fields_name(&self) -> Vec<String>;

    fn get_where_clause(&self, wrap_char: char, place_holder: char) -> String;

    fn check_valid_order_by(&self, fields: &[&str]) -> bool;

    fn gen_location_arguments_sqlite(&self) -> Result<SqliteArguments<'_>, BoxDynError> {
        Err(NotImplementError("gen_primary_arguments_sqlite".to_string()).into())
    }
    fn gen_location_arguments_mysql(&self) -> Result<MySqlArguments, BoxDynError> {
        Err(NotImplementError("gen_primary_arguments_mysql".to_string()).into())
    }
    fn gen_location_arguments_postgres(&self) -> Result<PgArguments, BoxDynError> {
        Err(NotImplementError("gen_primary_arguments_postgres".to_string()).into())
    }

    fn gen_change_arguments_sqlite<'a>(&'a self, mutation: &'a Self::Mutation) -> Result<SqliteArguments<'a>, BoxDynError> {
        Err(NotImplementError("gen_update_arguments_sqlite".to_string()).into())
    }
    fn gen_change_arguments_mysql(&self, mutation: &Self::Mutation) -> Result<MySqlArguments, BoxDynError> {
        Err(NotImplementError("gen_update_arguments_mysql".to_string()).into())
    }
    fn gen_change_arguments_postgres(&self, mutation: &Self::Mutation) -> Result<PgArguments, BoxDynError> {
        Err(NotImplementError("gen_update_arguments_postgres".to_string()).into())
    }
}

pub trait SelectedEntityNew<DB: Database>: Debug {
    type Selection: Selection;
    fn from_row(selection: &Self::Selection, row: DB::Row) -> Result<Self, SqlxError>
    where
        Self: Sized;
}




// pub trait SchemaNew<DB: Database> {
//     type Primary: Primary;
//     type Location: Location;
//     type Mutation: Mutation;
//     type Entity: EntityNew;
//
//     type Selected: SelectedEntityNew<DB>;
//
//     type Selection: Selection;
//
//     fn gen_insert_arguments(entity: &Self::Entity) -> Result<DB::Arguments<'_>, BoxDynError>;
//     fn gen_upsert_arguments(entity: &Self::Entity) -> Result<DB::Arguments<'_>, BoxDynError>;
//     fn gen_update_arguments<'a>(
//         mutation: &'a Self::Mutation,
//         primary: &'a Self::Primary,
//     ) -> Result<DB::Arguments<'a>, BoxDynError>;
//     fn gen_change_arguments<'a>(
//         mutation: &'a Self::Mutation,
//         location: &'a Self::Location,
//     ) -> Result<DB::Arguments<'a>, BoxDynError>;
//     fn gen_primary_arguments(primary: &Self::Primary) -> Result<DB::Arguments<'_>, BoxDynError>;
//     fn gen_location_arguments(location: &Self::Location) -> Result<DB::Arguments<'_>, BoxDynError>;
//     fn gen_selected_entity(
//         selection: &Self::Selection,
//         row: DB::Row,
//     ) -> Result<Self::Selected, BoxDynError>;
// }
