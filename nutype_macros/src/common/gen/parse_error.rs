use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::common::models::{ErrorTypeName, InnerType, ParseErrorTypeName, TypeName};

/// Generate a name for the error which is used for FromStr trait implementation.
pub fn gen_parse_error_name(type_name: &TypeName) -> ParseErrorTypeName {
    let ident = format_ident!("{type_name}ParseError");
    ParseErrorTypeName::new(ident)
}

/// Generate an error which is used for FromStr trait implementation of non-string types (e.g.
/// floats or integers)
pub fn gen_def_parse_error(
    inner_type: impl Into<InnerType>,
    type_name: &TypeName,
    maybe_error_type_name: Option<&ErrorTypeName>,
    parse_error_type_name: &ParseErrorTypeName,
) -> TokenStream {
    let inner_type: InnerType = inner_type.into();
    let type_name_str = type_name.to_string();

    let definition = if let Some(error_type_name) = maybe_error_type_name {
        quote! {
            #[derive(Debug)]
            pub enum #parse_error_type_name {
                Parse(<#inner_type as ::core::str::FromStr>::Err),
                Validate(#error_type_name),
            }

            impl ::core::fmt::Display for #parse_error_type_name {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    match self {
                        #parse_error_type_name::Parse(err) => write!(f, "Failed to parse {}: {}", #type_name_str, err),
                        #parse_error_type_name::Validate(err) => write!(f, "Failed to parse {}: {}", #type_name_str, err),
                    }

                }
            }
        }
    } else {
        quote! {
            #[derive(Debug)]
            pub enum #parse_error_type_name {
                Parse(<#inner_type as ::core::str::FromStr>::Err),
            }

            impl ::core::fmt::Display for #parse_error_type_name {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    match self {
                        #parse_error_type_name::Parse(err) => write!(f, "Failed to parse {}: {}", #type_name_str, err),
                    }
                }
            }
        }
    };

    let impl_std_error = quote! {
        impl ::std::error::Error for #parse_error_type_name {
            fn source(&self) -> Option<&(dyn ::std::error::Error + 'static)> {
                None
            }
        }
    };

    quote! {
        #definition
        #impl_std_error
    }
}
