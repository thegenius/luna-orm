/*
use luna_orm_macro::Location;
use luna_orm_macro::Selection;
use luna_orm_trait::CmpOperator;
use luna_orm_trait::Location;
use luna_orm_trait::LocationExpr;
use luna_orm_trait::Selection;
*/
use luna_orm::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json;
use sqlx::sqlx_macros;

fn false_value() -> bool {
    false
}

#[derive(Selection, Serialize, Deserialize, Clone, Debug)]
pub struct UserSelection {
    #[serde(default = "false_value")]
    name: bool,
    #[serde(default = "false_value")]
    age: bool,
}

#[derive(Location, Serialize, Deserialize, Clone, Debug)]
pub struct UserLocation {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<LocationExpr<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    age: Option<LocationExpr<i32>>,
}

#[derive(Selection, Serialize, Deserialize, Clone, Debug)]
pub struct ClassSelection {
    #[serde(default)]
    student_name: bool,
    #[serde(default)]
    class_name: bool,
}

#[derive(Location, Serialize, Deserialize, Clone, Debug)]
pub struct ClassLocation {
    #[serde(skip_serializing_if = "Option::is_none")]
    student_name: Option<LocationExpr<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    class_name: Option<LocationExpr<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserJoinedQuery {
    selection: UserSelection,
    location: UserLocation,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClassJoinedQuery {
    #[serde(flatten)]
    selection: ClassSelection,
    #[serde(alias = "where")]
    location: ClassLocation,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JoinedQuery {
    student: UserJoinedQuery,
    class: ClassJoinedQuery,
}

#[test]
pub fn test_location() {
    let content = "{ \"student_name\": true, \"class_name\": false }";
    let result: Result<ClassSelection, _> = serde_json::from_str(content);
    assert_eq!(result.is_ok(), true);
    let ClassSelection {
        student_name,
        class_name,
    } = result.unwrap();
    assert_eq!(student_name, true);
    assert_eq!(class_name, false);

    let content = "{ \"student_name\": true }";
    let result: Result<ClassSelection, _> = serde_json::from_str(content);
    assert_eq!(result.is_ok(), true);
    let ClassSelection {
        student_name,
        class_name,
    } = result.unwrap();
    assert_eq!(student_name, true);
    assert_eq!(class_name, false);

    let content = "{ \"student_name\": {\"cmp\": \"=\", \"val\": \"hello\"} }";
    let result: Result<ClassLocation, _> = serde_json::from_str(content);
    dbg!(&result);
    assert_eq!(result.is_ok(), true);
    let ClassLocation {
        student_name,
        class_name: _,
    } = result.unwrap();
    assert_eq!(
        student_name,
        Some(LocationExpr {
            cmp: CmpOperator::Eq,
            val: "hello".to_string()
        })
    );
}
