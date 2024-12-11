use crate::database::sqlite::SqliteCommander;
use crate::SqlApi;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone)]
pub struct DB<T: SqlApi>(pub T);

impl<T> Deref for DB<T>
where
    T: SqlApi,
{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for DB<T>
where
    T: SqlApi,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<SqliteCommander> for DB<SqliteCommander> {
    fn from(value: SqliteCommander) -> Self {
        Self(value)
    }
}
