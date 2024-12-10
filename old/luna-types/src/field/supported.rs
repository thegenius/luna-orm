
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use serde_json::Value;
use crate::constraint::common::ConstraintTrait;
use crate::constraint::error::ConstraintError;
use crate::field::supports::integer::Integer;
use crate::field::supports::string::Text;
use crate::constraint::supported::Constraint;
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Field<'a> {
    SmallInt(Integer<i16>),
    Int(Integer<i32>),
    BigInt(Integer<i64>),
    SmallUInt(Integer<u16>),
    UInt(Integer<u32>),
    BigUInt(Integer<u64>),
    Text(Text<'a>),
    DateTime(DateTime<Utc>)
}

impl From<i16> for Field<'_> {
    fn from(value: i16) -> Self {
        Self::SmallInt(Integer::from(value))
    }
}

impl From<i32> for Field<'_> {
    fn from(value: i32) -> Self {
        Self::Int(Integer::from(value))
    }
}
impl From<i64> for Field<'_> {
    fn from(value: i64) -> Self {
        Self::BigInt(Integer::from(value))
    }
}
impl From<u16> for Field<'_> {
    fn from(value: u16) -> Self {
        Self::SmallUInt(Integer::from(value))
    }
}
impl From<u32> for Field<'_> {
    fn from(value: u32) -> Self {
        Self::UInt(Integer::from(value))
    }
}
impl From<u64> for Field<'_> {
    fn from(value: u64) -> Self {
        Self::BigUInt(Integer::from(value))
    }
}
impl<'a> From<&'a str> for Field<'a> {
    fn from(value: &'a str) -> Field<'a> {
        Self::Text(Text::from(value))
    }
}



pub fn try_from_json<'a>(
    value: &'a Value,
    constraint: &'a Constraint<'a>,
) -> Result<Field<'a>, ConstraintError<'a>> {
    match constraint {
        Constraint::SmallInt(named_constraint) => {
            let val = named_constraint.get_valid_from_json(value)?;
            return Ok(Field::SmallInt(val.into()));
        }
        Constraint::Int(named_constraint) => {
            let val = named_constraint.get_valid_from_json(value)?;
            return Ok(Field::Int(val.into()));
        }
        Constraint::BigInt(named_constraint) => {
            let val = named_constraint.get_valid_from_json(value)?;
            return Ok(Field::BigInt(val.into()));
        }
        Constraint::SmallUInt(named_constraint) => {
            let val = named_constraint.get_valid_from_json(value)?;
            return Ok(Field::SmallUInt(val.into()));
        }
        Constraint::UInt(named_constraint) => {
            let val = named_constraint.get_valid_from_json(value)?;
            return Ok(Field::UInt(val.into()));
        }
        Constraint::BigUInt(named_constraint) => {
            let val = named_constraint.get_valid_from_json(value)?;
            return Ok(Field::BigUInt(val.into()));
        }
        Constraint::Text(cons) => {
            let data: Option<&str> = value.as_str();
            if let Some(data) = data {
                let cow_data: Cow<'_, str> = data.into();
                if cons.is_valid(&cow_data) {
                    return Err(ConstraintError::new("text not valid"));
                }
                let text: Text<'_> = Text(cow_data);
                return Ok(Field::Text(text));
            } else {
                return Err(ConstraintError::new("no string found in json"));
            }
        }
        Constraint::DateTime(cons) => {
            let datetime: Result<DateTime<Utc>, serde_json::Error> = serde_json::from_value(value.clone());
            if let Ok(datetime) = datetime {
                if cons.is_valid(&datetime) {
                    return Ok(Field::DateTime(datetime));
                }
            }
            return Err(ConstraintError::new("datatime not valid"))
        }
    }
}
impl<'a> Field<'a> {
    pub fn from_json(
        value: &'a Value,
        constraint: &'a Constraint<'a>,
    ) -> Result<Self, ConstraintError<'a>> {
        try_from_json(value, constraint)
    }
}