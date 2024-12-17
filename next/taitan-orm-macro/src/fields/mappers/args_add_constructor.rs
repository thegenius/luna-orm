use crate::fields::FieldsParser;
use crate::types::{DefaultTypeChecker, TypeChecker};
use proc_macro2::TokenStream;
use quote::quote_spanned;
use syn::{Field};

/**
1. entity,    maybe option -> #field_name
2. unique,    not option   -> #field_name
3. location,  option,      -> #field_name.val
4. location + page, option + not option -> #field_name

5. mutation + location, option + option -> #field_name, #field_name.val
6. mutation + primary,  option + not option -> #field_name
-- 7. mutation + unique -> 2个encode Vec<Encode>
*/

pub trait ArgsAddConstructor {
    fn of_maybe_option(field: Field) -> TokenStream {
        let field_name = field.ident.unwrap();
        let span = field_name.span();
        let field_type = field.ty;
        if DefaultTypeChecker::type_is_option(&field_type) {
            quote_spanned! { span =>
                if let Some(#field_name) = &self.#field_name {
                    args.add(#field_name)?;
                }
            }
        } else {
            quote_spanned! { span =>
                args.add(&self.#field_name)?;
            }
        }
    }

    // treat field as not option, no matter weather field is actually option or not
    fn of_not_option(field: Field) -> TokenStream {
        let field_name = field.ident.unwrap();
        let span = field_name.span();
        quote_spanned! { span =>
            args.add(&self.#field_name)?;
        }
    }

    // treat field as option, no matter weather field is actually option or not
    fn of_option(field: Field) -> TokenStream {
        let field_name = field.ident.unwrap();
        let span = field_name.span();
        quote_spanned! { span =>
            if let Some(#field_name) = &self.#field_name {
                args.add(#field_name)?;
            }
        }
    }

    // treat field as option, no matter weather field is actually option or not, get value from #field_name.val
    fn of_location(field: Field) -> TokenStream {
        let field_name = field.ident.unwrap();
        let span = field_name.span();
        quote_spanned! { span =>
            if let Some(#field_name) = &self.#field_name {
                args.add(#field_name.val)?;
            }
        }
    }
}
