
use serde::{Serialize, Deserialize};
use std::borrow::Cow;
use crate::field::supported::Field;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NamedField<'a> {
    pub name: Cow<'a, str>,
    pub field: Field<'a>,
}

impl<'a> NamedField<'a> {
    pub fn new(name: impl Into<Cow<'a, str>>, field: Field<'a>) -> Self {
        Self {
            name: name.into(),
            field,
        }
    }
}