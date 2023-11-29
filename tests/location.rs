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
use sqlx::any::AnyArguments;
use sqlx::Arguments;
use sqlx::{any::AnyRow, Row};

#[derive(Selection, Serialize, Deserialize, Clone, Debug)]
pub struct UserSelection {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    age: Option<bool>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    student_name: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    class_name: Option<bool>,
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

#[tokio::test]
pub async fn test_location() {
    let content = "{ \"student_name\": true, \"class_name\": null }";
    let result: Result<ClassSelection, _> = serde_json::from_str(content);
    assert_eq!(result.is_ok(), true);
    let ClassSelection {
        student_name,
        class_name,
    } = result.unwrap();
    assert_eq!(student_name, Some(true));
    assert_eq!(class_name, None);

    let content = "{ \"student_name\": true }";
    let result: Result<ClassSelection, _> = serde_json::from_str(content);
    assert_eq!(result.is_ok(), true);
    let ClassSelection {
        student_name,
        class_name,
    } = result.unwrap();
    assert_eq!(student_name, Some(true));
    assert_eq!(class_name, None);

    let content = "{ \"student_name\": {\"cmp\": \"=\", \"val\": \"hello\"} }";
    let result: Result<ClassLocation, _> = serde_json::from_str(content);
    dbg!(&result);
    assert_eq!(result.is_ok(), true);
    let ClassLocation {
        student_name,
        class_name,
    } = result.unwrap();
    assert_eq!(
        student_name,
        Some(LocationExpr {
            cmp: CmpOperator::Eq,
            val: "hello".to_string()
        })
    );
}
