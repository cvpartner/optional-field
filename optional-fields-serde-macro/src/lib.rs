extern crate proc_macro;

mod util;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parser, Attribute, Field, Meta, NestedMeta, Path, Type};

use util::apply_function_to_struct_and_enum_fields;

/// Add `skip_serializing_if = "Field::is_missing"` and `default` annotations to [`optional_field::Field`] fields.
///
/// The attribute can be added to structs and enums.
///
/// Import this attribute with `use optional_field::serde_optional_fields;`.
///
#[proc_macro_attribute]
pub fn serde_optional_fields(_args: TokenStream, input: TokenStream) -> TokenStream {
    let res = match apply_function_to_struct_and_enum_fields(input, add_serde_optional_fields) {
        Ok(res) => res,
        Err(err) => err.to_compile_error(),
    };
    TokenStream::from(res)
}

/// Add the skip_serializing_if annotation to each field of the struct
fn add_serde_optional_fields(field: &mut Field) -> Result<(), String> {
    if let Type::Path(path) = &field.ty {
        if is_field(&path.path) {
            let has_skip_serializing_if =
                field_has_attribute(field, "serde", "skip_serializing_if");
            let has_default = field_has_attribute(field, "serde", "default");

            if !has_skip_serializing_if {
                let attr_tokens = quote!(
                    #[serde(skip_serializing_if = "optional_field::Field::is_missing")]
                );
                let parser = Attribute::parse_outer;
                let attrs = parser
                    .parse2(attr_tokens)
                    .expect("Static attr tokens should not panic");
                field.attrs.extend(attrs);
            }
            if !has_default {
                let attr_tokens = quote!(
                    #[serde(default)]
                );
                let parser = Attribute::parse_outer;
                let attrs = parser
                    .parse2(attr_tokens)
                    .expect("Static attr tokens should not panic");
                field.attrs.extend(attrs);
            }
        }
    }
    Ok(())
}

/// Return `true`, if the type path refers to `optional_field::Field`
///
/// Accepts
///
/// * `Field`
/// * `optional_field::Field`, with or without leading `::`
fn is_field(path: &Path) -> bool {
    (path.leading_colon.is_none() && path.segments.len() == 1 && path.segments[0].ident == "Field")
        || (path.segments.len() == 2
            && (path.segments[0].ident == "optional_field")
            && path.segments[1].ident == "Field")
}

/// Determine if the `field` has an attribute with given `namespace` and `name`
///
/// On the example of
/// `#[serde(skip_serializing_if = "Field::is_missing")]`
///
/// * `serde` is the outermost path, here namespace
/// * it contains a Meta::List
/// * which contains in another Meta a Meta::NameValue
/// * with the name being `skip_serializing_if`
fn field_has_attribute(field: &Field, namespace: &str, name: &str) -> bool {
    for attr in &field.attrs {
        if attr.path.is_ident(namespace) {
            // Ignore non parsable attributes, as these are not important for us
            if let Ok(Meta::List(expr)) = attr.parse_meta() {
                for expr in expr.nested {
                    if let NestedMeta::Meta(Meta::NameValue(expr)) = expr {
                        if let Some(ident) = expr.path.get_ident() {
                            if *ident == name {
                                return true;
                            }
                        }
                    }
                }
            }
        }
    }
    false
}
