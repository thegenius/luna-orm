use crate::FieldType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
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
pub struct Indexes(Vec<Index>);

impl Indexes {
    pub fn is_primary(&self, field: &Field) -> bool {
        for index in &self.0 {
            if index.i_type == IndexType::Primary && index.f_names.contains(&field.f_name) {
                return true;
            }
        }
        return false;
    }

    pub fn is_unique(&self, field: &Field) -> bool {
        for index in &self.0 {
            if index.i_type == IndexType::Primary && index.f_names.contains(&field.f_name) {
                return true;
            }
            if index.i_type == IndexType::Unique && index.f_names.contains(&field.f_name) {
                return true;
            }
        }
        return false;
    }

    pub fn is_index(&self, field: &Field) -> bool {
        for index in &self.0 {
            if index.f_names.contains(&field.f_name) {
                return true;
            }
        }
        return false;
    }

    pub fn is_non_primary(&self, field: &Field) -> bool {
        !self.is_primary(field)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum FieldAutoType {
    None,
    Generated,
    AutoIncrement,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Field {
    pub f_name: String,
    pub f_type: FieldType,
    pub f_constraint: String,
    pub f_auto_type: FieldAutoType,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Fields {
    pub field_list: Vec<Field>,
    pub names: Vec<String>,
    pub primary_names: Vec<String>,
    pub non_primary_names: Vec<String>,
    pub auto_increment_field: Option<String>,
}
impl Fields {
    pub fn from(fields: Vec<Field>, indexes: &Indexes) -> Fields {
        let names: Vec<String> = fields.iter().map(|e| e.f_name.clone()).collect();
        let primary_names: Vec<String> = fields
            .iter()
            .filter(|e| indexes.is_primary(e))
            .map(|e| e.f_name.clone())
            .collect();
        let non_primary_names: Vec<String> = fields
            .iter()
            .filter(|e| indexes.is_non_primary(e))
            .map(|e| e.f_name.clone())
            .collect();
        let mut auto_increment_fields: Vec<String> = fields
            .iter()
            .filter(|e| e.f_auto_type == FieldAutoType::AutoIncrement)
            .map(|e| e.f_name.clone())
            .collect();

        let auto_increment_field = auto_increment_fields.pop();
        Fields {
            field_list: fields,
            names,
            primary_names,
            non_primary_names,
            auto_increment_field,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SchemaDef {
    pub name: String,
    pub fields: Vec<Field>,
    pub indexes: Vec<Index>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ParsedSchema {
    pub name: String,
    pub fields: Fields,
    pub indexes: Indexes,
}

impl From<SchemaDef> for ParsedSchema {
    fn from(value: SchemaDef) -> Self {
        let indexes = Indexes(value.indexes);
        let fields = Fields::from(value.fields, &indexes);
        Self {
            name: value.name,
            fields,
            indexes,
        }
    }
}
