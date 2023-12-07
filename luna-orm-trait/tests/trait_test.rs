use luna_orm_trait::Primary;

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

#[test]
pub fn test_primary_trait() {
    assert_eq!(true, true);
}
