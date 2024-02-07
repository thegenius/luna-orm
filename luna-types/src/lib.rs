#![allow(clippy::needless_return)]

mod constraint;
mod field;
mod record;
mod sqlx;

pub use constraint::CachedConstraint;
pub use constraint::Constraint;
pub use constraint::ConstraintType;
pub use constraint::IntegerConstraint;
pub use constraint::IntegerConstraintBuilder;
pub use constraint::NamedConstraint;
pub use constraint::NamedIntConstraint;
pub use constraint::StringConstraint;
pub use constraint::StringConstraintBuilder;

pub use field::try_from_json;
pub use field::FieldType;
pub use field::Integer;
pub use field::Text;
pub use field::ValidField;

pub use record::Record;
pub use record::RecordConstraint;
