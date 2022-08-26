use syn::{DeriveInput, Expr, ExprLit, FieldsNamed, Ident, Lit, Meta, MetaNameValue};

use crate::{
    bail::bail,
    fields_data::{NamedFieldData, VariantData},
    target::Target,
};

const REPR_PATH: &str = "repr";

// ////////////////////////////////////////////////////////////////////////////////
// STRUCT WITH NAMED FIELDS
// ////////////////////////////////////////////////////////////////////////////////

pub fn collect_named_fields_data(
    fields: &FieldsNamed,
    target: Target,
) -> syn::Result<Vec<NamedFieldData>> {
    let mut fields_data = vec![];

    for field in &fields.named {
        // Fields are named, so an ident is necessarily found.
        //
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
                // There are different approaches; all a bit odd, but avoid duplicating the rest.
                //
                if path.is_ident(target.attribute_name()) {
                    if let Lit::Str(lit_val) = lit {
                        target.set_serialization_fn(&mut field_data, lit_val.to_owned());
                    } else {
                        bail!(format!(
                            "The `{}` attribute requires a string literal",
                            target.attribute_name()
                        ));
                    }
                }
            }
        }

        fields_data.push(field_data);
    }

    Ok(fields_data)
}

// ////////////////////////////////////////////////////////////////////////////////
// ENUMS
// ////////////////////////////////////////////////////////////////////////////////

pub fn find_type_numeric_repr(ast: &'_ DeriveInput) -> syn::Result<Ident> {
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

pub fn collect_variants_data(data_enum: &syn::DataEnum) -> syn::Result<Vec<VariantData>> {
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
