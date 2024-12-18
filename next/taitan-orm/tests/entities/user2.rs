use sqlx::error::BoxDynError;
use sqlx::mysql::MySqlArguments;
use sqlx::postgres::PgArguments;
use sqlx::sqlite::SqliteArguments;
use sqlx::{sqlx_macros, Row};
use sqlx::Arguments;
use taitan_orm_macro::Schema;
use taitan_orm_trait::NotImplementError;
use taitan_orm_trait::{Entity, Unique, Location, Schema, LocationExpr, Mutation, Selection, SelectedEntity};
use sqlx::{Sqlite, MySql, Postgres};
use sqlx::Database;


#[derive(Clone, Debug)]
pub struct UserEntity {
    age: Option<i32>,

    id: Option<i64>,

    name: String,
}
impl Entity for UserEntity
{
    fn get_table_name(& self) -> & 'static str { "user" } fn
get_insert_fields(& self) -> Vec < String >
{
    let mut fields = Vec :: new(); if self.age.is_some()
{ fields.push("age".to_string()); }; fields.push("name".to_string());
    ; return fields;
} fn get_upsert_set_fields(& self) -> Vec < String >
{
    let mut fields = Vec :: new(); if self.age.is_some()
{ fields.push("age".to_string()); }; fields.push("name".to_string());
    ; return fields;
} fn get_auto_increment_field(& self) -> Option < & 'static str >
{ Some("id") } fn
set_auto_increment_field(& mut self, value : Option < i64 >) -> bool
{ self.id = value; true } fn gen_insert_arguments_sqlite(& self) -> Result
< SqliteArguments < '_ > , BoxDynError >
{
    let mut args = SqliteArguments :: default(); if let Some(age) = &
    self.age { args.add(age) ? ; } args.add(& self.name) ? ; Ok(args)
} fn gen_upsert_arguments_sqlite(& self) -> Result < SqliteArguments < '_
> , BoxDynError >
{
    let mut args = SqliteArguments :: default(); if let Some(age) = &
    self.age { args.add(age) ? ; } args.add(& self.name) ? ; if let
    Some(age) = & self.age { args.add(age) ? ; } args.add(& self.name) ? ;
    Ok(args)
} fn gen_insert_arguments_mysql(& self) -> Result < MySqlArguments,
    BoxDynError >
{
    let mut args = MySqlArguments :: default(); if let Some(age) = &
    self.age { args.add(age) ? ; } args.add(& self.name) ? ; Ok(args)
} fn gen_upsert_arguments_mysql(& self) -> Result < MySqlArguments,
    BoxDynError >
{
    let mut args = MySqlArguments :: default(); if let Some(age) = &
    self.age { args.add(age) ? ; } args.add(& self.name) ? ; if let
    Some(age) = & self.age { args.add(age) ? ; } args.add(& self.name) ? ;
    Ok(args)
} fn gen_insert_arguments_postgres(& self) -> Result < PgArguments,
    BoxDynError >
{
    let mut args = PgArguments :: default(); if let Some(age) = & self.age
{ args.add(age) ? ; } args.add(& self.name) ? ; Ok(args)
} fn gen_upsert_arguments_postgres(& self) -> Result < PgArguments,
    BoxDynError >
{
    let mut args = PgArguments :: default(); if let Some(age) = & self.age
{ args.add(age) ? ; } args.add(& self.name) ? ; if let Some(age) = &
    self.age { args.add(age) ? ; } args.add(& self.name) ? ; Ok(args)
}
} #[derive(Default, Debug, Clone)] pub struct UserPrimary { id : i64, } impl
Unique for UserPrimary
{
    fn get_table_name(& self) -> & 'static str { "user" } fn
get_unique_field_names(& self) -> & 'static [& 'static str] { & ["id",] }
    fn gen_unique_arguments_sqlite(& self) -> Result < SqliteArguments < '_ >
        , BoxDynError >
    {
        let mut args = SqliteArguments :: default(); args.add(& self.id) ? ;
        Ok(args)
    } fn gen_unique_arguments_mysql(& self) -> Result < MySqlArguments,
    BoxDynError >
{
    let mut args = MySqlArguments :: default(); args.add(& self.id) ? ;
    Ok(args)
} fn gen_unique_arguments_postgres(& self) -> Result < PgArguments,
    BoxDynError >
{
    let mut args = PgArguments :: default(); args.add(& self.id) ? ;
    Ok(args)
}
} #[derive(Default, Debug, Clone)] pub struct UserLocation
{
    age : Option < LocationExpr < i32 >> , id : Option < LocationExpr < i64 >>
    , name : Option < LocationExpr < String >> ,
} impl Location for UserLocation
{
    fn get_table_name(& self) -> & 'static str { "user" } fn
get_location_fields_name(& self) -> Vec < String >
{
    let mut fields = Vec :: new(); if self.age.is_some()
{ fields.push("age".to_string()); }; if self.id.is_some()
{ fields.push("id".to_string()); }; fields.push("name".to_string()); ;
    return fields;
} fn get_where_clause(& self, wrap_char : char, place_holder : char) ->
String
{
    let mut sql = String :: default(); if let Some(age) = & self.age
{
    sql.push(wrap_char); sql.push_str("age"); sql.push(wrap_char);
    sql.push_str(age.cmp.get_sql()); sql.push(place_holder);
} if let Some(id) = & self.id
{
    sql.push(wrap_char); sql.push_str("id"); sql.push(wrap_char);
    sql.push_str(id.cmp.get_sql()); sql.push(place_holder);
} if let Some(name) = & self.name
{
    sql.push(wrap_char); sql.push_str("name"); sql.push(wrap_char);
    sql.push_str(name.cmp.get_sql()); sql.push(place_holder);
} return sql;
} fn gen_location_arguments_sqlite(& self) -> Result < SqliteArguments <
    '_ > , BoxDynError >
{
    let mut args = SqliteArguments :: default(); if let Some(age) = &
    self.age { args.add(& age.val) ? ; } if let Some(id) = & self.id
{ args.add(& id.val) ? ; } if let Some(name) = & self.name
{ args.add(& name.val) ? ; } Ok(args)
} fn gen_location_arguments_mysql(& self) -> Result < MySqlArguments,
    BoxDynError >
{
    let mut args = MySqlArguments :: default(); if let Some(age) = &
    self.age { args.add(& age.val) ? ; } if let Some(id) = & self.id
{ args.add(& id.val) ? ; } if let Some(name) = & self.name
{ args.add(& name.val) ? ; } Ok(args)
} fn gen_location_arguments_postgres(& self) -> Result < PgArguments,
    BoxDynError >
{
    let mut args = PgArguments :: default(); if let Some(age) = & self.age
{ args.add(& age.val) ? ; } if let Some(id) = & self.id
{ args.add(& id.val) ? ; } if let Some(name) = & self.name
{ args.add(& name.val) ? ; } Ok(args)
}
} #[derive(Default, Debug, Clone)] pub struct UserMutation
{ age : Option < i32 > , name : Option < String > , } impl Mutation for
UserMutation
{
    type Primary = UserPrimary; type Location = UserLocation; fn
get_mutation_fields_name(& self) -> Vec < String >
{
    let mut fields = Vec :: new(); if let Some(_) = self.age
{ fields.push("age".to_string()); }; if let Some(_) = self.name
{ fields.push("name".to_string()); }; return fields;
} fn gen_update_arguments_sqlite < 'a >
(& 'a self, primary : & 'a Self :: Primary,) -> Result < SqliteArguments <
        'a > , BoxDynError >
{
    let mut args = SqliteArguments :: default(); if let Some(age) = &
    self.age { args.add(age) ? ; } if let Some(name) = & self.name
{ args.add(name) ? ; } args.add(& primary.id) ? ; Ok(args)
} fn gen_update_arguments_mysql < 'a >
(& 'a self, primary : & 'a Self :: Primary,) -> Result < MySqlArguments,
        BoxDynError >
{
    let mut args = MySqlArguments :: default(); if let Some(age) = &
    self.age { args.add(age) ? ; } if let Some(name) = & self.name
{ args.add(name) ? ; } args.add(& primary.id) ? ; Ok(args)
} fn gen_update_arguments_postgres < 'a >
(& 'a self, primary : & 'a Self :: Primary,) -> Result < PgArguments,
        BoxDynError >
{
    let mut args = PgArguments :: default(); if let Some(age) = & self.age
{ args.add(age) ? ; } if let Some(name) = & self.name
{ args.add(name) ? ; } args.add(& primary.id) ? ; Ok(args)
} fn gen_change_arguments_sqlite < 'a >
(& 'a self, location : & 'a Self :: Location,) -> Result < SqliteArguments
    < 'a > , BoxDynError >
{
    let mut args = SqliteArguments :: default(); if let Some(age) = &
    self.age { args.add(age) ? ; } if let Some(name) = & self.name
{ args.add(name) ? ; } if let Some(age) = & location.age
{ args.add(& age.val) ? ; } if let Some(id) = & location.id
{ args.add(& id.val) ? ; } if let Some(name) = & location.name
{ args.add(& name.val) ? ; } Ok(args)
} fn gen_change_arguments_mysql < 'a >
(& 'a self, location : & 'a Self :: Location,) -> Result < MySqlArguments,
        BoxDynError >
{
    let mut args = MySqlArguments :: default(); if let Some(age) = &
    self.age { args.add(age) ? ; } if let Some(name) = & self.name
{ args.add(name) ? ; } if let Some(age) = & location.age
{ args.add(& age.val) ? ; } if let Some(id) = & location.id
{ args.add(& id.val) ? ; } if let Some(name) = & location.name
{ args.add(& name.val) ? ; } Ok(args)
} fn gen_change_arguments_postgres < 'a >
(& 'a self, location : & 'a Self :: Location,) -> Result < PgArguments,
        BoxDynError >
{
    let mut args = PgArguments :: default(); if let Some(age) = & self.age
{ args.add(age) ? ; } if let Some(name) = & self.name
{ args.add(name) ? ; } if let Some(age) = & location.age
{ args.add(& age.val) ? ; } if let Some(id) = & location.id
{ args.add(& id.val) ? ; } if let Some(name) = & location.name
{ args.add(& name.val) ? ; } Ok(args)
}
} #[derive(Default, Debug, Clone)] pub struct UserSelection
{ age : bool, id : bool, name : bool, } impl Selection for UserSelection
{
    fn get_table_name(& self) -> & 'static str { "user" } fn
get_selected_fields(& self) -> Vec < String >
{
    let mut fields = Vec :: new(); if self.age
{ fields.push("age".to_string()); }; if self.id
{ fields.push("id".to_string()); }; if self.name
{ fields.push("name".to_string()); }; return fields;
} fn full_fields() -> Self where Self : Sized,
{ Self { age : true, id : true, name : true, } }
} #[derive(Default, Debug, Clone)] pub struct UserSelectedEntity
{ age : Option < i32 > , id : Option < i64 > , name : Option < String > , }
impl SelectedEntity < Sqlite > for UserSelectedEntity
{
    type Selection = UserSelection; fn
from_row(selection : & Self :: Selection, row : < Sqlite as Database > ::
Row) -> Result < Self, sqlx :: Error > where Self : Sized
    {
    let mut selected = Self :: default(); if selection.age
    { selected.age = row.try_get("age").ok(); }; if selection.id
    { selected.id = row.try_get("id").ok(); }; if selection.name
    { selected.name = row.try_get("name").ok(); }; Ok(selected)
    } fn from_row_full(row : < Sqlite as Database > :: Row) -> Result < Self,
    sqlx :: Error > where Self : Sized,
{
    let mut selected = Self :: default(); selected.age =
    row.try_get("age").ok(); ; selected.id = row.try_get("id").ok(); ;
    selected.name = row.try_get("name").ok(); ; Ok(selected)
}
}
