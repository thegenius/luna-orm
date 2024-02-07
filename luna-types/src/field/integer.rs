use super::ValidField;
use crate::constraint::ConstraintError;
use crate::CachedConstraint;
use crate::IntegerConstraint;
use num_traits::PrimInt;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Integer<T: PrimInt>(pub T);

impl<T: PrimInt> From<T> for Integer<T> {
    fn from(value: T) -> Self {
        Integer(value)
    }
}

impl<T: PrimInt + Serialize + Debug> Integer<T> {
    pub fn from_valid(
        value: T,
        constraint: &CachedConstraint<<Self as ValidField>::ConstraintType>,
    ) -> Result<Self, ConstraintError<'_>> {
        <Self as ValidField>::try_from_valid(&value, constraint)?;
        return Ok(Integer(value));
    }
}
impl<T: PrimInt> Deref for Integer<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T: PrimInt> DerefMut for Integer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: PrimInt + Serialize + Debug> ValidField for Integer<T> {
    type ValueType = T;
    type ConstraintType = IntegerConstraint<T>;
}
