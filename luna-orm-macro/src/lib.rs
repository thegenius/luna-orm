#![allow(dead_code)]
use proc_macro::TokenStream;

mod entity;
mod location;
mod mutation;
mod order_by;
mod primary;
mod selected_entity;
mod selection;
mod utils;

use entity::impl_entity_macro;
use location::impl_location_macro;
use mutation::impl_mutation_macro;
use order_by::impl_order_by_macro;
use primary::impl_primary_macro;
use selected_entity::impl_selected_entity_macro;
use selection::impl_selection_macro;

#[proc_macro_derive(Primary, attributes(TableName))]
pub fn expand_primary_macro(input: TokenStream) -> TokenStream {
    impl_primary_macro(input)
}

#[proc_macro_derive(Location, attributes(TableName))]
pub fn expand_location_macro(input: TokenStream) -> TokenStream {
    impl_location_macro(input)
}

#[proc_macro_derive(Mutation)]
pub fn expand_mutation_macro(input: TokenStream) -> TokenStream {
    impl_mutation_macro(input)
}

#[proc_macro_derive(SelectedEntity)]
pub fn expand_selected_entity_macro(input: TokenStream) -> TokenStream {
    impl_selected_entity_macro(input)
}

#[proc_macro_derive(Selection)]
pub fn expand_selection_macro(input: TokenStream) -> TokenStream {
    impl_selection_macro(input)
}

#[proc_macro_derive(Entity, attributes(TableName, PrimaryKey))]
pub fn expand_entity_macro(input: TokenStream) -> TokenStream {
    impl_entity_macro(input)
}

#[proc_macro_derive(OrderBy)]
pub fn expand_order_by_macro(input: TokenStream) -> TokenStream {
    impl_order_by_macro(input)
}
