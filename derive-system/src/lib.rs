use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(System)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let type_name = &input.ident;

    let expanded = quote! {
        impl lifecycle::Lifecycle for #type_name {
            fn start(self) -> Self {
                let mut s = self;

                s
            }

            fn stop(self) -> Self {
                let mut s = self;

                s
            }
        }
    };

    expanded.into()
}
