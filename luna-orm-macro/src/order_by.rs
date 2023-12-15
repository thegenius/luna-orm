use proc_macro::{self, TokenStream};
use quote::quote;
use quote::quote_spanned;

use crate::utils::*;
use proc_macro2::{Ident, Span};
use syn::Attribute;
use syn::Field;
use syn::{
    parse_macro_input, token, Data, DataEnum, DataStruct, DataUnion, DeriveInput, Error, Fields,
    FieldsNamed, LitStr, Path, Result,
};
pub fn impl_order_by_macro(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    let fields = extract_fields(&data).unwrap();
    let fields_name = extract_order_by_fields_name(&fields);

    let output = quote! {
        impl OrderBy for #ident {
            fn get_order_by_fields(&self) -> Vec<String> {
                let mut fields = Vec::new();
                #(#fields_name)*
                return fields;
            }
        }
    };
    output.into()
}
