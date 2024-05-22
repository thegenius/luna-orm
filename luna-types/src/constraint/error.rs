use std::borrow::Cow;
use std::error::Error;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct ConstraintError<'a> {
    msg: Cow<'a, str>,
}

impl<'a> ConstraintError<'a> {
    pub fn new(msg: impl Into<Cow<'a, str>>) -> Self {
        Self { msg: msg.into() }
    }
}
impl<'a> Display for ConstraintError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl<'a> Error for ConstraintError<'a> {}
