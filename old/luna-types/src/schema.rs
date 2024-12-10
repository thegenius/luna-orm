use crate::constraint::error::ConstraintError;
use crate::record::Record;
use crate::record::RecordConstraint;
use crate::{constraint::supported::Constraint, constraint::named::NamedConstraint};

pub struct Schema<'a> {
    constraints: Vec<Constraint<'a>>,
}

impl<'a> Schema<'a> {
    pub fn validate(&self, record: Record) -> Result<(), ConstraintError> {
        return Ok(());
    }

    pub fn is_valid(&self, record: Record) -> bool {
        return true;
    }

    pub fn get_entity_schema(&self) -> Schema {
        unimplemented!()
    }

    pub fn get_selected_schema(&self) -> Schema {
        unimplemented!()
    }

    pub fn get_primary_schema(&self) -> Schema {
        unimplemented!()
    }

    pub fn get_location_schema(&self) -> Schema {
        unimplemented!()
    }

    pub fn get_mutation_schema(&self) -> Schema {
        unimplemented!()
    }
}
