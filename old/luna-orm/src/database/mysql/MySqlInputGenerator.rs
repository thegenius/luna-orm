use std::fmt::Arguments;
use sqlx::{Database, MySql};
use luna_orm_trait::{Entity, Location, Mutation, OrderBy, Primary};
use luna_orm_trait::input_generator::InputGenerator;

// struct MysqlInputGenerator;
// impl<DB: Database> InputGenerator<DB> for MysqlInputGenerator {
//     fn gen_insert_arguments(&self, entity: &dyn Entity) ->  DB::Arguments<'_> {
//         // let mut arguments: <MySql as Database>::Arguments<'_> = <MySql as Database>::Arguments::default();
//         // return arguments;
//         todo!()
//     }
//
//     fn gen_upsert_arguments(&self, entity: &dyn Entity) -> <DB as Database>::Arguments<'_> {
//         todo!()
//     }
//
//     fn gen_update_arguments(&self, mutation: &dyn Mutation, primary: &dyn Primary) -> <DB as Database>::Arguments<'_> {
//         todo!()
//     }
//
//     fn gen_change_arguments(&self, mutation: &dyn Mutation, location: &dyn Location) -> <DB as Database>::Arguments<'_> {
//         todo!()
//     }
//
//     fn gen_primary_arguments(&self, primary: &dyn Primary) -> <DB as Database>::Arguments<'_> {
//         todo!()
//     }
//
//     fn gen_location_arguments(&self, location: &dyn Location, order_by_option: Option<&dyn OrderBy>) -> <DB as Database>::Arguments<'_> {
//         todo!()
//     }
// }