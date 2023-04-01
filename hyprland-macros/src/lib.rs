#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ExprClosure};

/// Creates a async closure
#[proc_macro]
pub fn async_closure(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ExprClosure);
    let body = input.body;
    let inputs = input.inputs;
    let mova = input.capture;
    let expanded = quote! {{
        use std::future::IntoFuture;
        #mova |#inputs| Box::pin(async move { #body })
    }};
    expanded.into()
}
