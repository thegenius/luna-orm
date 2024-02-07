use luna_types::CachedConstraint;
use luna_types::ConstraintType;
use luna_types::Integer;
use luna_types::IntegerConstraint;
use luna_types::IntegerConstraintBuilder;
use luna_types::ValidField;
use sqlx::ConnectOptions;

#[test]
fn test_field_of_int() {
    let expect_int_constraint: IntegerConstraint<i16> = IntegerConstraintBuilder::default()
        .min(23i16)
        .build()
        .unwrap();
    let expect: CachedConstraint<IntegerConstraint<i16>> = expect_int_constraint.into();
    let expect_type = ConstraintType::SmallInt(expect);
    let expect_str = serde_json::to_string(&expect_type).unwrap();
    dbg!(&expect_str);

    let data = r#"{ "type": "smallint", "constraint": { "min": 23 } }"#;
    let mut constraint: ConstraintType = serde_json::from_str(data).unwrap();
    constraint.cache_str();

    assert_eq!(constraint, expect_type);
}
