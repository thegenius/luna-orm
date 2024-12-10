use crate::constraint::common::ConstraintTrait;
use derive_builder::Builder;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::borrow::Cow;
use std::{fmt::Debug};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Builder)]
#[builder(setter(into))]
pub struct DateTimeConstraint {
    #[builder(default = "None")]
    #[serde(default)]
    is_option: Option<bool>
}


impl ConstraintTrait for DateTimeConstraint {
    type ValueType = DateTime<Utc>;

    fn is_option(&self) -> bool {
        return self.is_option.unwrap_or(false);
    }

    fn is_valid_json(&self, value: &Value) -> bool {
        let value = value.as_str();
        if value.is_none() {
            return false;
        }
        let value = value.unwrap();
        let datetime_value = serde_json::from_str(value);
        if let Err(_) = datetime_value {
            return false;
        }
        return self.is_valid(&datetime_value.unwrap());
    }

    fn is_valid(&self, value: &DateTime<Utc>) -> bool {
        return true;
    }
}