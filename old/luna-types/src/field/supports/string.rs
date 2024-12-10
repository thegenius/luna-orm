use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Text<'a>(pub Cow<'a, str>);

impl<'a> From<&'a str> for Text<'a> {
    fn from(value: &'a str) -> Text<'a> {
        Self(Cow::Borrowed(value))
    }
}

impl From<String> for Text<'_> {
    fn from(value: String) -> Self {
        Self(Cow::Owned(value))
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

