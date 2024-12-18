use proc_macro2::TokenStream;
use syn::Field;
use quote::quote;
use crate::types::{DefaultTypeChecker, DefaultTypeExtractor, TypeChecker, TypeExtractor};

pub trait StructFieldConstructor {

    // field_name: T
    fn get_not_option_field(field: Field) -> TokenStream {
        let field_ident = field.ident;
        if DefaultTypeChecker::type_is_option(&field.ty) {
            let inner_type = DefaultTypeExtractor::get_option_inner_type(&field.ty).unwrap();
            quote! {
                #field_ident: #inner_type
            }
        } else {
            let field_ty = field.ty;
            quote!{
                #field_ident: #field_ty
            }
        }
    }

    // field_name: Option<T>
    fn get_option_field(field: Field) -> TokenStream {
        let field_ident = field.ident;
        let field_ty = field.ty;
        if DefaultTypeChecker::type_is_option(&field_ty) {
            quote! {
                #field_ident: #field_ty
            }
        } else {
            quote! {
                #field_ident: Option<#field_ty>
            }
        }
    }

    // field_name: Option<LocationExpr<T>>
    fn get_location_field(field: Field) -> TokenStream {
        let field_ident = field.ident;
        let field_ty = field.ty;
        if DefaultTypeChecker::type_is_option(&field_ty) {
            let inner_type = DefaultTypeExtractor::get_option_inner_type(&field_ty);
            quote! {
                #field_ident: Option<taitan_orm::traits::LocationExpr<#inner_type>>
            }
        } else {
            quote! {
                #field_ident: Option<taitan_orm::traits::LocationExpr<#field_ty>>
            }
        }
    }

    // field_name: bool
    fn get_bool_field(field: Field) -> TokenStream {
        let field_ident = field.ident;
        quote!{
            #field_ident: bool
        }
    }

    fn get_bool_true_field(field: Field) -> TokenStream {
        let field_ident = field.ident;
        quote!{
            #field_ident: true
        }
    }
}