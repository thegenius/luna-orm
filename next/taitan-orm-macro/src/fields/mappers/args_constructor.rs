use crate::fields::mappers::ArgsAddConstructor;
use crate::fields::FieldsContainer;
use proc_macro2::TokenStream;
use quote::{quote};

pub trait ArgsConstructorSqlite: FieldsContainer + ArgsAddConstructor {
    fn of_maybe_option_args_sqlite(&self) -> TokenStream {
        let args_add_clause = self.map_field_vec(&<Self as ArgsAddConstructor>::of_maybe_option);
        quote! {
            let mut args = SqliteArguments::default();
            #(#args_add_clause)*
            Ok(args)
        }
    }

    fn of_not_option_args_sqlite(&self) -> TokenStream {
        let args_add_clause = self.map_field_vec(&<Self as ArgsAddConstructor>::of_not_option);
        quote! {
            let mut args = SqliteArguments::default();
            #(#args_add_clause)*
            Ok(args)
        }
    }

    fn of_option_args_sqlite(&self) -> TokenStream {
        let args_add_clause = self.map_field_vec(&<Self as ArgsAddConstructor>::of_option);
        quote! {
            let mut args = SqliteArguments::default();
            #(#args_add_clause)*
            Ok(args)
        }
    }

    fn of_location_args_sqlite(&self) -> TokenStream {
        let args_add_clause = self.map_field_vec(&<Self as ArgsAddConstructor>::of_location);
        quote! {
            let mut args = SqliteArguments::default();
            #(#args_add_clause)*
            Ok(args)
        }
    }
}
pub trait ArgsConstructorMySql: FieldsContainer + ArgsAddConstructor {
    fn of_maybe_option_args_mysql(&self) -> TokenStream {
        let args_add_clause = self.map_field_vec(&<Self as ArgsAddConstructor>::of_maybe_option);
        quote! {
            let mut args = MySqlArguments::default();
            #(#args_add_clause)*
            Ok(args)
        }
    }

    fn of_not_option_args_mysql(&self) -> TokenStream {
        let args_add_clause = self.map_field_vec(&<Self as ArgsAddConstructor>::of_not_option);
        quote! {
            let mut args = MySqlArguments::default();
            #(#args_add_clause)*
            Ok(args)
        }
    }

    fn of_option_args_mysql(&self) -> TokenStream {
        let args_add_clause = self.map_field_vec(&<Self as ArgsAddConstructor>::of_option);
        quote! {
            let mut args = MySqlArguments::default();
            #(#args_add_clause)*
            Ok(args)
        }
    }

    fn of_location_args_mysql(&self) -> TokenStream {
        let args_add_clause = self.map_field_vec(&<Self as ArgsAddConstructor>::of_location);
        quote! {
            let mut args = MySqlArguments::default();
            #(#args_add_clause)*
            Ok(args)
        }
    }
}


pub trait ArgsConstructorPostgres: FieldsContainer + ArgsAddConstructor {
    fn of_maybe_option_args_postgres(&self) -> TokenStream {
        let args_add_clause = self.map_field_vec(&<Self as ArgsAddConstructor>::of_maybe_option);
        quote! {
            let mut args = PgArguments::default();
            #(#args_add_clause)*
            Ok(args)
        }
    }

    fn of_not_option_args_postgres(&self) -> TokenStream {
        let args_add_clause = self.map_field_vec(&<Self as ArgsAddConstructor>::of_not_option);
        quote! {
            let mut args = PgArguments::default();
            #(#args_add_clause)*
            Ok(args)
        }
    }

    fn of_option_args_postgres(&self) -> TokenStream {
        let args_add_clause = self.map_field_vec(&<Self as ArgsAddConstructor>::of_option);
        quote! {
            let mut args = PgArguments::default();
            #(#args_add_clause)*
            Ok(args)
        }
    }

    fn of_location_args_postgres(&self) -> TokenStream {
        let args_add_clause = self.map_field_vec(&<Self as ArgsAddConstructor>::of_location);
        quote! {
            let mut args = PgArguments::default();
            #(#args_add_clause)*
            Ok(args)
        }
    }
}