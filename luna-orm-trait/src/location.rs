use super::{Location, Selection};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
pub trait SelectionWithSend: Selection + Send {}
impl<T> SelectionWithSend for T where T: Selection + Send {}

pub trait LocationWithSend: Location + Send {}
impl<T> LocationWithSend for T where T: Location + Send {}


pub trait LocationTrait {
    fn get_cmp_sql(&self) -> &str;
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct LocationExpr<T> {
    pub val: T,
    pub cmp: CmpOperator,
}

impl <T> LocationTrait for LocationExpr<T> {
    fn get_cmp_sql(&self) -> &str {
        self.cmp.get_sql()
    }
}

impl<T> LocationExpr<T> {
    pub fn new(cmp: CmpOperator, val: T) -> Self {
        Self { cmp, val }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct SelectedLocationExpr<T> {
    pub selected: bool,
    pub val: T,
    pub cmp: CmpOperator,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum CmpOperator {
    #[serde(alias = "=")]
    Eq,
    #[serde(alias = "<")]
    LessThan,
    #[serde(alias = "<=")]
    LessOrEq,
    #[serde(alias = ">")]
    GreaterThan,
    #[serde(alias = ">=")]
    GreaterOrEq,
    #[serde(alias = "like")]
    Like,
}

impl CmpOperator {
    pub fn get_sql(&self) -> &'static str {
        match self {
            CmpOperator::Eq => "=",
            CmpOperator::LessThan => "<",
            CmpOperator::LessOrEq => "<=",
            CmpOperator::GreaterThan => ">",
            CmpOperator::GreaterOrEq => ">=",
            CmpOperator::Like => "LIKE",
        }
    }
}

/*
#[derive(Deserialize)]
pub struct LocationQuery<S, L>
where
    S: Selection + Send,
    L: Location + Send,
{
    selection: S,
    location: L,
}
*/

#[typetag::serde(tag = "table")]
pub trait LocatedQuery {
    fn get_selection(&self) -> &dyn Selection;
    fn get_location(&self) -> &dyn Location;
}

/*
impl<S, L> LocatedQuery for LocationQuery<S, L>
where
    S: Selection + Send,
    L: Location + Send,
{
    fn get_selection(&self) -> &dyn SelectionWithSend {
        &self.selection
    }
    fn get_location(&self) -> &dyn LocationWithSend {
        &self.location
    }
}
*/

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub enum JoinMode {
    #[serde(alias = "left")]
    Left,
    #[serde(alias = "right")]
    Right,
    #[serde(alias = "outer")]
    Outer,
    #[serde(alias = "inner")]
    Inner,
}

impl JoinMode {
    pub fn get_join_operator(&self) -> &'static str {
        match self {
            JoinMode::Left => "LEFT JOIN",
            JoinMode::Right => "RIGHT JOIN",
            JoinMode::Outer => "OUTER JOIN",
            JoinMode::Inner => "INNER JOIN",
        }
    }
}

#[derive(Serialize, PartialEq, Eq, Debug)]
pub struct JoinedField {
    table_name: String,
    field_name: String,
}

impl<'de> serde::de::Deserialize<'de> for JoinedField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let content = String::deserialize(deserializer)?;
        let pair: Vec<&str> = content.split('.').collect();
        if pair.len() != 2 {
            return Err(serde::de::Error::custom(
                "join field must have table name, and seperate by '.' ",
            ));
        }
        Ok(Self {
            table_name: pair.first().unwrap().to_string(),
            field_name: pair.last().unwrap().to_string(),
        })
    }
}

pub type JoinedFields = (JoinedField, JoinedField);

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct JoinedCondition {
    mode: JoinMode,
    #[serde(alias = "left")]
    left_table: String,
    #[serde(alias = "right")]
    right_table: String,
    #[serde(alias = "fields")]
    joined_fields: Vec<JoinedFields>,
}

fn get_on_clause(joined_fields: &Vec<JoinedFields>) -> String {
    let mut on_clause_vec: Vec<String> = Vec::new();
    for field in joined_fields {
        let on_seg = format!(
            "{}.{} = {}.{}",
            field.0.table_name, field.0.field_name, field.1.table_name, field.1.field_name
        );
        on_clause_vec.push(on_seg);
    }
    on_clause_vec.join(",")
}

pub trait FromClause {
    fn get_from_clause(&self) -> String;
}

impl FromClause for JoinedCondition {
    fn get_from_clause(&self) -> String {
        let on_clause = get_on_clause(&self.joined_fields);
        let join_operator = self.mode.get_join_operator();
        format!(
            "{} {} {} ON {}",
            self.left_table, join_operator, self.right_table, on_clause
        )
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct JoinedConditionPart {
    mode: JoinMode,
    table: String,
    #[serde(alias = "fields")]
    joined_fields: Vec<JoinedFields>,
}

impl FromClause for JoinedConditionPart {
    fn get_from_clause(&self) -> String {
        let on_clause = get_on_clause(&self.joined_fields);
        let join_operator = self.mode.get_join_operator();
        format!("{} {} ON {}", join_operator, self.table, on_clause)
    }
}

#[serde_as]
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct JoinedConditionArray<const N: usize> {
    root: JoinedCondition,
    #[serde_as(as = "[_; N]")]
    next: [JoinedConditionPart; N],
}

impl<const N: usize> JoinedConditionArray<N> {
    pub fn get_from_clause(&self) -> String {
        let root_join = self.root.get_from_clause();
        let mut part_clauses: Vec<String> = Vec::new();
        for part in &self.next {
            let part_clause = part.get_from_clause();
            part_clauses.push(part_clause);
        }
        let part_clause = part_clauses.join(" ");
        format!("{} {}", root_join, part_clause)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(untagged)]
pub enum JoinedConditions {
    Two(JoinedCondition),
    Three(JoinedConditionArray<1>),
    Four(JoinedConditionArray<2>),
    Five(JoinedConditionArray<3>),
    Six(JoinedConditionArray<4>),
    Seven(JoinedConditionArray<5>),
    Eight(JoinedConditionArray<6>),
    Nine(JoinedConditionArray<7>),
    Ten(JoinedConditionArray<8>),
    Eleven(JoinedConditionArray<9>),
    Twelve(JoinedConditionArray<10>),
}

impl FromClause for JoinedConditions {
    fn get_from_clause(&self) -> String {
        match &self {
            JoinedConditions::Two(e) => e.get_from_clause(),
            JoinedConditions::Three(e) => e.get_from_clause(),
            JoinedConditions::Four(e) => e.get_from_clause(),
            JoinedConditions::Five(e) => e.get_from_clause(),
            JoinedConditions::Six(e) => e.get_from_clause(),
            JoinedConditions::Seven(e) => e.get_from_clause(),
            JoinedConditions::Eight(e) => e.get_from_clause(),
            JoinedConditions::Nine(e) => e.get_from_clause(),
            JoinedConditions::Ten(e) => e.get_from_clause(),
            JoinedConditions::Eleven(e) => e.get_from_clause(),
            JoinedConditions::Twelve(e) => e.get_from_clause(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct JoinedQuery {
    query_vec: Vec<Box<dyn LocatedQuery>>,
    join_conditions: JoinedConditions,
}

#[cfg(test)]
mod test {
    use crate::location::JoinMode;

    use super::FromClause;
    use super::JoinedCondition;
    use super::JoinedConditions;
    use serde_json;

    #[test]
    pub fn test_joined_condition() {
        let content = r#"{ "mode": "inner", "left": "user", "right": "class", "fields": [ ["user.id", "class.id"] ]}"#;
        let joined_cond: JoinedCondition = serde_json::from_str(content).unwrap();
        assert_eq!(joined_cond.mode, JoinMode::Inner);
        assert_eq!(joined_cond.left_table, "user");
        assert_eq!(joined_cond.right_table, "class");
        assert_eq!(joined_cond.joined_fields[0].0.table_name, "user");
        assert_eq!(joined_cond.joined_fields[0].1.table_name, "class");
        assert_eq!(joined_cond.joined_fields[0].0.field_name, "id");
        assert_eq!(joined_cond.joined_fields[0].1.field_name, "id");
    }

    #[test]
    pub fn test_joined_conditions() {
        let content = r#" {"root": { "mode": "inner", "left": "user", "right": "class", "fields": [ ["user.id", "class.id"] ]}, "next":[
            {"mode": "outer", "table": "school", "fields": [["school.id", "user.id"], ["user.name", "school.name"] ] },
            {"mode": "outer", "table": "country",   "fields": [["country.id", "school.id"], ["coutry.name", "user.name"] ]}
        ] }"#;
        let joined_conds: JoinedConditions = serde_json::from_str(content).unwrap();
        match joined_conds {
            JoinedConditions::Four(_) => {}
            _ => panic!("deserialize wrong"),
        }

        let from_clause = joined_conds.get_from_clause();
        assert_eq!(from_clause, "user INNER JOIN class ON user.id = class.id OUTER JOIN school ON school.id = user.id,user.name = school.name OUTER JOIN country ON country.id = school.id,coutry.name = user.name");
    }
}
