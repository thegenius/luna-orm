use std::ops::{Deref, DerefMut};
use crate::SqlExecutor;

#[derive(Debug, Clone)]
pub struct DB<T: SqlExecutor>(pub T);

impl<T> Deref for DB<T>
where
    T: SqlExecutor,
{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for DB<T>
where
    T: SqlExecutor,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
