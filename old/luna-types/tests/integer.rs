use luna_types::constraint::supported::Constraint;
use luna_types::constraint::common::ConstraintTrait;
use luna_types::constraint::supports::integer:: IntegerConstraint;
use luna_types::constraint::supports::integer::IntegerConstraintBuilder;
use serde_json::{Number, Value};

#[test]
pub fn test_min_max() {
    let mut builder = IntegerConstraintBuilder::default();
    let constraint = builder.min(10).max(20).build().unwrap();
    let value = Value::Number(Number::from(15));
    assert!(constraint.is_valid_json(&value));
    let value = Value::Number(Number::from(22));
    assert_eq!(false, constraint.is_valid_json(&value));
}

#[test]
pub fn test_deserialize_min() {
    let mut builder = IntegerConstraintBuilder::default();
    let constraint_a = builder.min(10).build().unwrap();
    let constraint_b = serde_json::from_str(r#" {"min": 10} "#).unwrap();
    assert_eq!(constraint_a, constraint_b);
}

#[test]
pub fn test_deserialize_max() {
    let mut builder = IntegerConstraintBuilder::default();
    let constraint_a = builder.max(20).build().unwrap();
    let constraint_b = serde_json::from_str(r#" {"max": 20} "#).unwrap();
    assert_eq!(constraint_a, constraint_b);
}

#[test]
pub fn test_deserialize_min_max() {
    let mut builder = IntegerConstraintBuilder::default();
    let constraint_a = builder.min(10).max(20).build().unwrap();
    let constraint_b = serde_json::from_str(r#" {"min": 10, "max": 20} "#).unwrap();
    assert_eq!(constraint_a, constraint_b);
}
