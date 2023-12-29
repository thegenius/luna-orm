use proc_macro::{self, TokenStream};
use quote::quote;
use quote::quote_spanned;

use crate::field_utils::*;
use crate::location::generate_location;
use crate::mutation::generate_mutation;
use crate::primary::generate_primary;
use crate::selected_entity::generate_selected_entity;
use crate::selection::generate_selection;
use crate::type_check;
use crate::type_check::field_is_option;
use crate::utils::*;
use proc_macro2::{Ident, Span};
use syn::Attribute;
use syn::Field;
use syn::{
    parse_macro_input, token, Data, DataEnum, DataStruct, DataUnion, DeriveInput, Error, Fields,
    FieldsNamed, LitStr, Path, Result,
};

pub fn impl_auto_entity_macro(input: TokenStream) -> TokenStream {
    let DeriveInput {
        attrs, ident, data, ..
    } = parse_macro_input!(input);

    let fields = extract_fields(&data).unwrap();

    let _fields_name = extract_fields_name(&fields);
    let primary_fields = extract_annotated_fields(&fields, "PrimaryKey");
    if primary_fields.is_empty() {
        panic!("Entity must has at least one PrimaryKey!")
    }

    let primary_fields_name = build_fields_name(&primary_fields);

    let body_fields = extract_not_annotated_fields(&fields, "PrimaryKey");
    //let body_fields_name = build_fields_name(&body_fields);
    let body_fields_name = build_fields_name_with_option(&body_fields);

    let generated_fields = extract_annotated_fields(&fields, "Generated");
    let generated_fields_name = build_fields_name(&generated_fields);
    let name = extract_table_name(&ident, &attrs);

    let generated_primary = generate_primary(&name, &primary_fields);
    let generated_mutation = generate_mutation(&name, &body_fields);

    let primary_args_add_ref: Vec<proc_macro2::TokenStream> =
        gen_args_add_maybe_option(&primary_fields);

    let body_args_add_ref: Vec<proc_macro2::TokenStream> = gen_args_add_maybe_option(&body_fields);

    let mut full_fields: Vec<Field> = Vec::new();
    full_fields.extend(primary_fields);
    full_fields.extend(body_fields);

    let clone_full_fields = full_fields.clone();
    let generated_selection = generate_selection(&name, &clone_full_fields);
    let generated_selected_entity = generate_selected_entity(&name, &clone_full_fields);
    let unique_indexes = extract_unique_index(&attrs);
    let generated_location = generate_location(&name, &clone_full_fields, unique_indexes);

    let from_row_get_statement_members = map_fields(&fields, &|field: Field| {
        if field_is_option(&field) {
            map_field(field, FieldMapType::RowGetOption)
        } else {
            map_field(field, FieldMapType::RowGet)
        }
    });

    let clone_full_fields = full_fields.clone();
    let from_row_field_members = clone_full_fields.into_iter().map(|field| {
        let field_name = field.ident.unwrap();
        let span = field_name.span();
        quote_spanned! { span =>
            #field_name
        }
    });

    let cloned_fields = full_fields.clone();
    let option_fields: Vec<String> = cloned_fields
        .iter()
        .filter(|f| type_check::field_is_option(f))
        .map(|f| f.ident.as_ref().unwrap().to_string())
        .collect();

    let mut output = quote! {
    impl Entity for #ident {

        fn get_generated_fields_name(&self) -> &'static [&'static str] {
            &[ #(#generated_fields_name, )*  ]
        }

        fn get_table_name(&self) -> &'static str {
                #name
        }

        fn from_any_row(row: AnyRow) -> Result<Self, SqlxError> where Self: Sized {
            #(#from_row_get_statement_members ;)*
            let result = #ident{ #(#from_row_field_members ,)*  };
            return Ok(result);
        }

        fn get_primary_fields_name(&self) -> Vec<String> {
            vec![
                #(#primary_fields_name, )*
            ]
        }

        fn get_body_fields_name(&self) -> Vec<String> {
                #(#body_fields_name)*
        }

        fn get_primary_args(&self) -> AnyArguments<'_> {
            let mut arguments = AnyArguments::default();
            #(#primary_args_add_ref; )*
            return arguments;
        }

        fn get_body_args(&self) -> AnyArguments<'_> {
            let mut arguments = AnyArguments::default();
            #(#body_args_add_ref; )*
            return arguments;
        }

        fn any_arguments_of_insert(&self) -> AnyArguments<'_> {
            let mut arguments = AnyArguments::default();
            #(#primary_args_add_ref; )*
            #(#body_args_add_ref; )*
            return arguments;
        }

        fn any_arguments_of_upsert(&self) -> AnyArguments<'_> {
            let mut arguments = AnyArguments::default();
            #(#primary_args_add_ref; )*
            #(#body_args_add_ref; )*
            #(#body_args_add_ref; )*
            return arguments;
        }

        fn any_arguments_of_update(&self) -> AnyArguments<'_> {
            let mut arguments = AnyArguments::default();
            #(#body_args_add_ref; )*
            #(#primary_args_add_ref; )*
            return arguments;
        }
    }
    };
    output.extend(generated_primary);
    output.extend(generated_selection);
    output.extend(generated_selected_entity);
    output.extend(generated_mutation);
    output.extend(generated_location);
    //panic!("{}", output);
    output.into()
}
