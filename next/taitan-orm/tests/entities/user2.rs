
use sqlx::error::BoxDynError;
use sqlx::mysql::MySqlArguments;
use sqlx::postgres::PgArguments;
use sqlx::sqlite::SqliteArguments;
use sqlx::{sqlx_macros};

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
    self.age { sqlx::Arguments::add(&mut args, age) ? ; } sqlx::Arguments::add(&mut args,& self.name) ? ; Ok(args)
} fn gen_upsert_arguments_sqlite(& self) -> Result < SqliteArguments < '_
> , BoxDynError >
{
    let mut args = SqliteArguments :: default(); if let Some(age) = &
    self.age { sqlx::Arguments::add(&mut args,age) ? ; } sqlx::Arguments::add(&mut args,& self.name) ? ; if let
    Some(age) = & self.age { sqlx::Arguments::add(&mut args,age) ? ; } sqlx::Arguments::add(&mut args,& self.name) ? ;
    Ok(args)
} fn gen_insert_arguments_mysql(& self) -> Result < MySqlArguments,
    BoxDynError >
{
    let mut args = MySqlArguments :: default(); if let Some(age) = &
    self.age { sqlx::Arguments::add(&mut args,age) ? ; } sqlx::Arguments::add(&mut args,& self.name) ? ; Ok(args)
} fn gen_upsert_arguments_mysql(& self) -> Result < MySqlArguments,
    BoxDynError >
{
    let mut args = MySqlArguments :: default(); if let Some(age) = &
    self.age { sqlx::Arguments::add(&mut args,age) ? ; } sqlx::Arguments::add(&mut args,& self.name) ? ; if let
    Some(age) = & self.age { sqlx::Arguments::add(&mut args,age) ? ; } sqlx::Arguments::add(&mut args,& self.name) ? ;
    Ok(args)
} fn gen_insert_arguments_postgres(& self) -> Result < PgArguments,
    BoxDynError >
{
    let mut args = PgArguments :: default(); if let Some(age) = & self.age
{ sqlx::Arguments::add(&mut args,age) ? ; } sqlx::Arguments::add(&mut args,& self.name) ? ; Ok(args)
} fn gen_upsert_arguments_postgres(& self) -> Result < PgArguments,
    BoxDynError >
{
    let mut args = PgArguments :: default(); if let Some(age) = & self.age
{ sqlx::Arguments::add(&mut args,age) ? ; } sqlx::Arguments::add(&mut args,& self.name) ? ; if let Some(age) = &
    self.age { sqlx::Arguments::add(&mut args,age) ? ; } sqlx::Arguments::add(&mut args,& self.name) ? ; Ok(args)
}
} #[derive(Default, Debug, Clone)] pub struct UserPrimary { id : i64, } impl
Unique for UserPrimary
{
    fn get_table_name(& self) -> & 'static str { "user" } fn
get_unique_field_names(& self) -> & 'static [& 'static str] { & ["id",] }
    fn gen_unique_arguments_sqlite(& self) -> Result < SqliteArguments < '_ >
        , BoxDynError >
    {
        let mut args = SqliteArguments :: default(); sqlx::Arguments::add(&mut args,& self.id) ? ;
        Ok(args)
    } fn gen_unique_arguments_mysql(& self) -> Result < MySqlArguments,
    BoxDynError >
{
    let mut args = MySqlArguments :: default(); sqlx::Arguments::add(&mut args,& self.id) ? ;
    Ok(args)
} fn gen_unique_arguments_postgres(& self) -> Result < PgArguments,
    BoxDynError >
{
    let mut args = PgArguments :: default(); sqlx::Arguments::add(&mut args,& self.id) ? ;
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
    self.age { sqlx::Arguments::add(&mut args,& age.val) ? ; } if let Some(id) = & self.id
{ sqlx::Arguments::add(&mut args,& id.val) ? ; } if let Some(name) = & self.name
{ sqlx::Arguments::add(&mut args,& name.val) ? ; } Ok(args)
} fn gen_location_arguments_mysql(& self) -> Result < MySqlArguments,
    BoxDynError >
{
    let mut args = MySqlArguments :: default(); if let Some(age) = &
    self.age { sqlx::Arguments::add(&mut args,& age.val) ? ; } if let Some(id) = & self.id
{ sqlx::Arguments::add(&mut args,& id.val) ? ; } if let Some(name) = & self.name
{ sqlx::Arguments::add(&mut args,& name.val) ? ; } Ok(args)
} fn gen_location_arguments_postgres(& self) -> Result < PgArguments,
    BoxDynError >
{
    let mut args = PgArguments :: default(); if let Some(age) = & self.age
{ sqlx::Arguments::add(&mut args,& age.val) ? ; } if let Some(id) = & self.id
{ sqlx::Arguments::add(&mut args,& id.val) ? ; } if let Some(name) = & self.name
{ sqlx::Arguments::add(&mut args,& name.val) ? ; } Ok(args)
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
    self.age { sqlx::Arguments::add(&mut args,age) ? ; } if let Some(name) = & self.name
{ sqlx::Arguments::add(&mut args,name) ? ; } sqlx::Arguments::add(&mut args,& primary.id) ? ; Ok(args)
} fn gen_update_arguments_mysql < 'a >
(& 'a self, primary : & 'a Self :: Primary,) -> Result < MySqlArguments,
        BoxDynError >
{
    let mut args = MySqlArguments :: default(); if let Some(age) = &
    self.age { sqlx::Arguments::add(&mut args,age) ? ; } if let Some(name) = & self.name
{ sqlx::Arguments::add(&mut args,name) ? ; } sqlx::Arguments::add(&mut args,& primary.id) ? ; Ok(args)
} fn gen_update_arguments_postgres < 'a >
(& 'a self, primary : & 'a Self :: Primary,) -> Result < PgArguments,
        BoxDynError >
{
    let mut args = PgArguments :: default(); if let Some(age) = & self.age
{ sqlx::Arguments::add(&mut args,age) ? ; } if let Some(name) = & self.name
{ sqlx::Arguments::add(&mut args,name) ? ; } sqlx::Arguments::add(&mut args,& primary.id) ? ; Ok(args)
} fn gen_change_arguments_sqlite < 'a >
(& 'a self, location : & 'a Self :: Location,) -> Result < SqliteArguments
    < 'a > , BoxDynError >
{
    let mut args = SqliteArguments :: default(); if let Some(age) = &
    self.age { sqlx::Arguments::add(&mut args,age) ? ; } if let Some(name) = & self.name
{ sqlx::Arguments::add(&mut args,name) ? ; } if let Some(age) = & location.age
{ sqlx::Arguments::add(&mut args,& age.val) ? ; } if let Some(id) = & location.id
{ sqlx::Arguments::add(&mut args,& id.val) ? ; } if let Some(name) = & location.name
{ sqlx::Arguments::add(&mut args,& name.val) ? ; } Ok(args)
} fn gen_change_arguments_mysql < 'a >
(& 'a self, location : & 'a Self :: Location,) -> Result < MySqlArguments,
        BoxDynError >
{
    let mut args = MySqlArguments :: default(); if let Some(age) = &
    self.age { sqlx::Arguments::add(&mut args,age) ? ; } if let Some(name) = & self.name
{ sqlx::Arguments::add(&mut args,name) ? ; } if let Some(age) = & location.age
{ sqlx::Arguments::add(&mut args,& age.val) ? ; } if let Some(id) = & location.id
{ sqlx::Arguments::add(&mut args,& id.val) ? ; } if let Some(name) = & location.name
{ sqlx::Arguments::add(&mut args,& name.val) ? ; } Ok(args)
} fn gen_change_arguments_postgres < 'a >
(& 'a self, location : & 'a Self :: Location,) -> Result < PgArguments,
        BoxDynError >
{
    let mut args = PgArguments :: default(); if let Some(age) = & self.age
{ sqlx::Arguments::add(&mut args,age) ? ; } if let Some(name) = & self.name
{ sqlx::Arguments::add(&mut args,name) ? ; } if let Some(age) = & location.age
{ sqlx::Arguments::add(&mut args,& age.val) ? ; } if let Some(id) = & location.id
{ sqlx::Arguments::add(&mut args,& id.val) ? ; } if let Some(name) = & location.name
{ sqlx::Arguments::add(&mut args,& name.val) ? ; } Ok(args)
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
    { selected.age = sqlx::Row::try_get(&row, "age").ok(); }; if selection.id
    { selected.id = sqlx::Row::try_get(&row,"id").ok(); }; if selection.name
    { selected.name = sqlx::Row::try_get(&row,"name").ok(); }; Ok(selected)
    } fn from_row_full(row : < Sqlite as Database > :: Row) -> Result < Self,
    sqlx :: Error > where Self : Sized,
{
    let mut selected = Self :: default(); selected.age =
    sqlx::Row::try_get(&row,"age").ok(); ; selected.id = sqlx::Row::try_get(&row,"id").ok(); ;
    selected.name = sqlx::Row::try_get(&row,"name").ok(); ; Ok(selected)
}
}

#[derive(Debug, Default)] pub struct UserOrdering < 'a >
{ fields : Vec < std::borrow::Cow < 'a, str >> , } impl < 'a > taitan_orm :: traits ::
OrderBy for UserOrdering < 'a >
{
    fn unique_fields(& self) -> & [& [& str]] { & [& ["id"]] } fn
get_fields(& self) -> & [std::borrow::Cow < 'a, str >] { & self.fields }
} impl < 'a > UserOrdering < 'a >
{
    pub fn build<I, S>(fields: I) -> Result<Self, Box<dyn std::
    error::Error + 'static>>
    where
        I: IntoIterator<Item=S> + Clone,
        S
        : AsRef<str> + Into<std::borrow::Cow<'a, str>>,
    {
        let order_by = Self::default();
        taitan_orm::traits::
        validate_order_by(fields.clone(), taitan_orm::traits::OrderBy::unique_fields(&order_by))?;
        Ok(Self { fields: fields.into_iter().map(Into::into).collect(), })
    }
}