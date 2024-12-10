use crate::Schema;

pub struct Generator {
    schema: Schema,
}

impl Generator {
    pub fn new(schema: Schema) -> Self {
        Self { schema }
    }

    pub fn from_yaml<T: AsRef<str>>(value: T) -> serde_yaml::Result<Self> {
        let schema = serde_yaml::from_str(value.as_ref())?;
        Ok(Generator::new(schema))
    }

    pub fn generate_axum_router() -> String {
        "".to_string()
    }
}
