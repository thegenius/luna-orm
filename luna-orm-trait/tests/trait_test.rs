use luna_orm_trait::Primary;
use sqlx::any::AnyArguments;
use sqlx::Any;
use sqlx::Encode;
use sqlx::{AnyExecutor, Arguments};
pub struct HelloPrimary {
    name: String,
}

/*
impl Primary for HelloPrimary {
    const TABLE_NAME: &'static str = "hello";
    const PRIMARY_FIELD_NAMES: &'static [&'static str] = &["name"];

    fn get_arguments<'q, DB, T>(&self) -> Vec<T>
    where
        DB: sqlx::Database,
        T: sqlx::Encode<'q, DB> + sqlx::Type<DB>,
    {
    }
}
*/

impl Primary for HelloPrimary {
    fn get_table_name(&self) -> &'static str {
        "user"
    }
    fn get_primary_field_names(&self) -> &'static [&'static str] {
        &["name", "age"]
    }

    fn any_arguments(&self) -> sqlx::any::AnyArguments<'_> {
        let mut args = AnyArguments::default();
        //<Encode<'_, Any>>::encode_by_ref(self.name, args.values);
        luna_orm_trait::add_arg(&mut args, &self.name);
        args
    }
}

#[test]
pub fn test_primary_trait() {
    assert_eq!(true, true);
}
