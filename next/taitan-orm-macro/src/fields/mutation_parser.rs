use crate::fields::{FieldsContainer, FieldsParser};
use proc_macro2::TokenStream;
use std::fmt::Debug;

pub trait MutationParser: FieldsContainer {
    fn get_mutation_fields_name(&self) -> TokenStream;
    fn gen_update_arguments_sqlite(&self) -> TokenStream;
    fn gen_update_arguments_mysql(&self) -> TokenStream;
    fn gen_update_arguments_postgres(&self) -> TokenStream;
    fn gen_change_arguments_sqlite(&self) -> TokenStream;
    fn gen_change_arguments_mysql(&self) -> TokenStream;
    fn gen_change_arguments_postgres(&self) -> TokenStream;
}

impl MutationParser for FieldsParser {
    fn get_mutation_fields_name(&self) -> TokenStream {
        todo!()
    }

    fn gen_update_arguments_sqlite(&self) -> TokenStream {
        todo!()
    }

    fn gen_update_arguments_mysql(&self) -> TokenStream {
        todo!()
    }

    fn gen_update_arguments_postgres(&self) -> TokenStream {
        todo!()
    }

    fn gen_change_arguments_sqlite(&self) -> TokenStream {
        todo!()
    }

    fn gen_change_arguments_mysql(&self) -> TokenStream {
        todo!()
    }

    fn gen_change_arguments_postgres(&self) -> TokenStream {
        todo!()
    }
}