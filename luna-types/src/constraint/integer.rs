use super::Constraint;
use derive_builder::Builder;
use num_traits::int::PrimInt;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::Debug;

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Builder)]
#[builder(setter(into))]
pub struct IntegerConstraint<T: PrimInt + Serialize + Debug> {
    #[builder(default = "None")]
    min: Option<T>,
    #[builder(default = "None")]
    max: Option<T>,
}

#[inline]
fn value_to_primitive_int<T: PrimInt>(value: &Value) -> Option<T> {
    let raw_i64 = value.as_i64();
    if let Some(val) = raw_i64 {
        return T::from(val);
    }
    let raw_u64 = value.as_u64();
    if let Some(val) = raw_u64 {
        return T::from(val);
    }
    return None;
}

impl<T: PrimInt + Serialize + Debug> Constraint for IntegerConstraint<T> {
    type ValueType = T;

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
