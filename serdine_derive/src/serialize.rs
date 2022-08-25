use crate::bail::bail;
use crate::field_data::FieldData;

use proc_macro2::Ident;
use quote::quote;
use syn::{self, parse2, Data, DataStruct, DeriveInput, Fields, Lit, Meta, MetaNameValue};

type TokenStream2 = proc_macro2::TokenStream;

const SERIALIZE_ATTR: &str = "serialize";

pub(crate) fn impl_serialize(input: impl Into<TokenStream2>) -> syn::Result<TokenStream2> {
    let ast: DeriveInput = parse2(input.into())?;

    let fields_data = collect_fields_data(&ast)?;

    let serialize_impl = impl_serialize_trait(&ast, fields_data)?;

    Ok(quote!(
        #serialize_impl
    ))
}
fn collect_fields_data(ast: &'_ DeriveInput) -> syn::Result<Vec<FieldData>> {
    if let Data::Struct(DataStruct {
        fields: Fields::Named(fields),
        ..
    }) = &ast.data
    {
        let mut fields_data = vec![];

        for f in &fields.named {
            // Fields are named, so an ident is necessarily found.
            let mut field_data = FieldData::new(f.ident.clone().unwrap());

            for attr in &f.attrs {
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
    } else {
        bail!("Unexpected input; named fields expected")
    }
}

fn impl_serialize_trait(
    ast: &'_ DeriveInput,
    fields_data: Vec<FieldData>,
) -> syn::Result<TokenStream2> {
    let type_name = &ast.ident;

    let fields_serialization = fields_data.iter().map(
        |FieldData {
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
