use sqlx::{Arguments, Database};
use crate::{Entity, Location, Mutation, OrderBy, Primary};

pub trait InputGenerator<DB: Database> {
    fn gen_insert_arguments(&self, entity: &dyn Entity) -> DB::Arguments<'_>;
    fn gen_upsert_arguments(&self, entity: &dyn Entity) -> <DB as Database>::Arguments<'_>;
    fn gen_update_arguments(&self, mutation: &dyn Mutation, primary: &dyn Primary) -> <DB as Database>::Arguments<'_>;
    fn gen_change_arguments(&self, mutation: &dyn Mutation, location: &dyn Location) -> <DB as Database>::Arguments<'_>;
    fn gen_primary_arguments(&self, primary: &dyn Primary)  -> <DB as Database>::Arguments<'_>;
    fn gen_location_arguments(&self, location: &dyn Location, order_by_option: Option<&dyn OrderBy>) -> <DB as Database>::Arguments<'_>;
}
