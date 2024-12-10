use luna_orm_trait::Entity;
use luna_orm_trait::ParsedSchema;
use serde_json::Value;
use sqlx::any::AnyArguments;

#[derive(Debug, Clone)]
pub struct ActiveEntity {
    value: Value,
    schema: ParsedSchema,
}

impl Entity for ActiveEntity {
    fn get_table_name(&self) -> &str {
        &self.schema.name
    }

    fn get_insert_fields(&self) -> Vec<String> {
        self.schema.fields.names.clone()
    }

    fn get_upsert_set_fields(&self) -> Vec<String> {
        self.schema.fields.non_primary_names.clone()
    }

    fn get_auto_increment_field(&self) -> Option<&str> {
        self.schema.fields.auto_increment_field.as_deref()
    }
    fn set_auto_increment_field(&mut self, value: Option<i64>) -> bool {
        if value.is_none() {
            return false;
        }

        let name = self.get_auto_increment_field();
        if name.is_none() {
            return false;
        }
        let name = name.unwrap().to_string();

        match &mut self.value {
            Value::Object(record) => {
                record.insert(name, Value::Number(value.unwrap().into()));
                true
            }
            _ => false,
        }
    }
    fn any_arguments_of_insert(&self) -> sqlx::any::AnyArguments<'_> {
        let record = self.value.as_object().unwrap();
        let arg = AnyArguments::default();
        for name in &self.schema.fields.names {
            let value = record.get(name);
        }
        return arg;
    }
    fn any_arguments_of_upsert(&self) -> sqlx::any::AnyArguments<'_> {
        let arg = AnyArguments::default();
        return arg;
    }
}
