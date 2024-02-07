mod integer;
mod string;

pub use integer::IntegerConstraint;
pub use integer::IntegerConstraintBuilder;
use num_traits::PrimInt;
pub use string::StringConstraint;
pub use string::StringConstraintBuilder;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::borrow::Cow;
use std::error::Error;
use std::fmt::Debug;
use std::fmt::Display;
use std::ops::Deref;

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
    fn is_valid_json(&self, value: &Value) -> bool;
    fn is_valid(&self, value: &Self::ValueType) -> bool;
}

pub type CachedIntConstraint<T> = CachedConstraint<IntegerConstraint<T>>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "constraint")]
pub enum ConstraintType<'a> {
    #[serde(rename = "smallint")]
    SmallInt(CachedIntConstraint<i16>),

    #[serde(alias = "int")]
    Int(CachedIntConstraint<i32>),

    #[serde(alias = "bigint")]
    BigInt(CachedIntConstraint<i64>),

    #[serde(alias = "smalluint")]
    SmallUInt(CachedIntConstraint<u16>),

    #[serde(alias = "uint")]
    UInt(CachedIntConstraint<u32>),

    #[serde(alias = "biguint")]
    BigUInt(CachedIntConstraint<u64>),

    #[serde(alias = "text")]
    Text(StringConstraint<'a>),
}

impl<'a> ConstraintType<'a> {
    pub fn cache_str(&mut self) {
        match self {
            Self::SmallInt(val) => val.cache_str(),
            Self::Int(val) => val.cache_str(),
            Self::BigInt(val) => val.cache_str(),
            Self::SmallUInt(val) => val.cache_str(),
            Self::UInt(val) => val.cache_str(),
            Self::BigUInt(val) => val.cache_str(),
            Self::Text(_) => {}
        }
    }
}

pub trait JsonConstraint {
    fn is_valid_json(&self, value: &Value) -> bool;
}

impl<T: Constraint> JsonConstraint for T {
    fn is_valid_json(&self, value: &Value) -> bool {
        self.is_valid_json(value)
    }
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
