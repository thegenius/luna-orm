use serde_json::{Number, Value};

#[derive(Debug, Clone)]
pub struct ActiveJson {
    value: Value,
}
/*
impl ActiveJson {
    pub fn as_str(&self) -> Option<&str> {
        match &self.value {
            Value::String(data) => Some(data),
            _ => None,
        }
    }
    pub fn as_bool(&self) -> Option<bool> {
        match &self.value {
            Value::Bool(data) => Some(*data),
            _ => None,
        }
    }
    pub fn as_null(&self) -> Option<()> {
        match &self.value {
            Value::Null => Some(()),
            _ => None,
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        match &self.value {
            Value::Number(data) => data.as_i64(),
            _ => None,
        }
    }

    pub fn as_u64(&self) -> Option<u64> {
        match &self.value {
            Value::Number(data) => data.as_u64(),
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match &self.value {
            Value::Number(data) => data.as_f64(),
            _ => None,
        }
    }
}
*/
