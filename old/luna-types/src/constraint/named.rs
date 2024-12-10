use crate::constraint::common::ConstraintTrait;
use crate::constraint::supported::Constraint;
use serde::de::{self, Deserializer, MapAccess, SeqAccess, Visitor};
use serde::Deserialize;
use serde::Serialize;
use std::fmt;
use std::marker::PhantomData;
use std::ops::Deref;
use std::ops::DerefMut;

#[derive(Debug, Eq, Serialize, Clone)]
pub struct NamedConstraint<'a> {
    pub name: String,

    // #[serde(flatten)]
    pub constraint: Constraint<'a>,

    #[serde(skip)]
    constraint_str: String,
}

#[derive(Deserialize)]
#[serde(field_identifier, rename_all = "lowercase")]
enum NamedConstraintField {
    Name,
    Constraint,
}

struct ConstraintVisitor<'a> {
    phantom: PhantomData<&'a ()>
}
impl<'a> ConstraintVisitor<'a> {
    pub fn new()-> Self {
        Self {
            phantom: PhantomData::default()
        }
    }
}


impl<'de, 'a> Visitor<'de> for ConstraintVisitor<'a> {
    type Value = NamedConstraint<'a>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("struct NamedConstraint")
    }

    fn visit_map<V>(self, mut map: V) -> Result<NamedConstraint<'a>, V::Error>
    where
        V: MapAccess<'de>,
    {
        let mut name = None;
        let mut constraint = None;
        while let Some(key) = map.next_key()? {
            match key {
                NamedConstraintField::Name => {
                    if name.is_some() {
                        return Err(de::Error::duplicate_field("name"));
                    }
                    name = Some(map.next_value()?);
                }
                NamedConstraintField::Constraint => {
                    if constraint.is_some() {
                        return Err(de::Error::duplicate_field("constraint"));
                    }
                    constraint = Some(map.next_value()?);
                }
            }
        }
        let name: String = name.ok_or_else(|| de::Error::missing_field("name"))?;
        let constraint: Constraint = constraint.ok_or_else(|| de::Error::missing_field("constraint"))?;
        Ok(NamedConstraint::from_named(name, constraint))
    }
}

impl<'de, 'a> Deserialize<'de> for NamedConstraint<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["name", "constraint"];
        deserializer.deserialize_struct("NamedConstraint", FIELDS, ConstraintVisitor::new())
    }
}

impl<'a> Deref for NamedConstraint<'a> {
    type Target = Constraint<'a>;
    fn deref(&self) -> &Self::Target {
        &self.constraint
    }
}

impl<'a> DerefMut for NamedConstraint<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.constraint
    }
}

impl<'a> PartialEq for NamedConstraint<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.constraint == other.constraint
    }
}

impl<'a> NamedConstraint<'a> {
    pub fn from_named(name: impl Into<String>, value: Constraint<'a>) -> Self {
        let format_str =
            serde_json::to_string(&value).unwrap_or("CONSTRAINT SERIALIZE ERROR".to_string());
        Self {
            name: name.into(),
            constraint: value,
            constraint_str: format_str,
        }
    }
    pub fn as_str(&self) -> &str {
        return &self.constraint_str;
    }
}


impl<'a> NamedConstraint<'a> {
    pub fn cache_str(&mut self) -> &str {
        return &self.constraint_str;
    }
    pub fn name(&self) -> &str {
        return &self.name;
    }
}