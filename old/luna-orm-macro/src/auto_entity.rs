use crate::entity::generate_entity_impl;
use crate::fields_parser::FieldsParser;
use crate::location::generate_location;
use crate::mutation::generate_mutation;
use crate::primary::generate_primary;
use crate::selected_entity::generate_selected_entity;
use crate::selection::generate_selection;
use crate::utils::*;
use proc_macro::{self, TokenStream};
use syn::{parse_macro_input, DeriveInput};

pub fn impl_auto_entity_macro(input: TokenStream) -> TokenStream {
    let DeriveInput {
        attrs, ident, data, ..
    } = parse_macro_input!(input);

    let fields = extract_fields(&data).unwrap();
    let mut output = generate_entity_impl(&ident, &attrs, &fields);

    let primary_fields = FieldsParser::from_named(&fields).filter_annotated_fields("PrimaryKey");
    let body_fields = FieldsParser::from_named(&fields).filter_not_annotated_fields("PrimaryKey");
    let full_fields = FieldsParser::from_named(&fields).get_sorted_fields();

    let table_name = extract_table_name(&ident, &attrs);
    let generated_primary = generate_primary(&table_name, &primary_fields);
    let generated_mutation = generate_mutation(&table_name, &body_fields);
    let generated_selection = generate_selection(&table_name, &full_fields);
    let generated_selected_entity = generate_selected_entity(&table_name, &full_fields);
    let generated_location = generate_location(&table_name, &full_fields, &attrs);

    output.extend(generated_primary);
    output.extend(generated_selection);
    output.extend(generated_selected_entity);
    output.extend(generated_mutation);
    output.extend(generated_location);
    //panic!("{}", output);
    output.into()
}
