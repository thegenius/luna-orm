use luna_types::FieldType;
use serde_json::Value;

#[test]
fn test_field() {
    let data_str = r#"{  }"#;
    let value: Value = serde_json::from_str(data_str).unwrap();

    assert!(true);
}
