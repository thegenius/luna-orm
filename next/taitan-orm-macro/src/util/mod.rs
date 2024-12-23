mod parser;
mod utils;
mod life_time_checker;

pub use utils::extract_fields;
pub use utils::create_path_from_str;
pub use life_time_checker::check_field_lifetime;
pub use life_time_checker::check_type_lifetime;
pub use life_time_checker::extract_generic_lifetimes;
pub use life_time_checker::build_struct_ident;
pub use life_time_checker::build_impl_trait_token;