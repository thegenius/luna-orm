use std::borrow::Cow;

#[derive(Debug, PartialEq, Clone)]
pub struct FieldName {
    pub name: Cow<'static, str>,
    pub is_null: bool,
}

impl FieldName {
    pub fn new(name: Cow<'static, str>, is_null: bool) -> Self {
        Self { name, is_null }
    }

    pub fn from_str(name: &'static str, is_null: bool) -> Self {
        Self {
            name: Cow::Borrowed(name),
            is_null,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::field::FieldName;

    #[test]
    pub fn test_field_name() {
        let field_name = FieldName::from_str("foo", true);
    }
}