
use crate::constraint;
use crate::constraint::common::ConstraintTrait;
use crate::constraint::error::ConstraintError;
use crate::constraint::supported::Constraint;
use crate::field::named::NamedField;
use crate::field::supported::Field;

pub struct ValidField<'a> {
    field: Field<'a>,
    constraint: Constraint<'a>,
}

impl<'a> ValidField<'a> {
    pub fn get_field(&'a self)-> &Field<'a> {
        return &self.field;
    }
    pub fn get_constraint(&'a self) -> &Constraint<'a> {
        return &self.constraint;
    }

    pub fn from_valid(field: Field<'a>, constraint: Constraint<'a>) -> Result<ValidField<'a>, ConstraintError<'a>> {
        if !constraint.is_valid(&field) {
            return Err(ConstraintError::new("not valid"));
        }
        return Ok(ValidField { field, constraint });
    }
}


pub struct ValidNamedField<'a> {
    field: NamedField<'a>,
    constraint: Constraint<'a>,
}

impl<'a> ValidNamedField<'a> {
    pub fn get_named_field(&'a self)-> &NamedField<'a> {
        return &self.field;
    }
    pub fn get_constraint(&'a self) -> &Constraint<'a> {
        return &self.constraint;
    }

    pub fn from_valid(field: NamedField<'a>, constraint: Constraint<'a>) -> Result<ValidNamedField<'a>, ConstraintError<'a>> {
        if !constraint.is_valid(&field.field) {
            return Err(ConstraintError::new("not valid"));
        }
        return Ok(ValidNamedField { field, constraint });
    }
}