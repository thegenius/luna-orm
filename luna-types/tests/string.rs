use luna_types::constraint::supported::Constraint;
use luna_types::constraint::common::ConstraintTrait;
use luna_types::constraint::supports::string::StringConstraint;
use luna_types::constraint::supports::string::StringConstraintBuilder;
use serde_json::{Number, Value};

#[test]
pub fn test_min_max() {
    let mut builder = StringConstraintBuilder::default();
    let constraint = builder.min_len(5).max_len(8).build().unwrap();
    let value = Value::String("hello".to_string());
    assert!(constraint.is_valid_json(&value));
    let value = Value::String("hello world".to_string());
    assert_eq!(false, constraint.is_valid_json(&value));
}

#[test]
pub fn test_regex() {
    let mut builder = StringConstraintBuilder::default();
    let constraint = builder
        .min_len(5)
        .max_len(8)
        .regex("^hello")
        .build()
        .unwrap();
    let value = Value::String("hello".to_string());
    assert!(constraint.is_valid_json(&value));
    let value = Value::String("hello world".to_string());
    assert_eq!(false, constraint.is_valid_json(&value));
    let value = Value::String("test".to_string());
    assert_eq!(false, constraint.is_valid_json(&value));
}
