use std::fmt::Debug;

pub trait OrderBy: Sync + Debug {
    fn get_order_by_fields(&self) -> &'static [&'static str];
}