use serde::Serialize;
use serde_json::Value;
use std::fmt::Debug;

pub trait ConstraintTrait: Debug + Serialize {
    type ValueType;

    fn is_option(&self) -> bool;

    // for dynamic json value
    fn is_valid_json(&self, value: &Value) -> bool;

    // for specific type fo value
    fn is_valid(&self, value: &Self::ValueType) -> bool;
}
