use proc_macro::{self, TokenStream};
use quote::quote;
use quote::quote_spanned;

use crate::utils::*;
use case::CaseExt;
use proc_macro2::{Ident, Span};
use syn::Attribute;
use syn::Field;
use syn::{
    parse_macro_input, token, Data, DataEnum, DataStruct, DataUnion, DeriveInput, Error, Fields,
    FieldsNamed, LitStr, Path, Result,
};

pub fn impl_order_by_macro(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    //let fields = extract_fields(&data).unwrap();
    //let fields_name = extract_order_by_fields_name(&fields);
    let variant_list = extract_enum_variants(&data).unwrap();
    let mut match_token_stream = quote!();
    for variant_ident in &variant_list {
        let order_by_variant_string = variant_ident.to_string().to_snake();
        let order_by_field_list: Vec<&str> =
            order_by_variant_string.split("_").collect::<Vec<&str>>();

        let match_arm = quote!(
            #ident::#variant_ident => vec![ #(#order_by_field_list.to_string() ,)* ],
        );
        match_token_stream.extend(match_arm);
    }

    let output = quote! {
        impl OrderBy for #ident {
            fn get_order_by_fields(&self) -> Vec<String> {
                match self {
                    #match_token_stream
                }
            }
        }
    };
    //panic!("{}", output)
    output.into()
}
