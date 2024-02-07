use num::cast::NumCast;
use num_traits::{PrimInt, ToPrimitive};
use serde::Serialize;
use std::collections::HashMap;
use std::convert::TryFrom;

use super::Constraint;
use crate::constraint::{self, ConstraintError};
use crate::{CachedConstraint, ConstraintType, NamedConstraint};

use std::borrow::Cow;
mod integer;
mod string;

pub use integer::Integer;
use serde::Deserialize;
use serde_json::Value;
pub use string::Text;

pub trait ValidField {
    type ValueType;
    type ConstraintType: Constraint<ValueType = Self::ValueType>;

    fn try_from_valid<'a, 'b>(
        value: &'a Self::ValueType,
        constraint: &'b CachedConstraint<Self::ConstraintType>,
    ) -> Result<&'a Self::ValueType, ConstraintError<'b>> {
        let valid = constraint.is_valid(value);
        if valid {
            return Ok(value);
        } else {
            let err_msg = format!("constraint: {} not match", constraint.as_str());
            let err = ConstraintError::new(err_msg);
            return Err(err);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NamedFieldType<'a> {
    pub name: Cow<'a, str>,
    pub field: FieldType<'a>,
}

impl<'a> NamedFieldType<'a> {
    pub fn new(name: impl Into<Cow<'a, str>>, field: FieldType<'a>) -> Self {
        Self {
            name: name.into(),
            field,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FieldType<'a> {
    SmallInt(Integer<i16>),
    Int(Integer<i32>),
    BigInt(Integer<i64>),
    SmallUInt(Integer<u16>),
    UInt(Integer<u32>),
    BigUInt(Integer<u64>),
    Text(Text<'a>),
}

pub fn try_from_json<'a>(
    value: &'a Value,
    constraint: &'a ConstraintType,
) -> Result<FieldType<'a>, ConstraintError<'a>> {
    match constraint {
        ConstraintType::SmallInt(named_constraint) => {
            let val = named_constraint.get_valid_from_json(value)?;
            return Ok(FieldType::SmallInt(val.into()));
        }
        ConstraintType::Int(named_constraint) => {
            let val = named_constraint.get_valid_from_json(value)?;
            return Ok(FieldType::Int(val.into()));
        }
        ConstraintType::BigInt(named_constraint) => {
            let val = named_constraint.get_valid_from_json(value)?;
            return Ok(FieldType::BigInt(val.into()));
        }
        ConstraintType::SmallUInt(named_constraint) => {
            let val = named_constraint.get_valid_from_json(value)?;
            return Ok(FieldType::SmallUInt(val.into()));
        }
        ConstraintType::UInt(named_constraint) => {
            let val = named_constraint.get_valid_from_json(value)?;
            return Ok(FieldType::UInt(val.into()));
        }
        ConstraintType::BigUInt(named_constraint) => {
            let val = named_constraint.get_valid_from_json(value)?;
            return Ok(FieldType::BigUInt(val.into()));
        }
        ConstraintType::Text(cons) => {
            let data: Option<&str> = value.as_str();
            if let Some(data) = data {
                let data = Text::from_valid(data, &cons.constraint)?;
                return Ok(FieldType::Text(data));
            } else {
                return Err(ConstraintError::new("no string found in json"));
            }
        }
    }
}
impl<'a> FieldType<'a> {
    pub fn from_json(
        value: &'a Value,
        constraint: &'a ConstraintType,
    ) -> Result<Self, ConstraintError<'a>> {
        try_from_json(value, constraint)
    }
}
