use crate::structs::expand_struct;
use proc_macro2::TokenStream as TokenStream2;
use syn::{Data, Error, Ident};

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
