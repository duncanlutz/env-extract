//! # env-extract
//!
//! This crate provides a procedural macro for deriving the `EnvVar` trait for an enum in Rust.
//! The `EnvVar` trait allows you to easily retrieve an enum variant based on the value of an environment variable.
//!
//! ## Usage
//!
//! To use the `EnvVar` macro, add `env-extract` as a dependency in your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! env-extract = "0.1.2"
//! ```
//!
//! Then, in your Rust code, import the procedural macro by adding the following line:
//!
//! ```rust
//! use env_extract::EnvVar;
//! ```
//!
//! ## Deriving `EnvVar`
//!
//! The `EnvVar` trait can be derived for an enum using the `#[derive(EnvVar)]` attribute.
//! Each variant of the enum represents a possible value for the environment variable.
//!
//! By default, the macro performs an exact case-sensitive comparison between the environment variable value and the enum variant names.
//! However, you can specify a case conversion for individual enum variants using the `#[case]` attribute.
//!
//! ## Case Conversion
//!
//! The `#[case]` attribute accepts a `convert` parameter with the following options:
//!
//! - `"uppercase"`: Converts the environment variable value to uppercase before comparison.
//! - `"lowercase"`: Converts the environment variable value to lowercase before comparison.
//! - `"exact"`: Performs an exact case-sensitive comparison (default).
//! - `"any"`: Skips case comparison, treating any value of the environment variable as a match.
//!
//! ## Examples
//!
//! ```rust
//! use env_extract::EnvVar;
//!
//! #[derive(EnvVar)]
//! enum LogLevel {
//!     #[case(convert = "uppercase")]
//!     Error,
//!     #[case(convert = "uppercase")]
//!     Warning,
//!     #[case(convert = "uppercase")]
//!     Info,
//! }
//!
//! fn main() {
//!     match LogLevel::get() {
//!         Ok(LogLevel::Error) => eprintln!("An error occurred"),
//!         Ok(LogLevel::Warning) => eprintln!("Warning: Something may not be right"),
//!         Ok(LogLevel::Info) => eprintln!("Informational message"),
//!         Err(err) => eprintln!("Invalid log level: {}", err),
//!     }
//! }
//! ```
//!
//! In this example, the `LogLevel` enum is derived using the `EnvVar` macro.
//! Each variant is annotated with the `#[case]` attribute and set to convert the environment variable value to uppercase before comparison.
//! The `get()` method is then used to retrieve the appropriate log level based on the environment variable value.
//! If the environment variable matches one of the enum variants, the corresponding action is performed.
//! Otherwise, an error message is printed indicating an invalid log level.
//!

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, DeriveInput, Lit, Meta, MetaNameValue};

/// This macro derives the `EnvVar` trait for an enum, allowing you to easily retrieve an enum variant based on the value of an environment variable.
/// The `EnvVar` trait provides a `get()` method that returns a `Result` indicating whether the environment variable value matches any of the enum variants.
///
/// # Usage
///
/// The `EnvVar` trait can be derived for an enum by using the `#[derive(EnvVar)]` attribute.
/// Each variant of the enum represents a possible value for the environment variable.
///
/// ## Case Conversion
///
/// By default, the macro performs an exact case-sensitive comparison between the environment variable value and the enum variant names.
/// However, you can specify a case conversion for individual enum variants using the `#[case]` attribute.
/// The `#[case]` attribute accepts a `convert` parameter with the following options:
///
/// - `"uppercase"`: Converts the environment variable value to uppercase before comparison.
/// - `"lowercase"`: Converts the environment variable value to lowercase before comparison.
/// - `"exact"`: Performs an exact case-sensitive comparison (default).
/// - `"any"`: Converts both the environment variable value and the enum variant name to lowercase before comparison, treating any value of the environment variable as a match.
///
/// # Examples
///
/// ```rust
/// use env_extract::EnvVar;
///
/// #[derive(EnvVar)]
/// enum LogLevel {
///     #[case(convert = "uppercase")]
///     Error,
///     #[case(convert = "uppercase")]
///     Warning,
///     #[case(convert = "uppercase")]
///     Info,
/// }
///
/// fn main() {
///     match LogLevel::get() {
///         Ok(LogLevel::Error) => eprintln!("An error occurred"),
///         Ok(LogLevel::Warning) => eprintln!("Warning: Something may not be right"),
///         Ok(LogLevel::Info) => eprintln!("Informational message"),
///         Err(err) => eprintln!("Invalid log level: {}", err),
///     }
/// }
/// ```
///
/// In this example, the `LogLevel` enum is derived using the `EnvVar` macro.
/// Each variant is annotated with the `#[case]` attribute and set to convert the environment variable value to uppercase before comparison.
/// The `get()` method is then used to retrieve the appropriate log level based on the environment variable value.
/// If the environment variable matches one of the enum variants, the corresponding action is performed.
/// Otherwise, an error message is printed indicating an invalid log level.
#[proc_macro_derive(EnvVar, attributes(case))]
pub fn enum_from_env(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let enum_name = &input.ident;
    let enum_name_upper = enum_name.to_string().to_uppercase();
    let variants = match input.data {
        syn::Data::Enum(ref variants) => &variants.variants,
        _ => panic!("EnvVar can only be derived for enums"),
    };

    let mut check_variants = Vec::new();
    for variant in variants {
        if let syn::Fields::Unit = variant.fields {
            let variant_name = &variant.ident;

            let case = get_case_conversion(&variant.attrs);

            let variant_case_conversion = match case {
                CaseConversion::Uppercase => quote! { .to_uppercase() },
                CaseConversion::Lowercase => quote! { .to_lowercase() },
                CaseConversion::Exact => quote! {},
                CaseConversion::Any => quote! { .to_lowercase() },
            };

            let var_case_conversion = if let CaseConversion::Any = case {
                quote! { .to_lowercase() }
            } else {
                quote! {}
            };

            check_variants.push(quote! {
                if match std::env::var(#enum_name_upper) { Ok(v) => { Some((v)#var_case_conversion) }, Err(..) => None}.as_deref() == Some(&(stringify!(#variant_name)#variant_case_conversion)[..]) {
                    return Ok(#enum_name::#variant_name);
                }
            });
        }
    }

    let expanded = quote! {
        impl #enum_name {
            fn get() -> Result<Self, String> {
                #(#check_variants)*
                Err("Invalid environment variable value".to_string())
            }
        }
    };

    TokenStream::from(expanded)
}

enum CaseConversion {
    Uppercase,
    Lowercase,
    Exact,
    Any,
}

fn get_case_conversion(attrs: &[Attribute]) -> CaseConversion {
    for attr in attrs {
        if let Ok(Meta::List(meta_list)) = attr.parse_meta() {
            if meta_list.path.is_ident("case") {
                for nested_meta in meta_list.nested {
                    if let syn::NestedMeta::Meta(Meta::NameValue(MetaNameValue {
                        path,
                        lit: Lit::Str(value),
                        ..
                    })) = nested_meta
                    {
                        if path.is_ident("convert") {
                            match value.value().as_str() {
                                "uppercase" => return CaseConversion::Uppercase,
                                "lowercase" => return CaseConversion::Lowercase,
                                "exact" => return CaseConversion::Exact,
                                "any" => return CaseConversion::Any,
                                _ => panic!("Invalid case conversion specified"),
                            }
                        }
                    }
                }
            }
        }
    }

    CaseConversion::Exact
}
