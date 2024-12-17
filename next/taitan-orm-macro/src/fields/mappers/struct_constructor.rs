use proc_macro2::TokenStream;
use quote::quote;
use crate::fields::FieldsContainer;
use crate::fields::mappers::{StructFieldConstructor};

pub trait StructConstructor: FieldsContainer + StructFieldConstructor {
    fn of_not_option(&self, struct_ident: &str) -> TokenStream {
        let fields_tokens = self.map_field_vec(&<Self as StructFieldConstructor>::get_not_option_field);
        quote! {
            #[derive(Default, Debug, Clone)]
            pub struct #struct_ident {
                #(#fields_tokens,)*
            }
        }
    }

    fn of_option(&self, struct_ident: &str) -> TokenStream {
        let fields_tokens = self.map_field_vec(&<Self as StructFieldConstructor>::get_option_field);
        quote! {
            #[derive(Default, Debug, Clone)]
            pub struct #struct_ident {
                #(#fields_tokens,)*
            }
        }
    }

    // field_name: Option<LocationExpr<T>>
    fn of_location(&self, struct_ident: &str) -> TokenStream {
        let fields_tokens = self.map_field_vec(&<Self as StructFieldConstructor>::get_location_field);
        quote! {
            #[derive(Default, Debug, Clone)]
            pub struct #struct_ident {
                #(#fields_tokens,)*
            }
        }
    }

    // field_name: bool
    fn of_bool(&self, struct_ident: &str) -> TokenStream {
        let fields_tokens = self.map_field_vec(&<Self as StructFieldConstructor>::get_bool_field);
        quote! {
            #[derive(Default, Debug, Clone)]
            pub struct #struct_ident {
                #(#fields_tokens,)*
            }
        }
    }
}