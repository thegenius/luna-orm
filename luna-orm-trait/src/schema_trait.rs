use std::fmt::Debug;
use sqlx_core::any::AnyRow;
use sqlx_core::database::Database;
use sqlx_core::error::BoxDynError;
use sqlx_core::row::Row;
use crate::{Entity, Location, Mutation, Primary, SelectedEntity, Selection, SqlxError};


pub trait SelectedEntityNew<DB: Database>: Debug {
    type Selection: Selection;
    fn from_row(selection: &Self::Selection, row: DB::Row) -> Result<Self, SqlxError>
    where
        Self: Sized;
}


pub trait SchemaNew<DB: Database> {
    type Primary: Primary;
    type Location: Location;
    type Mutation: Mutation;
    type Entity: Entity;

    type Selected: SelectedEntityNew<DB>;

    type Selection: Selection;

    fn gen_insert_arguments(entity: &Self::Entity) -> Result<DB::Arguments<'_>, BoxDynError>;
    fn gen_upsert_arguments<'a>(&'a self, entity: &'a Self::Entity) -> Result<DB::Arguments<'_>, BoxDynError>;
    fn gen_update_arguments<'a>(&'a self, mutation: &'a Self::Mutation, primary: &'a Self::Primary) -> Result<DB::Arguments<'_>, BoxDynError>;
    fn gen_change_arguments<'a>(&'a self, mutation: &'a Self::Mutation, location: &'a Self::Location) -> Result<DB::Arguments<'_>, BoxDynError>;
    fn gen_primary_arguments(primary: &Self::Primary)  -> Result<DB::Arguments<'_>, BoxDynError>;
    fn gen_location_arguments<'a>(&'a self, location: &'a Self::Location) -> Result<DB::Arguments<'_>, BoxDynError>;
    fn gen_selected_entity<'a>(&'a self, selection: &'a Self::Selection, row: DB::Row) -> Result<Self::Selected, BoxDynError>;
}