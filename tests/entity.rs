use luna_orm::prelude::*;

#[derive(Schema, Clone, Debug)]
#[TableName = "user"]
pub struct UserEntity {
    age: Option<i32>,

    #[PrimaryKey]
    #[AutoIncrement]
    id: Option<i32>,

    name: String,
}

#[test]
pub fn test_entity_with_auto() {
    let entity = UserEntity {
        id: None,
        age: Some(23),
        name: "test".to_string(),
    };

    let table_name = entity.get_table_name();
    assert_eq!(table_name, "user");

    let insert_field_names = entity.get_insert_fields();
    assert_eq!(insert_field_names, ["age", "name"]);

    let body_names = entity.get_upsert_set_fields();
    assert_eq!(body_names, ["age", "name"]);

    let arguments: AnyArguments = entity.any_arguments_of_insert();
    let len = arguments.values.0.len();
    assert_eq!(len, 2);

    let arguments: AnyArguments = entity.any_arguments_of_upsert();
    let len = arguments.values.0.len();
    assert_eq!(len, 4);
}

#[derive(Schema, Clone, Debug)]
#[TableName = "user2"]
pub struct UserEntity2 {
    age: Option<i32>,

    #[PrimaryKey]
    id: i32,

    name: String,
}

#[test]
pub fn test_entity_with_primary() {
    let entity = UserEntity2 {
        id: 1,
        age: Some(23),
        name: "test".to_string(),
    };

    let table_name = entity.get_table_name();
    assert_eq!(table_name, "user2");

    let insert_field_names = entity.get_insert_fields();
    assert_eq!(insert_field_names, ["id", "age", "name"]);

    let body_names = entity.get_upsert_set_fields();
    assert_eq!(body_names, ["age", "name"]);

    let arguments: AnyArguments = entity.any_arguments_of_insert();
    let len = arguments.values.0.len();
    assert_eq!(len, 3);

    let arguments: AnyArguments = entity.any_arguments_of_upsert();
    let len = arguments.values.0.len();
    assert_eq!(len, 5);
}
