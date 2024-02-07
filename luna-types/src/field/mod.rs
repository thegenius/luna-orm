use std::collections::HashMap;

use super::Constraint;
use crate::constraint::{ConstraintError, JsonConstraint};
use crate::CachedConstraint;

mod integer;
mod string;

pub use integer::Integer;
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

pub enum FieldType<'a> {
    SmallInt(Integer<i16>),
    Int(Integer<i32>),
    BigInt(Integer<i64>),
    Text(Text<'a>),
}

impl<'a> FieldType<'a> {
    pub fn from_json(value: &Value, field_constraints: HashMap<String, Box<dyn JsonConstraint>>) {
        return ();
    }
}
