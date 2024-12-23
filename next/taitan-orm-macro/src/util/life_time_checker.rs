
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields, Field, Type, GenericParam, Generics, Lifetime};

use syn::visit::Visit;

// 自定义访问器用于查找生命周期
struct LifetimeFinder {
    has_lifetime: bool,
}

impl<'ast> Visit<'ast> for LifetimeFinder {
    fn visit_lifetime(&mut self, _: &'ast syn::Lifetime) {
        self.has_lifetime = true;
    }
}

pub fn build_struct_ident(ident: &Ident, lifetimes: &Vec<Lifetime>) -> TokenStream {
    if lifetimes.is_empty() {
        quote! {
            #ident
        }
    } else {
        quote! {
            #ident<#(#lifetimes),*>
        }
    }
}

pub fn extract_generic_lifetimes(generics: Generics) -> Vec<Lifetime> {
    let mut lifetimes: Vec<Lifetime> = Vec::new();
    for param in generics.params.iter() {
        if let GenericParam::Lifetime(lifetime_def) = param {
            lifetimes.push(lifetime_def.lifetime.clone());
        }
    }
    lifetimes
}


pub fn check_type_lifetime(ty: &Type) -> bool {
    let mut finder = LifetimeFinder { has_lifetime: false };
    finder.visit_type(ty);
    finder.has_lifetime
}
pub fn check_field_lifetime(field: &Field) -> bool {
    let mut finder = LifetimeFinder { has_lifetime: false };
    finder.visit_field(field);
    finder.has_lifetime
}


// #[proc_macro_derive(CheckStruct)]
// pub fn check_struct(input: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(input as DeriveInput);
//     let struct_name = &input.ident;
//
//     // 初始化生命周期查找器
//     let mut lifetime_finder = LifetimeFinder { has_lifetime: false };
//
//     // 检查结构体字段是否包含生命周期
//     if let Data::Struct(data_struct) = input.data {
//         match data_struct.fields {
//             Fields::Named(ref fields) | Fields::Unnamed(ref fields) => {
//                 for field in fields.named.iter() {
//                     lifetime_finder.visit_type(&field.ty);
//                 }
//             },
//             Fields::Unit => {} // 单位结构体没有字段
//         }
//     }
//
//     // 生成输出代码
//     let has_lifetime = lifetime_finder.has_lifetime;
//     let expanded = quote! {
//         impl #struct_name {
//             const HAS_LIFETIME: bool = #has_lifetime;
//         }
//     };
//
//     TokenStream::from(expanded)
// }