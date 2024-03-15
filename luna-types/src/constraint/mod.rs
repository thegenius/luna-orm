mod integer;
mod string;

pub use integer::IntegerConstraint;
pub use integer::IntegerConstraintBuilder;
use num_traits::NumCast;
use num_traits::PrimInt;
use num_traits::ToPrimitive;
pub use string::StringConstraint;
pub use string::StringConstraintBuilder;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::borrow::Cow;
use std::error::Error;
use std::fmt::Debug;
use std::fmt::Display;
use std::ops::Deref;

use crate::FieldType;

#[derive(Debug, Clone)]
pub struct ConstraintError<'a> {
    msg: Cow<'a, str>,
}

impl<'a> ConstraintError<'a> {
    pub fn new(msg: impl Into<Cow<'a, str>>) -> Self {
        Self { msg: msg.into() }
    }
}

impl<'a> Display for ConstraintError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl<'a> Error for ConstraintError<'a> {}

pub trait Constraint: Debug + Serialize {
    type ValueType;
    fn is_option(&self) -> bool;
    fn is_valid_json(&self, value: &Value) -> bool;
    fn is_valid(&self, value: &Self::ValueType) -> bool;
}

#[derive(Debug, Eq, Serialize, Deserialize, Clone)]
pub struct CachedConstraint<T: Constraint + Serialize> {
    #[serde(flatten)]
    constraint: T,
    #[serde(skip)]
    constraint_str: String,
}

impl<T: Constraint + Serialize> CachedConstraint<T> {
    pub fn cache_str(&mut self) {
        if self.constraint_str.is_empty() {
            let format_str = serde_json::to_string(&self.constraint)
                .unwrap_or("CONSTRAINT SERIALIZE ERROR".to_string());
            self.constraint_str = format_str;
        }
    }
    pub fn as_str(&self) -> &str {
        return &self.constraint_str;
    }
}
impl<T: Constraint + Serialize + PartialEq> PartialEq for CachedConstraint<T> {
    fn eq(&self, other: &Self) -> bool {
        self.constraint == other.constraint
    }
}

impl<T: Constraint + Serialize> From<T> for CachedConstraint<T> {
    fn from(value: T) -> Self {
        let format_str =
            serde_json::to_string(&value).unwrap_or("CONSTRAINT SERIALIZE ERROR".to_string());
        Self {
            constraint: value,
            constraint_str: format_str,
        }
    }
}

impl<T: Constraint + Serialize> Deref for CachedConstraint<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.constraint
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct NamedConstraint<T: Constraint + Serialize> {
    pub name: String,
    pub constraint: CachedConstraint<T>,
}

impl<T: Constraint + Serialize> Deref for NamedConstraint<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.constraint.constraint
    }
}

pub type NamedIntConstraint<T> = NamedConstraint<IntegerConstraint<T>>;
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ConstraintType<'a> {
    #[serde(rename = "smallint")]
    SmallInt(NamedIntConstraint<i16>),

    #[serde(alias = "int")]
    Int(NamedIntConstraint<i32>),

    #[serde(alias = "bigint")]
    BigInt(NamedIntConstraint<i64>),

    #[serde(alias = "smalluint")]
    SmallUInt(NamedIntConstraint<u16>),

    #[serde(alias = "uint")]
    UInt(NamedIntConstraint<u32>),

    #[serde(alias = "biguint")]
    BigUInt(NamedIntConstraint<u64>),

    #[serde(alias = "text")]
    Text(NamedConstraint<StringConstraint<'a>>),
}

impl<'a> ConstraintType<'a> {
    pub fn cache_str(&mut self) {
        match self {
            Self::SmallInt(val) => val.constraint.cache_str(),
            Self::Int(val) => val.constraint.cache_str(),
            Self::BigInt(val) => val.constraint.cache_str(),
            Self::SmallUInt(val) => val.constraint.cache_str(),
            Self::UInt(val) => val.constraint.cache_str(),
            Self::BigUInt(val) => val.constraint.cache_str(),
            Self::Text(val) => val.constraint.cache_str(),
        }
    }
    pub fn name(&self) -> &str {
        match self {
            Self::SmallInt(val) => &val.name,
            Self::Int(val) => &val.name,
            Self::BigInt(val) => &val.name,
            Self::SmallUInt(val) => &val.name,
            Self::UInt(val) => &val.name,
            Self::BigUInt(val) => &val.name,
            Self::Text(val) => &val.name,
        }
    }
}

/*
pub trait JsonConstraint {
    fn is_valid_json(&self, value: &Value) -> bool;
}

impl<T: Constraint> JsonConstraint for T {
    fn is_valid_json(&self, value: &Value) -> bool {
        self.is_valid_json(value)
    }
}
*/
