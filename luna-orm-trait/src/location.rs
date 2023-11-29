use super::{Location, Selection};

pub struct LocatedQuery<S, L>
where
    S: Selection + Send,
    L: Location + Send,
{
    selection: S,
    location: L,
}

pub struct JoinField {
    table_name: String,
    field_name: String,
}

pub type JoinConditions = Vec<JoinField>;

#[cfg(test)]
mod test {}
