use crate::collection::{collect_named_fields_data, collect_variants_data, find_type_numeric_repr};
use crate::fields_data::NamedFieldData;
use crate::target::Target::ForSerialization;
use crate::{bail::bail, fields_data::VariantData};

use proc_macro2::Ident;
use quote::quote;
use syn::{self, parse2, Data, DataStruct, DeriveInput, Fields};

type TokenStream2 = proc_macro2::TokenStream;

pub(crate) fn impl_serialize(input: impl Into<TokenStream2>) -> syn::Result<TokenStream2> {
    let ast: DeriveInput = parse2(input.into())?;
    let type_name = &ast.ident;

    let serialize_impl = match &ast.data {
        Data::Struct(DataStruct { fields, .. }) => match fields {
            Fields::Named(fields) => {
                let named_fields_data = collect_named_fields_data(fields, ForSerialization)?;
                impl_trait_with_named_fields(type_name, named_fields_data)?
            }
            Fields::Unnamed(_) => bail!("Unnamed fields not supported!"),
            Fields::Unit => bail!("Unit fields not supported!"),
        },
        Data::Enum(data_enum) => {
            let enum_repr = find_type_numeric_repr(&ast)?;
            let variants_data = collect_variants_data(data_enum)?;
            impl_trait_with_enum_variants(type_name, enum_repr, variants_data)?
        }
        Data::Union(_) => bail!("Unions not supported!"),
    };

    Ok(quote!(
        #serialize_impl
    ))
}

fn impl_trait_with_named_fields(
    type_name: &Ident,
    fields_data: Vec<NamedFieldData>,
) -> syn::Result<TokenStream2> {
    let fields_serialization = fields_data.iter().map(
        |NamedFieldData {
             field,
             serialization_fn,
             ..
         }| {
            if let Some(serialization_fn) = serialization_fn {
                let serialization_fn =
                    Ident::new(&serialization_fn.value(), serialization_fn.span());
                quote! { #serialization_fn(&self.#field, &mut w)?; }
            } else {
                quote! { self.#field.serialize(&mut w)?; }
            }
        },
    );

    Ok(quote!(
        impl serdine::Serialize for #type_name {
            fn serialize<W: std::io::Write>(&self, mut w: W) -> Result<(), std::io::Error> {
                    #(#fields_serialization)*

                    Ok(())
            }
        }
    ))
}

fn impl_trait_with_enum_variants(
    type_name: &Ident,
    enum_repr: Ident,
    variants_data: Vec<VariantData>,
) -> syn::Result<TokenStream2> {
    let field_matches = variants_data.iter().map(
        |VariantData {
             variant,
             discriminant,
         }| {
            quote! { Self::#variant => #discriminant, }
        },
    );

    Ok(quote!(
        impl serdine::Serialize for #type_name {
            fn serialize<W: std::io::Write>(&self, mut w: W) -> Result<(), std::io::Error> {
                let numeric_value = match self {
                    #(#field_matches)*
                };

                #enum_repr::serialize(&numeric_value, &mut w)?;

                Ok(())
            }
        }
    ))
}
