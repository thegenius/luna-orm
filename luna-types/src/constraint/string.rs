use super::Constraint;
use derive_builder::Builder;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::borrow::Cow;
use std::{fmt::Debug, marker::PhantomData};

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(into))]
pub struct StringConstraint<'a> {
    #[builder(default = "None")]
    min_len: Option<usize>,

    #[builder(default = "None")]
    max_len: Option<usize>,

    #[builder(default = "false")]
    deny_blank: bool,

    #[builder(setter(custom))]
    #[serde(with = "serde_regex")]
    #[builder(default = "None")]
    regex: Option<Regex>,

    #[builder(setter(skip))]
    phantom: PhantomData<&'a ()>,
}

impl<'a> StringConstraintBuilder<'a> {
    pub fn regex(&mut self, expression: &str) -> &mut Self {
        let exp: Option<Regex> = Regex::new(expression).ok();
        self.regex = Some(exp);
        self
    }
}

impl<'a> Eq for StringConstraint<'a> {}
impl<'a> PartialEq for StringConstraint<'a> {
    fn eq(&self, other: &Self) -> bool {
        if self.min_len != other.min_len {
            return false;
        }
        if self.max_len != other.max_len {
            return false;
        }
        if self.deny_blank != other.deny_blank {
            return false;
        }
        if self.regex.is_some() == other.regex.is_some() {
            if self.regex.is_some() {
                let same_regx =
                    self.regex.as_ref().unwrap().as_str() == other.regex.as_ref().unwrap().as_str();
                if !same_regx {
                    return false;
                }
            }
        } else {
            return false;
        }

        return true;
    }
}

impl<'a> Constraint for StringConstraint<'a> {
    type ValueType = Cow<'a, str>;

    fn is_valid_json(&self, value: &Value) -> bool {
        let value = value.as_str();
        if value.is_none() {
            return false;
        }
        let value = value.unwrap();
        let value_cow = Cow::Borrowed(value);
        return self.is_valid(&value_cow);
    }

    fn is_valid(&self, value: &Cow<'a, str>) -> bool {
        if let Some(min_len) = self.min_len {
            if value.len() < min_len {
                return false;
            }
        }
        if let Some(max_len) = self.max_len {
            if value.len() > max_len {
                return false;
            }
        }
        if self.deny_blank {
            if value.is_empty() {
                return false;
            }
            if value.trim().is_empty() {
                return false;
            }
        }
        if let Some(regex) = &self.regex {
            if !regex.is_match(value) {
                return false;
            }
        }
        return true;
    }
}
