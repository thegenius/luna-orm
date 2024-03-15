use super::Constraint;
use super::ConstraintError;
use derive_builder::Builder;
use num::NumCast;
use num::ToPrimitive;
use num_traits::int::PrimInt;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::Debug;

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Builder)]
#[builder(setter(into))]
pub struct IntegerConstraint<T: PrimInt + Serialize + Debug> {
    #[builder(default = "None")]
    #[serde(default)]
    is_option: Option<bool>,

    #[builder(default = "None")]
    #[serde(default)]
    min: Option<T>,

    #[builder(default = "None")]
    #[serde(default)]
    max: Option<T>,
}

#[inline]
fn value_to_primitive_int<T: NumCast>(value: &Value) -> Option<T> {
    let raw_i64 = value.as_i64();
    if let Some(val) = raw_i64 {
        return NumCast::from(val);
    }
    let raw_u64 = value.as_u64();
    if let Some(val) = raw_u64 {
        return NumCast::from(val);
    }
    return None;
}

impl<T: PrimInt + Serialize + Debug> IntegerConstraint<T> {
    pub fn get_valid_num<'a, S: ToPrimitive + NumCast>(
        &self,
        value: S,
    ) -> Result<T, ConstraintError<'a>> {
        let data: Option<T> = NumCast::from(value);
        return self.check_valid(data);
    }

    pub fn get_valid_from_json<'a>(&self, value: &Value) -> Result<T, ConstraintError<'a>> {
        let data: Option<T> = value_to_primitive_int(value);
        return self.check_valid(data);
    }

    pub fn check_valid<'a>(&self, value: Option<T>) -> Result<T, ConstraintError<'a>> {
        if let Some(data) = value {
            let valid = self.is_valid(&data);
            if valid {
                return Ok(data);
            } else {
                return Err(ConstraintError::new("int constraint check fail"));
            }
        }
        return Err(ConstraintError::new("int cast error, maybe overflow."));
    }

    pub fn is_valid_num<S: ToPrimitive + NumCast>(&self, value: S) -> bool {
        let data: Option<T> = NumCast::from(value);
        return data.map_or(false, |e| self.is_valid(&e));
    }
}
impl<T: PrimInt + Serialize + Debug> Constraint for IntegerConstraint<T> {
    type ValueType = T;

    fn is_option(&self) -> bool {
        return self.is_option.unwrap_or(false);
    }

    fn is_valid_json(&self, value: &Value) -> bool {
        let data_opt: Option<T> = value_to_primitive_int(value);
        if data_opt.is_none() {
            return false;
        }
        let data = data_opt.unwrap();
        return self.is_valid(&data);
    }

    fn is_valid(&self, value: &T) -> bool {
        let value = *value;
        if let Some(min) = self.min {
            if value < min {
                return false;
            }
        }
        if let Some(max) = self.max {
            if value > max {
                return false;
            }
        }
        return true;
    }
}
