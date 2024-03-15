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
    #[serde(default)]
    is_option: Option<bool>,

    #[builder(default = "None")]
    #[serde(default)]
    min_len: Option<usize>,

    #[builder(default = "None")]
    #[serde(default)]
    max_len: Option<usize>,

    #[builder(default = "false")]
    #[serde(default = "bool::default")]
    deny_blank: bool,

    #[builder(setter(custom))]
    #[serde(with = "serde_regex")]
    #[builder(default = "None")]
    #[serde(default)]
    regex: Option<Regex>,

    #[builder(setter(skip))]
    #[serde(default)]
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

    fn is_option(&self) -> bool {
        return self.is_option.unwrap_or(false);
    }

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

#[cfg(test)]
mod test {
    use super::*;
    use serde_json;

    #[test]
    fn test_default() {
        let constraint_str = r#" {}   "#;
        let string_constrait: StringConstraint = serde_json::from_str(constraint_str).unwrap();
        let json_value_str = r#" "" "#;
        let json_value: serde_json::Value = serde_json::from_str(json_value_str).unwrap();
        let is_option = string_constrait.is_option();
        assert_eq!(is_option, false);
        let is_valid = string_constrait.is_valid_json(&json_value);
        assert_eq!(is_valid, true);
    }

    #[test]
    fn test_is_option() {
        let constraint_str = r#" { "is_option": true }   "#;
        let string_constrait: StringConstraint = serde_json::from_str(constraint_str).unwrap();
        let is_option = string_constrait.is_option();
        assert_eq!(is_option, true);
    }

    #[test]
    fn test_min_length() {
        let constraint_str = r#" { "min_len": 10 }   "#;
        let string_constrait: StringConstraint = serde_json::from_str(constraint_str).unwrap();
        let json_value_str = r#" "test" "#;
        let json_value: serde_json::Value = serde_json::from_str(json_value_str).unwrap();
        let is_option = string_constrait.is_option();
        assert_eq!(is_option, false);
        let is_valid = string_constrait.is_valid_json(&json_value);
        assert_eq!(is_valid, false);
    }
}
