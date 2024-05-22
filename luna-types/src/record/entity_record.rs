use crate::constraint::supported::Constraint;
use crate::schema::Schema;

pub struct EntityRecord<'a> {
    fields: Vec<Constraint<'a>>,
    constraints: &'a Schema<'a>
}
