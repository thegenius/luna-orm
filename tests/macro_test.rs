use luna_orm::prelude::*;

#[derive(Entity, Clone)]
pub struct TestDescribe {
    #[PrimaryKey]
    id: i32,
    #[PrimaryKey]
    id2: i32,
    content: String,
}

#[test]
fn it_works() {
    let describe = TestDescribe {
        id: 23,
        id2: 24,
        content: "content".to_string(),
    };

    let primary_fields = describe.get_primary_fields_name();
    assert_eq!(primary_fields, vec!["id".to_string(), "id2".to_string()]);

    let body_fields = describe.get_body_fields_name();
    assert_eq!(body_fields, vec!["content".to_string()]);

    let arguments: AnyArguments = describe.any_arguments_of_update();
    let len = arguments.values.0.len();
    assert_eq!(len, 3);

    let arguments: AnyArguments = describe.any_arguments_of_insert();
    let len = arguments.values.0.len();
    assert_eq!(len, 3);

    let arguments: AnyArguments = describe.any_arguments_of_upsert();
    let len = arguments.values.0.len();
    assert_eq!(len, 4);
}
