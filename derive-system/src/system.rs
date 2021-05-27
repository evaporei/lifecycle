use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{punctuated::Punctuated, token::Comma, Data, DataStruct, Error, Field, Fields, Ident};

pub fn expand(type_name: &Ident, data: &Data) -> Result<TokenStream2, Error> {
    match data {
        Data::Struct(ref data_struct) => expand_struct(type_name, data_struct),
        Data::Enum(ref data_enum) => Err(Error::new(
            data_enum.enum_token.span,
            "derive-system does not support derive for enums yet",
        )),
        Data::Union(ref data_union) => Err(Error::new(
            data_union.union_token.span,
            "derive-system does not support derive for unions",
        )),
    }
}

fn expand_struct(struct_name: &Ident, data_struct: &DataStruct) -> Result<TokenStream2, Error> {
    match data_struct.fields {
        Fields::Named(ref fields) => Ok(expand_named_struct(struct_name, &fields.named)),
        Fields::Unnamed(_) => Err(Error::new(
            data_struct.struct_token.span,
            "derive-system does not support derive for unnamed structs yet",
        )),
        Fields::Unit => Err(Error::new(
            data_struct.struct_token.span,
            "derive-system does not support derive for unit structs yet",
        )),
    }
}

fn expand_named_struct(struct_name: &Ident, fields: &Punctuated<Field, Comma>) -> TokenStream2 {
    let field_name = fields.iter().map(|field| &field.ident);
    let field_name_rev = fields.iter().map(|field| &field.ident).rev();

    quote! {
        impl lifecycle::Lifecycle for #struct_name {
            fn start(self) -> Self {
                let mut s = self;

                #(s.#field_name = s.#field_name.start();)*

                s
            }

            fn stop(self) -> Self {
                let mut s = self;

                #(s.#field_name_rev = s.#field_name_rev.stop();)*

                s
            }
        }
    }
}
