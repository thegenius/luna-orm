use luna_types::constraint::named:: NamedConstraint;
use luna_types::constraint::supported::Constraint;
use luna_types::constraint::supports::integer::IntegerConstraint;
use luna_types::constraint::supports::integer::IntegerConstraintBuilder;

#[test]
fn test_constraint_serialize_deserialize() {
    let int_constraint: IntegerConstraint<i16> = IntegerConstraintBuilder::default()
        .min(23i16)
        .build()
        .unwrap();

    let constraint_type = Constraint::SmallInt(int_constraint);
    let constraint_type_serde_str = serde_json::to_string(&constraint_type).unwrap();
    let expect_str = "{\"type\":\"smallint\",\"is_option\":null,\"min\":23,\"max\":null}";
    assert_eq!(constraint_type_serde_str, expect_str);

    let named_cons: NamedConstraint = NamedConstraint::from_named("hello".to_string(), constraint_type.clone());
    let named_cons_str = serde_json::to_string(&named_cons).unwrap();
    let named_cons_expect_str = "{\"name\":\"hello\",\"constraint\":{\"type\":\"smallint\",\"is_option\":null,\"min\":23,\"max\":null}}";
    assert_eq!(named_cons_str, named_cons_expect_str);
}
