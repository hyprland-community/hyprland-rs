use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ExprClosure};

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
