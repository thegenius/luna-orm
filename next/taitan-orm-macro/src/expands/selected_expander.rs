use case::CaseExt;
use proc_macro2::{Ident, Span, TokenStream};
use syn::{Attribute, FieldsNamed};
use crate::attrs::{AttrParser, DefaultAttrParser};
use crate::fields::{FieldsParser, StructConstructor, RowConstructor, RowGetConstructor};
use quote::quote;
pub fn generate_selected_struct_and_impl(
    ident: &Ident,
    attrs: &Vec<Attribute>,
    fields: &FieldsNamed,
) -> TokenStream {
    let table_name = DefaultAttrParser::extract_table_name(ident, attrs);
    let selected_name = format!("{}SelectedEntity", table_name.to_camel());
    let selected_ident = Ident::new(&selected_name, Span::call_site());

    let selection_name = format!("{}Selection", table_name.to_camel());
    let selection_ident = Ident::new(&selection_name, Span::call_site());

    let parser = FieldsParser::from_named(fields);
    let struct_stream = parser.of_option(&selected_name);
    let sqlite_ident = Ident::new("Sqlite", Span::call_site());
    let mysql_ident = Ident::new("MySql", Span::call_site());
    let postgres_ident = Ident::new("Postgres", Span::call_site());

    let sqlite_impl = generate_selected_and_impl(fields, &selected_ident, &selection_ident, &sqlite_ident);
    let mysql_impl = generate_selected_and_impl(fields, &selected_ident, &selection_ident, &mysql_ident);
    let postgres_impl = generate_selected_and_impl(fields, &selected_ident, &selection_ident, &postgres_ident);

    let output = quote! {
        #struct_stream

        #sqlite_impl

        #mysql_impl

        #postgres_impl
    };

    output
}

fn generate_selected_and_impl(fields: &FieldsNamed, selected_ident: &Ident, selection_ident: &Ident, db_ident: &Ident) -> TokenStream {

    let parser = FieldsParser::from_named(fields);

    let selected_row_construct = parser.gen_selected_row();
    let selected_bits_row_construct = parser.gen_selected_bits_row();
    let selected_self_row_construct = parser.gen_selected_self_row();
    let full_row_construct = parser.gen_full_row();

    let output = quote! {
        impl taitan_orm::traits::SelectedEntity<sqlx::#db_ident> for #selected_ident {
            type Selection = #selection_ident;

             fn from_row(selection: &Self::Selection, row: <sqlx::#db_ident as sqlx::Database>::Row) -> Result<Self, sqlx::Error>
                where
                    Self: Sized {
                    #selected_row_construct
                }

             fn from_row_bits(bits: &bit_vec::BitVec, row: <sqlx::#db_ident as sqlx::Database>::Row) -> Result<Self, sqlx::Error>
                where
                    Self: Sized {
                    #selected_bits_row_construct
                }

            fn select_from_row(selection: &Self, row: <sqlx::#db_ident as sqlx::Database>::Row) -> Result<Self, sqlx::Error>
                where
                    Self: Sized {
                    #selected_self_row_construct
            }

            fn from_row_full(row: <sqlx::#db_ident as sqlx::Database>::Row) -> Result<Self, sqlx::Error>
            where
                Self: Sized,
            {
                #full_row_construct
            }
        }
    };
    output
}

