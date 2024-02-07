use crate::{
    constraint::ConstraintError, field::NamedFieldType, try_from_json, ConstraintType, FieldType,
};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use sqlx::{any::AnyArguments, AnyPool, Arguments, Encode};
use std::collections::HashMap;

/* must not be hashmap, because it will mess the field order*/
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Record<'a>(Vec<NamedFieldType<'a>>);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RecordConstraint<'a>(Vec<ConstraintType<'a>>);

impl<'a> Record<'a> {
    pub fn from_json(
        value: &'a Value,
        constraints: &'a RecordConstraint,
    ) -> Result<Self, ConstraintError<'a>> {
        let mut fields: Vec<NamedFieldType<'a>> = Vec::new();
        let value_map: &Map<String, Value> = value.as_object().ok_or(ConstraintError::new(
            "only json object can transfer to record.",
        ))?;
        for constraint in &constraints.0 {
            let name = constraint.name();
            let value: Option<&Value> = value_map.get(name);
            if let Some(value) = value {
                let data = try_from_json(value, constraint)?;
                let field = NamedFieldType::new(name, data);
                fields.push(field);
            } else {
                return Err(ConstraintError::new(format!("{} not found", name)));
            }
        }
        return Ok(Record(fields));
    }

    pub fn into_any_arguments(&self) -> AnyArguments<'_> {
        let mut args = AnyArguments::default();
        for val in &self.0 {
            let _ = val.field.encode_by_ref(&mut args.values);
        }
        return args;
    }
}
