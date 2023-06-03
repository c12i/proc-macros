extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, ItemFn};

#[proc_macro_attribute]
pub fn request_handler(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let args = parse_macro_input!(attr as AttributeArgs);
    let mut args = args.iter();

    let method = match &args.next().unwrap() {
        syn::NestedMeta::Meta(syn::Meta::Path(s)) => {
            let m =  s.segments.first().unwrap().ident.to_string();
            if !["GET", "POST", "PUT", "PATCH", "DELETE"].contains(&m.as_ref()) {
                panic!("Invalid method.")
            }
            m
        },
        _ => panic!("Invalid method."),
    };

    let path = match &args.next().unwrap() {
        syn::NestedMeta::Lit(syn::Lit::Str(path)) => path.value(),
        _ => panic!("Invalid path."),
    };

    // store a mapping of the route to to the request handler maybe?

    println!("method: {:?}", method);
    println!("path: {:?}", path);

    let fn_name = &input.sig.ident;
    let fn_body = &input.block;
    let fn_params = &input.sig.inputs;

    let expanded = quote! {
        fn #fn_name(#fn_params) {
            // Function body goes here
            #fn_body
        }
    };

    TokenStream::from(expanded)
}
