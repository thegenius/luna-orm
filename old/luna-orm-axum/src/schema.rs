use crate::FieldType;
use case::CaseExt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Field {
    pub f_name: String,
    pub f_type: FieldType,
    pub f_constraint: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum IndexType {
    Normal,
    Primary,
    Unique,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Index {
    pub i_name: String,
    pub i_type: IndexType,
    pub f_names: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Schema {
    pub name: String,
    pub fields: Vec<Field>,
    pub indexes: Vec<Index>,
}
