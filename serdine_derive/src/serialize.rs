use crate::fields_data::NamedFieldData;
use crate::{bail::bail, fields_data::VariantData};

use proc_macro2::Ident;
use quote::quote;
use syn::{
    self, parse2, Data, DataStruct, DeriveInput, Expr, ExprLit, Fields, FieldsNamed, Lit, Meta,
    MetaNameValue,
};

type TokenStream2 = proc_macro2::TokenStream;

const SERIALIZE_ATTR: &str = "serialize";
const REPR_PATH: &str = "repr";

pub(crate) fn impl_serialize(input: impl Into<TokenStream2>) -> syn::Result<TokenStream2> {
    let ast: DeriveInput = parse2(input.into())?;
    let type_name = &ast.ident;

    let serialize_impl = match &ast.data {
        Data::Struct(DataStruct { fields, .. }) => match fields {
            Fields::Named(fields) => {
                let named_fields_data = collect_named_fields_data(fields)?;
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

// ////////////////////////////////////////////////////////////////////////////////
// ENUMS
// ////////////////////////////////////////////////////////////////////////////////

fn find_type_numeric_repr(ast: &'_ DeriveInput) -> syn::Result<Ident> {
    for attr in &ast.attrs {
        if attr.path.is_ident(REPR_PATH) {
            if let Ok(ident) = attr.parse_args::<Ident>() {
                // It seems that there is no way of natively identifying primitive types, so we must
                // verify manually (see https://stackoverflow.com/q/66906261).

                let ident_str = ident.to_string();
                let mut ident_chars = ident_str.chars();

                let numeric_type = ident_chars.next();

                if matches!(numeric_type, Some('i') | Some('u')) {
                    let type_width = ident_chars.collect::<String>();

                    if type_width.parse::<u8>().is_ok() {
                        return Ok(ident);
                    }
                }
            }
        };
    }

    bail!("Enum repr() not found!")
}

fn collect_variants_data(data_enum: &syn::DataEnum) -> syn::Result<Vec<VariantData>> {
    let mut variants_data = vec![];

    for variant in &data_enum.variants {
        let ident = variant.ident.clone();
        let discriminant = if let Some((
            _,
            Expr::Lit(ExprLit {
                lit: Lit::Int(lit_int),
                ..
            }),
        )) = &variant.discriminant
        {
            lit_int.clone()
        } else {
            bail!(format!("'{}' variant discriminant not found!", ident))
        };

        variants_data.push(VariantData::new(ident, discriminant));
    }

    Ok(variants_data)
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
            fn serialize<W: std::io::Write>(&self, mut w: W) {
                let numeric_value = match self {
                    #(#field_matches)*
                };

                #enum_repr::serialize(&numeric_value, &mut w);

            }
        }
    ))
}
