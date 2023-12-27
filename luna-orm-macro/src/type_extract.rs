use crate::type_check::type_is_option;
use quote::quote;
use syn::Path;
use syn::Type;

pub fn get_option_inner_type(ty: &syn::Type) -> Option<&syn::Type> {
    if !type_is_option(ty) {
        return None;
    }

    let syn::Type::Path(ty) = ty else { return None };
    if ty.qself.is_some() {
        return None;
    }

    let last_segment = ty.path.segments.last().unwrap();
    let syn::PathArguments::AngleBracketed(generics) = &last_segment.arguments else {
        return None;
    };
    if generics.args.len() != 1 {
        return None;
    }
    let syn::GenericArgument::Type(inner_type) = &generics.args[0] else {
        return None;
    };

    Some(inner_type)
}

pub fn extract_type_path(ty: &syn::Type) -> Option<&Path> {
    match *ty {
        syn::Type::Path(ref typepath) if typepath.qself.is_none() => Some(&typepath.path),
        _ => None,
    }
}

pub fn create_option_type(ty: &syn::Type) -> proc_macro2::TokenStream {
    let output = quote!(
        std::option::Option<#ty>
    );
    output
}
