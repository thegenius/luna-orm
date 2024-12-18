use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use crate::fields::FieldsContainer;
use crate::fields::mappers::{StructFieldConstructor};

pub trait StructConstructor: FieldsContainer + StructFieldConstructor {
    fn of_not_option(&self, struct_name: &str) -> TokenStream {
        let struct_ident = Ident::new(&struct_name, Span::call_site());
        let fields_tokens = self.map_field_vec(&<Self as StructFieldConstructor>::get_not_option_field);
        quote! {
            #[derive(Default, Debug, Clone)]
            pub struct #struct_ident {
                #(#fields_tokens,)*
            }
        }
    }

    fn of_option(&self, struct_name: &str) -> TokenStream {
        let struct_ident = Ident::new(&struct_name, Span::call_site());
        let fields_tokens = self.map_field_vec(&<Self as StructFieldConstructor>::get_option_field);
        quote! {
            #[derive(Default, Debug, Clone)]
            pub struct #struct_ident {
                #(#fields_tokens,)*
            }
        }
    }

    // field_name: Option<LocationExpr<T>>
    fn of_location(&self, struct_name: &str) -> TokenStream {
        let struct_ident = Ident::new(&struct_name, Span::call_site());
        let fields_tokens = self.map_field_vec(&<Self as StructFieldConstructor>::get_location_field);
        quote! {
            #[derive(Default, Debug, Clone)]
            pub struct #struct_ident {
                #(#fields_tokens,)*
            }
        }
    }

    // field_name: bool
    fn of_bool(&self, struct_name: &str) -> TokenStream {
        let struct_ident = Ident::new(&struct_name, Span::call_site());
        let fields_tokens = self.map_field_vec(&<Self as StructFieldConstructor>::get_bool_field);
        quote! {
            #[derive(Default, Debug, Clone)]
            pub struct #struct_ident {
                #(#fields_tokens,)*
            }
        }
    }

    fn of_bool_true(&self) -> TokenStream {
        let fields_tokens = self.map_field_vec(&<Self as StructFieldConstructor>::get_bool_true_field);
        quote! {
            Self {
                #(#fields_tokens,)*
            }
        }
    }
}