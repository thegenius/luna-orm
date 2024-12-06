use sqlx_core::database::Database;
use crate::{Entity, Location, Mutation, OrderBy, Primary, SelectedEntity, Selection};

trait Schema<DB: Database> {
    type Primary: Primary;
    type Location: Location;
    type Mutation: Mutation;
    type Entity: Entity;

    type Selected: SelectedEntity;

    type Selection: Selection;

    fn gen_insert_arguments(&self, entity: &Self::Entity) -> DB::Arguments<'_>;
    fn gen_upsert_arguments(&self, entity: &Self::Entity) -> DB::Arguments<'_>;
    fn gen_update_arguments(&self, mutation: &Self::Mutation, primary: &Self::Primary) -> DB::Arguments<'_>;
    fn gen_change_arguments(&self, mutation: &Self::Mutation, location: &Self::Location) -> DB::Arguments<'_>;
    fn gen_primary_arguments(&self, primary: &Self::Primary)  -> DB::Arguments<'_>;
    fn gen_location_arguments(&self, location: &Self::Location) -> DB::Arguments<'_>;
}