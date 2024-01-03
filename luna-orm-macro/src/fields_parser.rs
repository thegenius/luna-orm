use crate::field_utils::map_field;
use crate::field_utils::map_field_vec;
use crate::field_utils::FieldMapType;
use crate::type_check::type_is_option;
use crate::type_extract::get_option_inner_type;
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::Field;
use syn::FieldsNamed;
use syn::LitStr;

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

    // get struct of bool
    pub fn get_bool_fields(&self) -> TokenStream {
        let tokens = map_field_vec(&self.fields, &|field| {
            let field_ident = field.ident;
            quote!(#field_ident: bool)
        });
        quote!(#(#tokens, )*)
    }

    // get struct of LocationExpr
    pub fn get_location_expr_fields(&self) -> TokenStream {
        let fields_tokens = map_field_vec(&self.fields, &|field| {
            let field_ident = field.ident;
            let field_ty = field.ty;
            if type_is_option(&field_ty) {
                let inner_type = get_option_inner_type(&field_ty);
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
        let fields_tokens = map_field_vec(&self.fields, &|field| {
            let field_ident = field.ident;
            if type_is_option(&field.ty) {
                let inner_type = get_option_inner_type(&field.ty).unwrap();
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
        let fields_tokens = map_field_vec(&self.fields, &|field| {
            let field_ident = field.ident;
            let field_ty = field.ty;
            if type_is_option(&field_ty) {
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
        let tokens = map_field_vec(&self.fields, &|field: Field| {
            map_field(field, FieldMapType::Str)
        });
        quote!(
            &[ #(#tokens, )* ]
        )
    }

    // get name of Vec<String>
    pub fn get_name_vec(&self) -> TokenStream {
        let tokens = map_field_vec(&self.fields, &|field: Field| {
            map_field(field, FieldMapType::String)
        });
        quote!(vec![ #(#tokens, )* ])
    }

    pub fn get_option_name_vec(&self) -> TokenStream {
        let tokens = map_field_vec(&self.fields, &|field: Field| {
            map_field(field, FieldMapType::OptionNamePush)
        });
        quote!(
            let mut fields = Vec::new();
            #(#tokens)*
            return fields;
        )
    }

    pub fn get_bool_name_vec(&self) -> TokenStream {
        let tokens = map_field_vec(&self.fields, &|field: Field| {
            map_field(field, FieldMapType::BoolPush)
        });
        quote!(
            let mut fields = Vec::new();
            #(#tokens)*
            return fields;
        )
    }

    pub fn get_option_args(&self) -> TokenStream {
        let args_add_clause = map_field_vec(&self.fields, &|field: Field| {
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
        let args_add_clause = map_field_vec(&self.fields, &|field: Field| {
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

    pub fn get_option_location_args(&self) -> TokenStream {
        let args_add_clause = map_field_vec(&self.fields, &|field: Field| {
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

    pub fn get_where_clause(&self) -> TokenStream {
        let where_clause_members = map_field_vec(&self.fields, &|field| {
            let field_name = field.ident.unwrap();
            let span = field_name.span();
            let field_name_stringified = LitStr::new(&field_name.to_string(), span);
            quote_spanned! { span=>
                if let Some(#field_name) = &self.#field_name {
                    sql.push(wrap_char);
                    sql.push_str(#field_name_stringified);
                    sql.push(wrap_char);
                    sql.push_str(#field_name.cmp.get_sql());
                    sql.push(place_holder);
                }
            }
        });
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
        let row_get_stmts = map_field_vec(&self.fields, &|field: Field| {
            let field_name = field.ident.unwrap();
            let span = field_name.span();
            let field_name_stringified = LitStr::new(&field_name.to_string(), span);
            let field_type = field.ty;
            if type_is_option(&field_type) {
                quote_spanned! { span =>
                    let #field_name: #field_type = row.try_get(#field_name_stringified).ok();
                }
            } else {
                quote_spanned! { span =>
                    let #field_name: Option<#field_type> = row.try_get(#field_name_stringified).ok();
                }
            }
        });
        let construct_fields = self.get_construct_fields();

        quote! {
            #(#row_get_stmts)*
            let result = Self { #construct_fields  };
            Ok(result)
        }
    }
}
