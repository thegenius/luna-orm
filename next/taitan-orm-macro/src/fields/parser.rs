
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::Field;
use syn::FieldsNamed;
use crate::types::{DefaultTypeChecker, TypeChecker};
use crate::types::{DefaultTypeExtractor, TypeExtractor};
use crate::attrs::{AttrParser, DefaultAttrParser};
use crate::fields::{FieldMapType, DefaultFieldMapper, FieldMapper};


pub struct FieldsParser {
    fields: Vec<Field>,
}

impl FieldsParser {
    pub fn from_vec(fields: &Vec<Field>) -> FieldsParser {
        Self {
            fields: fields.clone(),
        }
    }
    pub fn from_named(fields: &FieldsNamed) -> FieldsParser {
        let fields: Vec<Field> = fields.clone().named.into_iter().collect();
        Self { fields }
    }
}






impl FieldsParser {
    pub fn map_with<F>(self, map_fn: &F) -> Vec<TokenStream>
    where
        F: Fn(Field) -> TokenStream,
    {
        self.fields
            .into_iter()
            .map(map_fn)
            .collect::<Vec<TokenStream>>()
    }

    pub fn filter_annotated_fields(&self, annotation_str: &str) -> Vec<Field> {
        let mut result: Vec<Field> = Vec::new();
        for field in self.fields.iter() {
            let has_attr = <DefaultAttrParser as AttrParser>::check_has_attr(&field.attrs, annotation_str);
            if has_attr {
                result.push(field.clone());
            }
        }
        result
    }

    pub fn filter_not_annotated_fields(&self, annotation_str: &str) -> Vec<Field> {
        let mut result: Vec<Field> = Vec::new();
        for field in self.fields.iter() {
            let has_attr = <DefaultAttrParser as AttrParser>::check_has_attr(&field.attrs, annotation_str);
            if !has_attr {
                result.push(field.clone());
            }
        }
        result
    }

    pub fn filter_not_auto_generated(&self) -> Vec<Field> {
        let mut result: Vec<Field> = Vec::new();
        for field in self.fields.iter() {
            let is_generated = <DefaultAttrParser as AttrParser>::check_has_attr(&field.attrs, "Generated");
            let is_auto = <DefaultAttrParser as AttrParser>::check_has_attr(&field.attrs, "AutoIncrement");
            if (!is_generated) && (!is_auto) {
                result.push(field.clone());
            }
        }
        result
    }

    // get struct of bool
    pub fn get_bool_fields(&self) -> TokenStream {
        let tokens = <DefaultFieldMapper as FieldMapper>::map_field_vec(&self.fields, &|field| {
            let field_ident = field.ident;
            quote!(#field_ident: bool)
        });
        quote!(#(#tokens, )*)
    }

    // get struct of LocationExpr
    pub fn get_location_expr_fields(&self) -> TokenStream {
        let fields_tokens = <DefaultFieldMapper as FieldMapper>::map_field_vec(&self.fields, &|field| {
            let field_ident = field.ident;
            let field_ty = field.ty;
            if <DefaultTypeChecker as TypeChecker>::type_is_option(&field_ty) {
                let inner_type = <DefaultTypeExtractor as TypeExtractor>::get_option_inner_type(&field_ty);
                quote!(
                    #field_ident: Option<LocationExpr<#inner_type>>
                )
            } else {
                quote!(
                    #field_ident: Option<LocationExpr<#field_ty>>
                )
            }
        });
        quote!( #(#fields_tokens, )*  )
    }

    // get struct of not option
    pub fn get_not_option_fields(&self) -> TokenStream {
        let fields_tokens = <DefaultFieldMapper as FieldMapper>::map_field_vec(&self.fields, &|field| {
            let field_ident = field.ident;
            if <DefaultTypeChecker as TypeChecker>::type_is_option(&field.ty) {
                let inner_type = <DefaultTypeExtractor as TypeExtractor>::get_option_inner_type(&field.ty).unwrap();
                quote!(
                    #field_ident: #inner_type
                )
            } else {
                let field_ty = field.ty;
                quote!(
                    #field_ident: #field_ty
                )
            }
        });
        quote!(#(#fields_tokens, )* )
    }

    // get struct of option
    pub fn get_option_fields(&self) -> TokenStream {
        let fields_tokens = <DefaultFieldMapper as FieldMapper>::map_field_vec(&self.fields, &|field| {
            let field_ident = field.ident;
            let field_ty = field.ty;
            if <DefaultTypeChecker as TypeChecker>::type_is_option(&field_ty) {
                quote!(
                    #field_ident: #field_ty
                )
            } else {
                quote!(
                    #field_ident: Option<#field_ty>
                )
            }
        });
        quote! { #(#fields_tokens, )* }
    }

    pub fn get_construct_fields(&self) -> TokenStream {
        let field_members = self.fields.iter().map(|field| {
            let field_name = field.ident.as_ref().unwrap();
            let span = field_name.span();
            quote_spanned! { span =>
                #field_name
            }
        });
        quote! { #(#field_members, )* }
    }

    // get name of &'static [&'static str]
    pub fn get_name_array(&self) -> TokenStream {
        let tokens = <DefaultFieldMapper as FieldMapper>::map_field_vec(&self.fields, &|field: Field| {
            <DefaultFieldMapper as FieldMapper>::map_field(field, FieldMapType::Str)
        });
        quote!(
            &[ #(#tokens, )* ]
        )
    }

    // get name of Vec<String>
    pub fn get_name_vec(&self) -> TokenStream {
        let tokens = <DefaultFieldMapper as FieldMapper>::map_field_vec(&self.fields, &|field: Field| {
            <DefaultFieldMapper as FieldMapper>::map_field(field, FieldMapType::String)
        });
        quote!(vec![ #(#tokens, )* ])
    }

    pub fn get_option_name_vec(&self) -> TokenStream {
        let tokens = <DefaultFieldMapper as FieldMapper>::map_field_vec(&self.fields, &|field: Field| {
            <DefaultFieldMapper as FieldMapper>::map_field(field, FieldMapType::OptionNamePush)
        });
        quote!(
            let mut fields = Vec::new();
            #(#tokens;)*
            return fields;
        )
    }

    pub fn get_maybe_option_name_vec(&self) -> TokenStream {
        let tokens = <DefaultFieldMapper as FieldMapper>::map_field_vec(&self.fields, &|field: Field| {
            let field_type = &field.ty;
            if <DefaultTypeChecker as TypeChecker>::type_is_option(field_type) {
                <DefaultFieldMapper as FieldMapper>::map_field(field, FieldMapType::OptionNamePush)
            } else {
                <DefaultFieldMapper as FieldMapper>::map_field(field, FieldMapType::NamePush)
            }
        });
        quote!(
            let mut fields = Vec::new();
            #(#tokens;)*
            return fields;
        )
    }

    pub fn get_bool_name_vec(&self) -> TokenStream {
        let tokens = <DefaultFieldMapper as FieldMapper>::map_field_vec(&self.fields, &|field: Field| {
            <DefaultFieldMapper as FieldMapper>::map_field(field, FieldMapType::BoolPush)
        });
        quote!(
            let mut fields = Vec::new();
            #(#tokens)*
            return fields;
        )
    }

    pub fn get_option_args(&self) -> TokenStream {
        let args_add_clause = <DefaultFieldMapper as FieldMapper>::map_field_vec(&self.fields, &|field: Field| {
            let field_name = field.ident.unwrap();
            let span = field_name.span();
            quote_spanned! { span =>
                if let Some(#field_name) = &self.#field_name {
                    luna_add_arg(&mut arguments, &#field_name);
                }
            }
        });

        quote! {
            let mut arguments = AnyArguments::default();
            #(#args_add_clause)*
            arguments
        }
    }

    pub fn get_args(&self) -> TokenStream {
        let args_add_clause = <DefaultFieldMapper as FieldMapper>::map_field_vec(&self.fields, &|field: Field| {
            let field_name = field.ident.unwrap();
            let span = field_name.span();
            quote_spanned! { span =>
                luna_add_arg(&mut arguments, &self.#field_name);
            }
        });
        quote! {
            let mut arguments = AnyArguments::default();
            #(#args_add_clause)*
            arguments
        }
    }

    pub fn get_maybe_option_args_sqlite(&self) -> TokenStream {
        let args_add_clause = <DefaultFieldMapper as FieldMapper>::map_field_vec(&self.fields, &<DefaultFieldMapper as FieldMapper>::map_to_maybe_option_args_add);
        quote! {
            let mut args = SqliteArguments::default();
            #(#args_add_clause)*
            Ok(args)
        }
    }

    pub fn get_maybe_option_args(&self) -> TokenStream {
        let args_add_clause = <DefaultFieldMapper as FieldMapper>::map_field_vec(&self.fields, &<DefaultFieldMapper as FieldMapper>::map_to_any_args_add);
        quote! {
            let mut arguments = AnyArguments::default();
            #(#args_add_clause)*
            arguments
        }
    }

    pub fn get_option_location_args(&self) -> TokenStream {
        let args_add_clause = <DefaultFieldMapper as FieldMapper>::map_field_vec(&self.fields, &|field: Field| {
            let field_name = field.ident.unwrap();
            let span = field_name.span();
            quote_spanned! { span =>
                if let Some(#field_name) = &self.#field_name {
                    luna_add_arg(&mut arguments, &#field_name.val);
                }
            }
        });
        quote! {
            let mut arguments = AnyArguments::default();
            #(#args_add_clause)*
            arguments
        }
    }

    pub fn get_sorted_fields(&self) -> Vec<Field> {
        let primary_fields =
            FieldsParser::from_vec(&self.fields).filter_annotated_fields("PrimaryKey");
        let body_fields =
            FieldsParser::from_vec(&self.fields).filter_not_annotated_fields("PrimaryKey");
        let mut all_fields: Vec<Field> = Vec::new();
        all_fields.extend(primary_fields);
        all_fields.extend(body_fields);
        all_fields
    }

    pub fn get_insert_fields(&self) -> Vec<Field> {
        let primary_fields =
            FieldsParser::from_vec(&self.fields).filter_annotated_fields("PrimaryKey");
        let body_fields =
            FieldsParser::from_vec(&self.fields).filter_not_annotated_fields("PrimaryKey");
        let mut all_fields: Vec<Field> = Vec::new();
        all_fields.extend(primary_fields);
        all_fields.extend(body_fields);
        all_fields = FieldsParser::from_vec(&all_fields).filter_not_auto_generated();
        all_fields
    }

    pub fn get_upsert_fields(&self) -> Vec<Field> {
        let primary_fields =
            FieldsParser::from_vec(&self.fields).filter_annotated_fields("PrimaryKey");
        let body_fields =
            FieldsParser::from_vec(&self.fields).filter_not_annotated_fields("PrimaryKey");
        let mut all_fields: Vec<Field> = Vec::new();
        all_fields.extend(primary_fields);
        all_fields.extend(body_fields.clone());
        all_fields.extend(body_fields);
        all_fields = FieldsParser::from_vec(&all_fields).filter_not_auto_generated();
        all_fields
    }

    pub fn get_upsert_set_fields(&self) -> Vec<Field> {
        let mut body_fields =
            FieldsParser::from_vec(&self.fields).filter_not_annotated_fields("PrimaryKey");
        body_fields = FieldsParser::from_vec(&body_fields).filter_not_auto_generated();
        body_fields
    }

    pub fn get_auto_increment_field(&self) -> Option<Field> {
        let auto_increment_fields =
            FieldsParser::from_vec(&self.fields).filter_annotated_fields("AutoIncrement");
        let first_one = auto_increment_fields.first();
        if first_one.is_none() {
            return None;
        } else {
            return Some(first_one.unwrap().to_owned());
        }
    }

    pub fn get_insert_args(&self) -> TokenStream {
        let all_fields = FieldsParser::from_vec(&self.fields).get_insert_fields();
        FieldsParser::from_vec(&all_fields).get_maybe_option_args()
    }

    pub fn get_upsert_args(&self) -> TokenStream {
        let all_fields = FieldsParser::from_vec(&self.fields).get_upsert_fields();
        FieldsParser::from_vec(&all_fields).get_maybe_option_args()
    }

    pub fn get_where_clause(&self) -> TokenStream {
        let where_clause_members = <DefaultFieldMapper as FieldMapper>::map_field_vec(&self.fields, &<DefaultFieldMapper as FieldMapper>::map_to_where_field);
        quote! {
            let mut sql = String::default();
            #(#where_clause_members )*
            return sql;
        }
    }

    pub fn get_unique_index_check(&self, unique_indexes: &Vec<Vec<String>>) -> TokenStream {
        if unique_indexes.is_empty() {
            quote!(fields.len() == 0)
        } else {
            let mut check_token_stream = quote!();
            for unique_index in unique_indexes {
                let check_token = quote!(
                    if array_str_equal(&[#(#unique_index,)*], fields) {
                        return true;
                    }
                );
                check_token_stream.extend(check_token);
            }
            check_token_stream.extend(quote!(
                return false;
            ));
            check_token_stream
        }
    }

    pub fn get_row_construct(&self) -> TokenStream {
        // let row_get_stmts = <DefaultFieldMapper as FieldMapper>::map_field_vec(&self.fields, &|field: Field| {
        //     let field_name = field.ident.unwrap();
        //     let span = field_name.span();
        //     let field_name_lit = LitStr::new(&field_name.to_string(), span);
        //     let field_type = field.ty;
        //     if <DefaultTypeChecker as TypeChecker>::type_is_option(&field_type) {
        //         quote_spanned! { span =>
        //             let #field_name: #field_type = row.try_get(#field_name_lit).ok();
        //         }
        //     } else {
        //         quote_spanned! { span =>
        //             let #field_name: Option<#field_type> = row.try_get(#field_name_lit).ok();
        //         }
        //     }
        // });
        let row_get_stmts = <DefaultFieldMapper as FieldMapper>::map_field_vec(&self.fields, &<DefaultFieldMapper as FieldMapper>::map_to_selected_field);
        let construct_fields = self.get_construct_fields();

        quote! {
            #(#row_get_stmts)*
            let result = Self { #construct_fields  };
            Ok(result)
        }
    }
}
