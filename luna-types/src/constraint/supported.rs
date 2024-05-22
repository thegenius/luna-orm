

use crate::constraint::common::ConstraintTrait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::borrow::Cow;
use std::fmt::Debug;
use std::fmt::Display;
use std::ops::Deref;

use super::supports::integer::IntegerConstraint;
use super::supports::string::StringConstraint;
use super::supports::datetime::DateTimeConstraint;
use crate::field::supported::Field;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Constraint<'a> {
    #[serde(rename = "smallint")]
    SmallInt(IntegerConstraint<i16>),

    #[serde(alias = "int")]
    Int(IntegerConstraint<i32>),

    #[serde(alias = "bigint")]
    BigInt(IntegerConstraint<i64>),

    #[serde(alias = "smalluint")]
    SmallUInt(IntegerConstraint<u16>),

    #[serde(alias = "uint")]
    UInt(IntegerConstraint<u32>),

    #[serde(alias = "biguint")]
    BigUInt(IntegerConstraint<u64>),

    #[serde(alias = "text")]
    Text(StringConstraint<'a>),

    #[serde(alias = "datatime")]
    DateTime(DateTimeConstraint),
}

impl<'a> ConstraintTrait for Constraint<'a> {
    type ValueType = Field<'a>;
    fn is_option(&self) -> bool {
        match self {
            Self::SmallInt(val) => val.is_option(),
            Self::Int(val) => val.is_option(),
            Self::BigInt(val) => val.is_option(),
            Self::SmallUInt(val) => val.is_option(),
            Self::UInt(val) => val.is_option(),
            Self::BigUInt(val) => val.is_option(),
            Self::Text(val) => val.is_option(),
            Self::DateTime(val) => val.is_option()
        }
    }
    fn is_valid(&self, value: &Self::ValueType) -> bool {
        match self {
            Self::SmallInt(cons) => {
                if let Field::SmallInt(val) = value {
                    return cons.is_valid(val);
                }
                return false;
            },
            Self::Int(cons) => {
                if let Field::Int(val) = value {
                    return cons.is_valid(val);
                }
                return false;
            },
            Self::BigInt(cons) => {
                if let Field::BigInt(val) = value {
                    return cons.is_valid(val);
                }
                return false;
            },
            Self::SmallUInt(cons) => {
                if let Field::SmallUInt(val) = value {
                    return cons.is_valid(val);
                }
                return false;
            },
            Self::UInt(cons) => {
                if let Field::UInt(val) = value {
                    return cons.is_valid(val);
                }
                return false;
            },
            Self::BigUInt(cons) => {
                if let Field::BigUInt(val) = value {
                    return cons.is_valid(val);
                }
                return false;
            }
            Self::Text(cons) => {
                if let Field::Text(val) = value {
                    return cons.is_valid(val);
                }
                return false;
            }   
            Self::DateTime(cons) => {
                if let Field::DateTime(val) = value {
                    return cons.is_valid(val);
                }
                return false;
            }    
        }
    }

    fn is_valid_json(&self, value: &Value) -> bool {
        match self {
            Self::SmallInt(val) => val.is_valid_json(value),
            Self::Int(val) => val.is_valid_json(value),
            Self::BigInt(val) => val.is_valid_json(value),
            Self::SmallUInt(val) => val.is_valid_json(value),
            Self::UInt(val) => val.is_valid_json(value),
            Self::BigUInt(val) => val.is_valid_json(value),
            Self::Text(val) => val.is_valid_json(value),
            Self::DateTime(val) => val.is_valid_json(value)
        }
    }
}