use std::fmt::Debug;

pub trait Selection: Sync + Debug {
    fn get_table_name(&self) -> &'static str {
        todo!()
    }

    fn get_selected_fields(&self) -> Vec<String>{
        todo!()
    }
}
