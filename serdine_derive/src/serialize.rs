use crate::bail::bail;
use crate::fields_data::NamedFieldData;

use proc_macro2::Ident;
use quote::quote;
use syn::{
    self, parse2, Data, DataStruct, DeriveInput, Fields, FieldsNamed, Lit, Meta, MetaNameValue,
};

type TokenStream2 = proc_macro2::TokenStream;

const SERIALIZE_ATTR: &str = "serialize";

pub(crate) fn impl_serialize(input: impl Into<TokenStream2>) -> syn::Result<TokenStream2> {
    let ast: DeriveInput = parse2(input.into())?;

    let serialize_impl = match &ast.data {
        Data::Struct(DataStruct { fields, .. }) => match fields {
            Fields::Named(fields) => {
                let named_fields_data = collect_named_fields_data(fields)?;
                impl_trait_with_named_fields(&ast, named_fields_data)?
            }
            Fields::Unnamed(_) => bail!("Unnamed fields not supported!"),
            Fields::Unit => bail!("Unit fields not supported!"),
        },
        Data::Enum(_) => bail!("Enums not supported!"),
        Data::Union(_) => bail!("Unions not supported!"),
    };

    Ok(quote!(
        #serialize_impl
    ))
}

// ////////////////////////////////////////////////////////////////////////////////
// STRUCT WITH NAMED FIELDS
// ////////////////////////////////////////////////////////////////////////////////

fn collect_named_fields_data(fields: &FieldsNamed) -> syn::Result<Vec<NamedFieldData>> {
    let mut fields_data = vec![];

    for field in &fields.named {
        // Fields are named, so an ident is necessarily found.
        let mut field_data = NamedFieldData::new(field.ident.clone().unwrap());

        for attr in &field.attrs {
            let attr_meta = match attr.parse_meta() {
                Ok(meta) => meta,
                Err(error) => bail!(error),
            };

            if let Meta::NameValue(MetaNameValue {
                ref path, ref lit, ..
            }) = attr_meta
            {
                if path.is_ident(SERIALIZE_ATTR) {
                    if let Lit::Str(lit_val) = lit {
                        field_data.serialization_fn = Some(lit_val.to_owned());
                    } else {
                        bail!("The `serialize` attribute requires a string literal");
                    }
                }
            }
        }

        fields_data.push(field_data);
    }

    Ok(fields_data)
}

fn impl_trait_with_named_fields(
    ast: &'_ DeriveInput,
    fields_data: Vec<NamedFieldData>,
) -> syn::Result<TokenStream2> {
    let type_name = &ast.ident;

    let fields_serialization = fields_data.iter().map(
        |NamedFieldData {
             field,
             serialization_fn,
             ..
         }| {
            if let Some(serialization_fn) = serialization_fn {
                let serialization_fn =
                    Ident::new(&serialization_fn.value(), serialization_fn.span());
                quote! { #serialization_fn(&self.#field, &mut w); }
            } else {
                quote! { self.#field.serialize(&mut w); }
            }
        },
    );

    Ok(quote!(
        impl serdine::Serialize for #type_name {
            fn serialize<W: std::io::Write>(&self, mut w: W) {
                    #(#fields_serialization)*
            }
        }
    ))
}
