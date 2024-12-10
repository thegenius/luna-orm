use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum NumericType {
    #[serde(alias = "short")]
    Short,
    #[serde(alias = "int")]
    Integer,
    #[serde(alias = "long")]
    Long,

    #[serde(alias = "ushort")]
    UShort,
    #[serde(alias = "uint")]
    UInteger,
    #[serde(alias = "ulong")]
    ULong,

    #[serde(alias = "float")]
    Float,
    #[serde(alias = "double")]
    Double,
    #[serde(alias = "decimal")]
    Decimal,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum FieldType {
    #[serde(alias = "boolean")]
    Boolean,

    #[serde(alias = "timestamp")]
    Timestamp,

    #[serde(alias = "datetime")]
    DateTime,

    #[serde(alias = "uuid")]
    Uuid,

    #[serde(alias = "id")]
    Id,

    #[serde(alias = "string")]
    String,

    #[serde(alias = "cellphone")]
    Cellphone,

    #[serde(alias = "email")]
    Email,

    #[serde(untagged)]
    NumericType(NumericType),
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_deserialize() {
        let field_type: FieldType = serde_yaml::from_str("ushort").unwrap();
        let expect_type: FieldType = FieldType::NumericType(NumericType::UShort);
        assert_eq!(field_type, expect_type);
    }
}
