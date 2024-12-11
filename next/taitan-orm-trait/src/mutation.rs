use std::fmt::Debug;

pub trait Mutation: Sync + Debug {
    fn get_fields_name(&self) -> Vec<String>;
}
