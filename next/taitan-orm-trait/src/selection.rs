use std::fmt::Debug;

pub trait Selection: Sync + Debug {
    fn get_table_name(&self) -> &'static str;

    fn get_selected_fields(&self) -> Vec<String>;

    fn full_fields() -> Self where Self: Sized;
}
