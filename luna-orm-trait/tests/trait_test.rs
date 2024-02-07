use luna_orm_trait::Primary;
use sqlx::any::AnyArguments;
use sqlx::Any;
use sqlx::Encode;
use sqlx::{AnyExecutor, Arguments};
pub struct HelloPrimary {
    name: String,
}
use luna_orm_trait::luna_add_arg;

#[test]
pub fn test_primary_trait() {
    assert_eq!(true, true);
}
