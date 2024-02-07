use super::ValidField;
use crate::constraint::ConstraintError;
use crate::CachedConstraint;
use crate::StringConstraint;
use serde::Serialize;
use std::borrow::Cow;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

pub struct Text<'a>(Cow<'a, str>);

/*
impl<'a, T: Into<Cow<'a, str>>> From<T> for Text<'a> {
    fn from(value: T) -> Self {
        Text(value.into())
    }
}
*/

impl<'a> Text<'a> {
    pub fn from_valid(
        value: impl Into<Cow<'a, str>>,
        constraint: &CachedConstraint<<Self as ValidField>::ConstraintType>,
    ) -> Result<Self, ConstraintError<'_>> {
        let data = value.into();
        <Self as ValidField>::try_from_valid(&data, constraint)?;
        return Ok(Text(data));
    }
}

impl<'a> Deref for Text<'a> {
    type Target = Cow<'a, str>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> DerefMut for Text<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> ValidField for Text<'a> {
    type ValueType = Cow<'a, str>;
    type ConstraintType = StringConstraint<'a>;
}
