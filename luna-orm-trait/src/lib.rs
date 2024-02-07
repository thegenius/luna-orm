#![allow(dead_code)]
#![allow(clippy::needless_return)]
use sqlx::any::AnyArguments;
use sqlx::any::AnyRow;

use sqlx::Any;
use sqlx::Column;
use sqlx::Encode;
use sqlx::Row;
use sqlx::Type;
use sqlx_core::any::AnyColumn;
use sqlx_core::any::AnyTypeInfo;
use sqlx_core::any::AnyTypeInfoKind;

use serde::{Deserialize, Serialize};
use serde_json::Map;
use serde_json::Value;

mod field;
mod field_type;
mod location;
mod parser;
mod request;
mod schema;
mod timer;
mod utils;
pub use field_type::{FieldType, NumericType};
pub use location::*;
pub use parser::ParsedTemplateSql;
pub use request::WriteCommand;
pub use schema::{ParsedSchema, SchemaDef};
use std::collections::HashMap;
use std::fmt::Debug;
pub use timer::Timer;
pub use utils::array_str_equal;

pub type SqlxError = sqlx::Error;

pub trait Schema {}

pub trait Primary: Sync + Debug {
    fn get_table_name(&self) -> &'static str;

    fn get_primary_field_names(&self) -> &'static [&'static str];

    fn any_arguments(&self) -> AnyArguments<'_>;
}

pub trait Mutation: Sync + Debug {
    fn any_arguments(&self) -> AnyArguments<'_>;

    fn get_fields_name(&self) -> Vec<String>;
}

pub trait Location: Sync + Debug {
    fn get_table_name(&self) -> &'static str;

    fn any_arguments(&self) -> AnyArguments<'_>;

    fn get_fields_name(&self) -> Vec<String>;

    fn get_where_clause(&self, wrap_char: char, place_holder: char) -> String;

    fn check_valid_order_by(&self, fields: &[&str]) -> bool;
}

pub trait Entity: Sync + Debug {
    fn get_table_name(&self) -> &str;

    fn get_insert_fields(&self) -> Vec<String>;

    fn get_upsert_set_fields(&self) -> Vec<String>;

    fn get_auto_increment_field(&self) -> Option<&str>;

    fn set_auto_increment_field(&mut self, value: Option<i64>) -> bool;

    fn any_arguments_of_insert(&self) -> AnyArguments<'_>;

    fn any_arguments_of_upsert(&self) -> AnyArguments<'_>;
}

pub trait Selection: Sync + Debug {
    fn get_table_name(&self) -> &'static str;

    fn get_selected_fields(&self) -> Vec<String>;
}

pub trait OrderBy: Sync + Debug {
    fn get_order_by_fields(&self) -> &'static [&'static str];
}

pub trait SelectedEntity: Debug {
    fn from_any_row(row: AnyRow) -> Result<Self, SqlxError>
    where
        Self: Sized;
}

#[derive(Debug, Clone)]
pub struct JsonResult {
    pub data: String,
}

impl SelectedEntity for JsonResult {
    fn from_any_row(row: AnyRow) -> Result<Self, SqlxError>
    where
        Self: Sized,
    {
        let record = convert_to_json(row);
        return Ok(JsonResult {
            data: record.to_string(),
        });
    }
}

pub fn convert_to_json(row: AnyRow) -> Value {
    let columns = row.columns();
    let mut value_map: Map<String, Value> = Map::new();
    for column in columns {
        let ordinal = column.ordinal();
        let name = column.name();
        let type_info = column.type_info();
        match type_info.kind() {
            AnyTypeInfoKind::SmallInt => {
                let data: i16 = row.get(ordinal);
                value_map.insert(name.to_string(), Value::Number(data.into()));
            }
            AnyTypeInfoKind::Integer => {
                let data: i32 = row.get(ordinal);
                value_map.insert(name.to_string(), Value::Number(data.into()));
            }
            AnyTypeInfoKind::BigInt => {
                let data: i64 = row.get(ordinal);
                value_map.insert(name.to_string(), Value::Number(data.into()));
            }
            AnyTypeInfoKind::Real => {
                let data: f32 = row.get(ordinal);
                value_map.insert(name.to_string(), data.into());
            }
            AnyTypeInfoKind::Double => {
                let data: f64 = row.get(ordinal);
                value_map.insert(name.to_string(), data.into());
            }
            AnyTypeInfoKind::Bool => {
                let data: bool = row.get(ordinal);
                value_map.insert(name.to_string(), data.into());
            }
            AnyTypeInfoKind::Blob => {
                let data: &[u8] = row.get(ordinal);
                value_map.insert(name.to_string(), data.into());
            }
            AnyTypeInfoKind::Text => {
                let data: String = row.get(ordinal);
                value_map.insert(name.to_string(), data.into());
            }
            AnyTypeInfoKind::Null => {
                value_map.insert(name.to_string(), Value::Null);
            }
        }
    }
    return Value::Object(value_map);
}

pub enum CountSql {
    Empty,
    PlainSql(String),
    VariabledSql(String),
}

pub trait TemplateRecord: Sync + Debug {
    fn get_sql(&self, page: Option<&Pagination>) -> String;

    fn get_count_sql(&self) -> CountSql;

    fn get_variables(&self) -> Vec<String>;

    fn any_arguments(&self) -> AnyArguments<'_>;
}

#[derive(Clone, Debug)]
pub struct RecordCount {
    pub count: i64,
}

impl SelectedEntity for RecordCount {
    fn from_any_row(row: AnyRow) -> Result<Self, SqlxError>
    where
        Self: Sized,
    {
        let count: i64 = row.try_get("count")?;
        Ok(Self { count })
    }
}

#[derive(Clone, Debug)]
pub struct LastRowId {
    pub id: i64,
}

impl SelectedEntity for LastRowId {
    fn from_any_row(row: AnyRow) -> Result<Self, SqlxError>
    where
        Self: Sized,
    {
        let last_row_id: i64 = row.try_get("last_row_id")?;
        Ok(Self { id: last_row_id })
    }
}

#[derive(Clone, Debug)]
pub struct Pagination {
    pub page_size: usize,
    pub page_num: usize,
}

#[derive(Clone, Debug)]
pub struct PageInfo {
    pub page_size: usize,
    pub page_num: usize,
    pub page_total: usize,
    pub total: usize,
}

impl PageInfo {
    pub fn empty(page_size: usize, page_num: usize) -> Self {
        Self {
            page_size,
            page_num,
            page_total: 0,
            total: 0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct PagedList<T>
where
    T: SelectedEntity,
{
    pub data: Vec<T>,
    pub page: PageInfo,
}

impl<T> PagedList<T>
where
    T: SelectedEntity,
{
    pub fn empty(page_size: usize, page_num: usize) -> Self {
        Self {
            page: PageInfo::empty(page_size, page_num),
            data: Vec::new(),
        }
    }
}

//pub fn merge_any_arguments<'p>(
pub fn luna_merge_args<'p>(
    mut args_a: AnyArguments<'p>,
    args_b: AnyArguments<'p>,
) -> AnyArguments<'p> {
    args_a.values.0.extend(args_b.values.0);
    args_a
}

//pub fn add_arg<'q, T>(args: &mut AnyArguments<'q>, value: &T)
pub fn luna_add_arg<'q, T>(args: &mut AnyArguments<'q>, value: &T)
where
    T: 'q + Send + Encode<'q, Any> + Type<Any>,
{
    let _ = value.encode_by_ref(&mut args.values);
}
