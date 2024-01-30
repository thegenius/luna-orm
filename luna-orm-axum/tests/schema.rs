use luna_orm_axum::Field;
use luna_orm_axum::FieldType;
use luna_orm_axum::Index;
use luna_orm_axum::IndexType;
use luna_orm_axum::Schema;

#[test]
pub fn test_serailize_index() {
    let index_type = IndexType::Normal;
    let serailized = serde_yaml::to_string(&index_type).unwrap();
    assert_eq!(serailized, "normal\n");
}

#[test]
pub fn test_serailize() {
    let schema = Schema {
        name: "article".to_string(),
        fields: vec![Field {
            f_name: "a".to_string(),
            f_type: FieldType::Cellphone,
            f_constraint: "".to_string(),
        }],
        indexes: vec![Index {
            i_name: "i_test".to_string(),
            i_type: IndexType::Normal,
            f_names: vec!["name".to_string()],
        }],
    };
    let schema_str: String = serde_yaml::to_string(&schema).unwrap();
    dbg!(&schema_str);
    assert_eq!(
        schema_str,
        r#"name: article
fields:
- f_name: a
  f_type: cellphone
  f_constraint: ''
indexes:
- i_name: i_test
  i_type: normal
  f_names:
  - name
"#
    );
}

#[test]
pub fn test_deserialize() {
    let schema_str = r#"
    name: article
    fields:
    - f_name: a
      f_type: email
      f_constraint: ''
    indexes: []        
    "#;
    let schema: Schema = serde_yaml::from_str(&schema_str).unwrap();

    assert!(true);
}
