#![allow(dead_code)]
use proc_macro::{self, TokenStream};
use quote::quote;
use quote::quote_spanned;

use proc_macro2::{Ident, Span};
use syn::Attribute;
use syn::Field;
use syn::{
    parse_macro_input, token, Data, DataEnum, DataStruct, DataUnion, DeriveInput, Error, Fields,
    FieldsNamed, LitStr, Path, Result,
};

fn extract_fields_name(fields: &FieldsNamed) -> impl Iterator<Item = proc_macro2::TokenStream> {
    let clone_fields = fields.clone();
    let data_expanded_members = clone_fields.named.into_iter().map(|field| {
        let field_name = field.ident.unwrap();
        let span = field_name.span();
        let field_name_stringified = LitStr::new(&field_name.to_string(), span);
        quote_spanned! { span=> #field_name_stringified.to_string()}
    });
    return data_expanded_members;
}

fn build_args_push_clause(fields: &FieldsNamed) -> impl Iterator<Item = proc_macro2::TokenStream> {
    let cloned_named = fields.named.clone();
    let arguments_expanded_members = cloned_named.into_iter().map(|field| {
        let field_name = field.ident.unwrap();
        let span = field_name.span();
        quote_spanned! { span =>
            arguments.add(self.#field_name);
        }
    });
    return arguments_expanded_members;
}

#[proc_macro_derive(Primary, attributes(TableName))]
pub fn impl_primary_macro(input: TokenStream) -> TokenStream {
    let DeriveInput {
        attrs, ident, data, ..
    } = parse_macro_input!(input);
    let fields = extract_fields(&data).unwrap();
    let args_push_clause = build_args_push_clause(&fields);
    let fields_name = extract_fields_name(&fields);
    let table_name = extract_table_name(&ident, &attrs);

    let output = quote! {

        impl Primary for #ident {

            fn name(&self) -> String {
                String::from(#table_name)
            }

            fn get_fields_name(&self) -> Vec<String> {
                vec![ #(#fields_name, )* ]
            }

            fn into_any_arguments<'p>(self) -> AnyArguments<'p> {
                let mut arguments = AnyArguments::default();
                #(#args_push_clause;)*
                return arguments;
            }
        }
    };

    // panic!("{}", output);
    output.into()
}

#[proc_macro_derive(Location, attributes(TableName))]
pub fn impl_location_macro(input: TokenStream) -> TokenStream {
    let DeriveInput {
        attrs, ident, data, ..
    } = parse_macro_input!(input);

    let fields = extract_fields(&data).unwrap();
    // let args_push_clause = build_args_push_clause(&fields);
    let fields_name = extract_fields_name(&fields);
    let cloned_named = fields.named.clone();
    let arguments_expanded_members = cloned_named.into_iter().map(|field| {
        let field_name = field.ident.unwrap();
        let span = field_name.span();
        quote_spanned! { span =>
            if let Some(#field_name) = self.#field_name {
                arguments.add(#field_name.val);
            }
        }
    });

    let cloned_named = fields.named.clone();
    let where_clause_members = cloned_named.into_iter().map(|field| {
        let field_name = field.ident.unwrap();
        let span = field_name.span();
        let field_name_stringified = LitStr::new(&field_name.to_string(), span);
        quote_spanned! { span=>
            if let Some(#field_name) = &self.#field_name {
                sql.push_str(#field_name_stringified);
                sql.push_str(#field_name.cmp.get_sql());
                sql.push_str(place_holder);
            }
        }
    });

    let table_name = extract_table_name(&ident, &attrs);

    let output = quote! {

        impl Location for #ident {

            fn name(&self) -> String {
                String::from(#table_name)
            }

            fn get_fields_name(&self) -> Vec<String> {
                vec![
                    #(#fields_name ,)*
                ]
            }

            fn get_where_clause(&self, wrap_char: char, place_holder: &str) -> String {
                let mut sql = String::default();
                #(#where_clause_members )*
                return sql;
            }

            fn into_any_arguments<'p>(self) -> AnyArguments<'p> {
                let mut arguments = AnyArguments::default();
                #(#arguments_expanded_members ;)*
                return arguments;
            }
        }
    };

    // panic!("{}", output);
    output.into()
}

#[proc_macro_derive(Mutation)]
pub fn impl_mutation_macro(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    let fields = extract_fields(&data).unwrap();
    let args_push_clause = build_args_push_clause(&fields);
    let fields_name = extract_fields_name(&fields);

    let output = quote! {
        impl Mutation for #ident {
            fn get_fields_name(&self) -> Vec<String> {
                vec![
                    #(#fields_name, )*
                ]
            }

            fn into_any_arguments<'p>(self) -> AnyArguments<'p> {
                let mut arguments = AnyArguments::default();
                #(#args_push_clause ;)*
                return arguments;
            }
        }
    };

    output.into()
}

// #[proc_macro_derive(IntoArguments)]
// pub fn into_arguments_macro_derive(input: TokenStream) -> TokenStream {
//     impl_into_arguments_macro(input)
// }

// fn impl_into_arguments_macro(input: TokenStream) -> TokenStream {
//     let DeriveInput { ident, data, .. } = parse_macro_input!(input);

//     let fields = extract_fields(&data).unwrap();

//     let data_expanded_members = fields.named.into_iter().map(|field| {
//         let field_name = field.ident.unwrap();
//         let span = field_name.span();
//         quote_spanned! { span =>
//             arguments.add(self.#field_name);
//         }
//     });

//     let output = quote! {
//         impl<'p> IntoArguments<'p, Any> for #ident {
//             fn into_arguments(self) -> AnyArguments<'p> {
//                 let mut arguments = AnyArguments::default();
//                 #(#data_expanded_members ;)*
//                 return arguments;
//             }
//         }
//     };

//     output.into()
// }

#[proc_macro_derive(SelectedEntity)]
pub fn impl_selected_entity_macro(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    let fields = extract_fields(&data).unwrap();

    let clone_named = fields.named.clone();
    let get_statement_members = clone_named.into_iter().map(|field| {
        let field_name = field.ident.unwrap();
        let span = field_name.span();
        let field_name_str = LitStr::new(&field_name.to_string(), span);
        let field_type = field.ty;
        let span = field_name.span();
        quote_spanned! { span =>
            let #field_name: #field_type = row.try_get(#field_name_str).ok();
        }
    });

    let field_members = fields.named.into_iter().map(|field| {
        let field_name = field.ident.unwrap();
        let span = field_name.span();
        quote_spanned! { span =>
            #field_name
        }
    });

    let output = quote! {
        impl SelectedEntity for #ident {
            fn from_any_row(row: AnyRow) -> Result<Self, SqlxError> where Self: Sized {
                #(#get_statement_members ;)*
                let result = #ident{ #(#field_members ,)*  };
                return Ok(result);
            }
        }
    };
    // panic!("{}", output);
    output.into()
}

fn extract_selected_fields_name(
    fields: &FieldsNamed,
) -> impl Iterator<Item = proc_macro2::TokenStream> {
    let clone_fields = fields.clone();
    let data_expanded_members = clone_fields.named.into_iter().map(|field| {
        let field_name = field.ident.unwrap();
        let span = field_name.span();
        let field_name_stringified = LitStr::new(&field_name.to_string(), span);
        quote_spanned! { span=>
            if let Some(true) = self.#field_name {
                fields.push(#field_name_stringified.to_string());
            }
        }
    });
    return data_expanded_members;
}

#[proc_macro_derive(Selection)]
pub fn impl_selection_macro(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    let fields = extract_fields(&data).unwrap();
    let fields_name = extract_selected_fields_name(&fields);

    let output = quote! {
        impl Selection for #ident {
            fn get_selected_fields(&self) -> Vec<String> {
                let mut fields = Vec::new();
                #(#fields_name)*
                return fields;
            }
        }
    };
    output.into()
}

fn extract_fields(data: &Data) -> Result<FieldsNamed> {
    let fields = match data {
        Data::Enum(DataEnum {
            enum_token: token::Enum { span },
            ..
        })
        | Data::Union(DataUnion {
            union_token: token::Union { span },
            ..
        }) => {
            return Err(Error::new(*span, "Expected a `struct`"));
        }

        Data::Struct(DataStruct {
            fields: Fields::Named(it),
            ..
        }) => it,

        Data::Struct(_) => {
            return Err(Error::new(
                Span::call_site(),
                "Expected a `struct` with named fields",
            ));
        }
    };
    return Ok(fields.clone());
}

fn extract_val_from_attr(attr: &Attribute, name: &str) -> Option<String> {
    let path: &Path = &attr.path;
    let path_ident = path.get_ident().unwrap();
    let attr_path_name = path_ident.to_string();
    if attr_path_name != name {
        return None;
    }

    let meta_info_result = attr.parse_meta();
    if meta_info_result.is_err() {
        return None;
    }

    let meta_info = meta_info_result.unwrap();
    let value = match meta_info {
        syn::Meta::NameValue(syn::MetaNameValue {
            lit: syn::Lit::Str(s),
            ..
        }) => s.value(),
        _ => panic!("malformed attribute syntax"),
    };
    return Some(value);
}

fn check_is_attr(attr: &Attribute, name: &str) -> bool {
    let path: &Path = &attr.path;
    let path_ident = path.get_ident().unwrap();
    let attr_path_name = path_ident.to_string();
    return attr_path_name == name;
}

fn extract_val_from_attrs(attrs: &Vec<Attribute>, name: &str) -> Option<String> {
    for attr in attrs {
        let val_opt = extract_val_from_attr(attr, name);
        if val_opt.is_some() {
            return val_opt;
        }
    }
    return None;
}

fn check_has_attr(attrs: &Vec<Attribute>, name: &str) -> bool {
    for attr in attrs {
        let is_attr = check_is_attr(attr, name);
        if is_attr {
            return true;
        }
    }
    return false;
}

fn extract_table_name(_ident: &Ident, attrs: &Vec<Attribute>) -> String {
    let mut name = stringify!(#ident).to_string();
    name = extract_val_from_attrs(attrs, "TableName").unwrap_or(name);
    return name;
}

fn extract_annotated_fields(fields: &FieldsNamed, name: &str) -> Vec<Field> {
    let mut result: Vec<Field> = Vec::new();
    let cloned_named: FieldsNamed = fields.clone();
    for field in cloned_named.named.into_iter() {
        let has_attr = check_has_attr(&field.attrs, name);
        if has_attr {
            result.push(field.clone());
        }
    }
    return result;
}

fn extract_not_annotated_fields(fields: &FieldsNamed, name: &str) -> Vec<Field> {
    let mut result: Vec<Field> = Vec::new();
    let cloned_named: FieldsNamed = fields.clone();
    for field in cloned_named.named.into_iter() {
        let has_attr = check_has_attr(&field.attrs, name);
        if !has_attr {
            result.push(field.clone());
        }
    }
    return result;
}

fn build_fields_name(fields: &Vec<Field>) -> impl Iterator<Item = proc_macro2::TokenStream> {
    let clone_fields = fields.clone();
    let data_expanded_members = clone_fields.into_iter().map(|field| {
        let field_name = field.ident.unwrap();
        let span = field_name.span();
        let field_name_stringified = LitStr::new(&field_name.to_string(), span);
        quote_spanned! { span=> #field_name_stringified.to_string()}
    });
    return data_expanded_members;
}

fn build_args_add_clause(
    fields: &Vec<Field>,
    cloned: bool,
) -> impl Iterator<Item = proc_macro2::TokenStream> {
    let clone_fields = fields.clone();
    let data_expanded_members = clone_fields.into_iter().map(move |field| {
        let field_name = field.ident.unwrap();
        let span = field_name.span();
        if cloned {
            quote_spanned! { span=>
                arguments.add(self.#field_name.clone());
            }
        } else {
            quote_spanned! { span=>
                arguments.add(self.#field_name);
            }
        }
    });
    return data_expanded_members;
}

#[proc_macro_derive(Entity, attributes(TableName, PrimaryKey))]
pub fn impl_entity_macro(input: TokenStream) -> TokenStream {
    let DeriveInput {
        attrs, ident, data, ..
    } = parse_macro_input!(input);

    let fields = extract_fields(&data).unwrap();

    let _fields_name = extract_fields_name(&fields);
    let primary_fields = extract_annotated_fields(&fields, "PrimaryKey");
    if primary_fields.len() == 0 {
        panic!("Entity must has at least one PrimaryKey!")
    }
    let primary_fields_name = build_fields_name(&primary_fields);

    let body_fields = extract_not_annotated_fields(&fields, "PrimaryKey");
    let body_fields_name = build_fields_name(&body_fields);

    let name = extract_table_name(&ident, &attrs);
    let primary_args_add: Vec<proc_macro2::TokenStream> =
        build_args_add_clause(&primary_fields, false).collect();
    let body_args_add: Vec<proc_macro2::TokenStream> =
        build_args_add_clause(&body_fields, false).collect();
    let body_args_add_cloned: Vec<proc_macro2::TokenStream> =
        build_args_add_clause(&body_fields, true).collect();

    let mut full_fields: Vec<Field> = Vec::new();
    full_fields.extend(primary_fields);
    full_fields.extend(body_fields);

    let clone_full_fields = full_fields.clone();
    let from_row_get_statement_members = clone_full_fields.into_iter().map(|field| {
        let field_name = field.ident.unwrap();
        let span = field_name.span();
        let field_name_str = LitStr::new(&field_name.to_string(), span);
        let field_type = field.ty;
        let span = field_name.span();
        quote_spanned! { span =>
            let #field_name: #field_type = row.try_get(#field_name_str)?;
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

    let output = quote! {
    impl Entity for #ident {
        fn name(&self) -> String {
            String::from(#name)
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
            vec![
                #(#body_fields_name, )*
            ]
        }

        fn into_insert_any_arguments<'p>(self) -> AnyArguments<'p> where Self: Sized {
            let mut arguments = AnyArguments::default();
            #(#primary_args_add; )*
            #(#body_args_add; )*
            return arguments;
        }

        fn into_update_any_arguments<'p>(self) -> AnyArguments<'p> where Self: Sized {
            let mut arguments = AnyArguments::default();
            #(#body_args_add; )*
            #(#primary_args_add; )*
            return arguments;
        }

        fn into_upsert_any_arguments<'p>(self) -> AnyArguments<'p> where Self: Sized {
            let mut arguments = AnyArguments::default();
            #(#primary_args_add; )*
            #(#body_args_add_cloned; )*
            #(#body_args_add; )*
            return arguments;
        }

    }
    };
    // panic!("{}", output);
    output.into()
}
