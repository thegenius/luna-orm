#![allow(dead_code)]
use sqlx::any::AnyArguments;
use sqlx::any::AnyRow;
use sqlx::Any;
use sqlx::Encode;
use sqlx::Row;
use sqlx::Type;

use serde::{Deserialize, Serialize};

mod field;
mod location;
mod request;
mod utils;
pub use location::*;
pub use request::WriteCommand;
pub use utils::array_str_equal;

pub type SqlxError = sqlx::Error;

pub trait Primary: Sync {
    fn get_table_name(&self) -> &'static str;

    fn get_primary_field_names(&self) -> &'static [&'static str];

    fn any_arguments(&self) -> AnyArguments<'_>;
}

pub trait Mutation: Sync {
    fn any_arguments<'p>(&self) -> AnyArguments<'p>;

    fn get_fields_name(&self) -> Vec<String>;
}

pub trait Location: Sync {
    fn get_table_name(&self) -> &'static str;

    fn any_arguments<'p>(&self) -> AnyArguments<'p>;

    fn get_fields_name(&self) -> Vec<String>;

    fn get_where_clause(&self, wrap_char: char, place_holder: char) -> String;

    fn check_valid_order_by(&self, fields: &[&str]) -> bool;
}

pub trait Entity: Sync {
    fn get_table_name(&self) -> &'static str;

    fn get_primary_fields_name(&self) -> Vec<String>;

    fn get_body_fields_name(&self) -> Vec<String>;

    fn any_arguments_of_insert<'p>(&self) -> AnyArguments<'p>;

    fn any_arguments_of_upsert<'p>(&self) -> AnyArguments<'p>;

    fn any_arguments_of_update<'p>(&self) -> AnyArguments<'p>;

    fn from_any_row(row: AnyRow) -> Result<Self, SqlxError>
    where
        Self: Sized;
}

pub trait Selection: Sync {
    fn get_selected_fields(&self) -> Vec<String>;
}

pub trait OrderBy: Sync {
    fn get_order_by_fields(&self) -> &'static [&'static str];
}

pub trait SelectedEntity {
    fn from_any_row(row: AnyRow) -> Result<Self, SqlxError>
    where
        Self: Sized;
}

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

pub fn merge_any_arguments<'p>(
    mut args_a: AnyArguments<'p>,
    args_b: AnyArguments<'p>,
) -> AnyArguments<'p> {
    args_a.values.0.extend(args_b.values.0);
    args_a
}

pub fn add_arg<'q, T>(args: &mut AnyArguments<'q>, value: &T)
where
    T: 'q + Send + Encode<'q, Any> + Type<Any>,
{
    let _ = value.encode_by_ref(&mut args.values);
}
