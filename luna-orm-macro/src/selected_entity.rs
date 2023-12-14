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
