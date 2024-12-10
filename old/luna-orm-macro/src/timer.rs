use quote::quote;
use syn::parse_macro_input;

pub fn impl_timed_func(
    _metadata: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input_fn: syn::ItemFn = parse_macro_input!(input as syn::ItemFn);
    let attrs = input_fn.attrs;
    let visibility = input_fn.vis;
    let ident = input_fn.sig.ident;
    let asyncness = input_fn.sig.asyncness;
    let inputs = input_fn.sig.inputs;
    let output = input_fn.sig.output;
    let generics = &input_fn.sig.generics;
    let where_clause = &input_fn.sig.generics.where_clause;
    let block = input_fn.block;

    let timer_name = ident.to_string();

    (quote!(
        #(#attrs)* #visibility #asyncness fn #ident #generics (#inputs) #output #where_clause {
            let _tmr = luna_orm_trait::Timer::new(#timer_name);
            #block
        }
    ))
    .into()
}
