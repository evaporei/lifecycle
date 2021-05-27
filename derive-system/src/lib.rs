use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod structs;
mod system;

#[proc_macro_derive(System)]
pub fn derive_system(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let type_name = &input.ident;

    system::expand(&type_name, &input.data)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}
