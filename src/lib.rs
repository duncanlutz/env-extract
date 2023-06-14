//! # env-extract
//!
//! The `env-extract` crate provides convenient methods of extracting environment variables into
//! different data types.
//!
//! The crate includes two procedural macros: `ConfigStruct` and `EnvVar`, which can be used to derive
//! traits and implement automatic extraction of values from environment variables.
//!
//! ## Usage
//!
//! To use the `EnvVar` and `ConfigStruct` macros, add `env-extract` as a dependency in your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! env-extract = "0.1.22"
//! ```
//!
//! Then, in your Rust code, import the procedural macros by adding the following line:
//!
//! ```rust
//! use env_extract::{EnvVar, ConfigStruct};
//! ```
//!
//! ## ConfigStruct Macro
//!
//! The `ConfigStruct` macro is applied to structs and derives the `ConfigStruct` trait. This trait
//! allows for the easy retrieval of a struct from environment variables by pulling each field from
//! the environment variables by name. The following types are valid for fields of a struct:
//!
//! - `String`
//! - `bool`
//! - `u8`, `u16`, `u32`, `u64`, `u128`
//! - `i8`, `i16`, `i32`, `i64`, `i128`
//! - `f32`, `f64`
//! - An enum that derives `EnvVar`
//!
//! The `ConfigStruct` macro supports the following attributes on the fields in the struct:
//!
//! - `default`: Sets a default value for the field. If this is not provided, the macro will panic
//!   if the environment variable is not set.
//! - `env_var`: Sets the name of the environment variable to use for this field. If this is not
//!   provided, the macro will use the name of the field in uppercase as the environment variable
//!   name.
//! - `enumerated`: Identifies an enum that implements the `EnvVar` trait. The macro will parse the
//!   environment variable to the enum type.
//!
//! ## EnvVar Macro
//!
//! The `EnvVar` macro is applied to enums and implements the `EnvVar` trait, which provides a
//! `.get()` method to retrieve a value of type `T` from an environment variable. The macro parses
//! the environment variable to the enum type.
//!
//! The `EnvVar` macro requires one of the following conditions to be met for the enum:
//!
//! - A variant called "Invalid", which will be returned if the environment variable does not match
//!   any of the variants.
//! - A variant marked with `#[default]`, which will be returned if the environment variable does
//!   not match any of the variants.
//! - The enum to be marked with `#[panic_on_invalid]`, which will panic if the environment variable
//!   does not match any of the variants.
//!
//! The `EnvVar` macro allows for the following attributes on the enum itself:
//!
//! - `#[env_var = "FOO"]`: Set a custom environment variable name to search for. Defaults to the
//!   name of the enum in uppercase.
//! - `#[panic_on_invalid]`: Panics if a valid variant is not found.
//! - `#[case(convert = "[uppercase|lowercase|exact|any]")]`: Converts all environment variable
//!   values to a specific case before comparing them to map the valid variant. This attribute is
//!   overwritten if the variant also contains this attribute.
//!
//! The `EnvVar` macro also supports the following attributes on the enum variants:
//!
//! - `#[case = "[uppercase|lowercase|exact|any]"]`: Specifies case conversion for the annotated
//!   enum variant. The `uppercase` and `lowercase` options convert the environment variable value
//!   to uppercase or lowercase before comparing it to the variant name. The `exact` option compares
//!   the environment variable value to the variant name without any case conversion. The `any`
//!   option converts both the environment variable value and the variant name to lowercase before
//!   comparing them.
//! - `#[default]`: Specifies the default enum variant.
//! - `#[ignore_variant]`: Ignores the annotated enum variant when checking for a match.
//!
//! ## Example Usage
//!
//! ```rust
//! use env_extract::{ConfigStruct, EnvVar};
//!
//! #[derive(Debug, EnvVar)]
//! #[var_name = "DATABASE_TYPE"]
//! #[panic_on_invalid]
//! #[case(convert = "lowercase")]
//! enum DatabaseType {
//!     Postgres,
//!     Mysql,
//!     Sqlite,
//! }
//!
//! #[derive(ConfigStruct, Debug)]
//! struct Config {
//!     db_host: String,
//!     db_port: u16,
//!     use_tls: bool,
//!
//!     #[enumerated]
//!     db_type: DatabaseType,
//! }
//!
//! fn main() {
//!     std::env::set_var("DB_HOST", "localhost");
//!     std::env::set_var("DB_PORT", "5432");
//!     std::env::set_var("USE_TLS", "true");
//!     std::env::set_var("DATABASE_TYPE", "postgres");
//!
//!     let config = Config::get();
//!
//!     assert_eq!(config.db_host, "localhost");
//!     assert_eq!(config.db_port, 5432);
//!     assert_eq!(config.use_tls, true);
//!     assert!(matches!(config.db_type, DatabaseType::Postgres));
//! }
//! ```
//!
//! In the example above, the `ConfigStruct` macro is used to derive the `ConfigStruct` trait for
//! the `Config` struct, enabling easy retrieval of values from environment variables. The `EnvVar`
//! trait is derived for the `DatabaseType` enum using the `EnvVar` macro, allowing the extraction
//! of the enum variant from the "DATABASE_TYPE" environment variable. The environment variable
//! values are parsed and converted according to the specified case conversions. Finally, the `Config`
//! struct is populated with values retrieved from environment variables, and assertions are used to
//! validate the extracted values.

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Attribute, DeriveInput, Lit, Meta, MetaNameValue};

/// This proc macro is applied to enums and implements the `EnvVar` trait, which provides a `.get()`
/// method to retrieve a value of type `T` from an environment variable.
///
/// The macro parses the environment variable to the enum type and requires one of the following:
/// - A variant called "Invalid", which will be returned if the environment variable does not match
///   any of the variants.
/// - A variant marked with `#[default]`, which will be returned if the environment variable does
///   not match any of the variants.
/// - The enum to be marked with `#[panic_on_invalid]`, which will panic if the environment variable
///   does not match any of the variants.
///
/// The macro supports the following attributes on the enum itself:
/// - `#[env_var = "FOO"]`: Set a custom environment variable name to search for. Defaults to the
///   name of the enum in uppercase.
/// - `#[panic_on_invalid]`: Panics if a valid variant is not found.
/// - `#[case(convert = "[uppercase|lowercase|exact|any]")]`: Converts all environment variable
///   values to a specific case before comparing them to map the valid variant. This attribute is
///   overwritten if the variant also contains this attribute.
///
/// The macro also supports the following attributes on the enum variants:
/// - `#[case = "[uppercase|lowercase|exact|any]"]`: Specifies case conversion for the annotated
///   enum variant. The `uppercase` and `lowercase` options convert the environment variable value
///   to uppercase or lowercase before comparing it to the variant name. The `exact` option compares
///   the environment variable value to the variant name without any case conversion. The `any`
///   option converts both the environment variable value and the variant name to lowercase before
///   comparing them.
/// - `#[default]`: Specifies the default enum variant.
/// - `#[ignore_variant]`: Ignores the annotated enum variant when checking for a match.
///
/// Example usage:
///
/// ```rust
/// use env_extract::EnvVar;
///
/// #[derive(EnvVar)]
/// #[var_name = "DATABASE_TYPE"]
/// #[case(convert = "uppercase")]
/// enum DatabaseType {
///     #[case(convert = "lowercase")]
///     Postgres,
///     Mysql,
///
///     #[default]
///     Sqlite,
/// }
///
/// fn main() {
///     std::env::set_var("DATABASE_TYPE", "MYSQL");
///
///     let database_type = DatabaseType::get();
///     assert!(matches!(database_type, DatabaseType::Mysql));
/// }
/// ```
///
/// In the example above, the `EnvVar` trait is implemented for the `DatabaseType` enum, allowing
/// the retrieval of a value from the "DATABASE_TYPE" environment variable. The enum variants are
/// compared to the environment variable value after applying case conversions specified by the
/// `#[case]` attributes. The `Mysql` variant is matched since the environment variable value is
/// converted to uppercase and the variant name to lowercase, resulting in a match.
#[proc_macro_derive(
    EnvVar,
    attributes(case, var_name, default, panic_on_invalid, ignore_variant)
)]
pub fn enum_from_env(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let enum_name = &input.ident;

    let var_name_to_check_for = match get_var_name(&input.attrs) {
        Some(v) => v,
        None => enum_name.to_string().to_uppercase(),
    };

    let variants = match input.data {
        syn::Data::Enum(ref variants) => &variants.variants,
        _ => panic!("EnvVar can only be derived for enums"),
    };

    let mut invalid_type: Option<&syn::Ident> = None;

    for variant in variants {
        if &variant.ident.to_token_stream().to_string() == "Invalid" {
            invalid_type = Some(&variant.ident);
        };
    }

    let mut default_value: Option<&syn::Ident> = None;

    let panic_on_invalid = input.attrs.iter().any(|attr| {
        if let Ok(Meta::Path(path)) = attr.parse_meta() {
            path.is_ident("panic_on_invalid")
        } else {
            false
        }
    });

    let default_case = get_case_conversion(&input.attrs);
    let default_case_conversion = match default_case.0 {
        CaseConversion::Uppercase => quote! { .to_uppercase() },
        CaseConversion::Lowercase => quote! { .to_lowercase() },
        CaseConversion::Exact => quote! {},
        CaseConversion::Any => quote! { .to_lowercase() },
    };

    let mut check_variants = Vec::new();
    let mut check_variants_result = Vec::new();
    for variant in variants {
        if let syn::Fields::Unit = variant.fields {
            let ignore_variant = get_empty_path_attribute(&variant.attrs, "ignore_variant");

            if ignore_variant {
                continue;
            }

            let variant_name = &variant.ident;

            let case = get_case_conversion(&variant.attrs);
            if default_value.is_none() {
                if get_empty_path_attribute(&variant.attrs, "default") {
                    default_value = Some(variant_name);
                }
            }

            let variant_case_conversion = if case.1 {
                match case.0 {
                    CaseConversion::Uppercase => quote! { .to_uppercase() },
                    CaseConversion::Lowercase => quote! { .to_lowercase() },
                    CaseConversion::Exact => quote! {},
                    CaseConversion::Any => quote! { .to_lowercase() },
                }
            } else {
                default_case_conversion.clone()
            };

            let var_case_conversion = if let CaseConversion::Any = case.0 {
                quote! { .to_lowercase() }
            } else {
                quote! {}
            };

            check_variants.push(quote! {
                if match std::env::var(#var_name_to_check_for) { Ok(v) => { Some((v)#var_case_conversion) }, Err(..) => None}.as_deref() == Some(&(stringify!(#variant_name)#variant_case_conversion)[..]) {
                    return #enum_name::#variant_name;
                }
            });

            check_variants_result.push(quote! {
                if match std::env::var(#var_name_to_check_for) { Ok(v) => { Some((v)#var_case_conversion) }, Err(..) => None}.as_deref() == Some(&(stringify!(#variant_name)#variant_case_conversion)[..]) {
                    return Ok(#enum_name::#variant_name);
                }
            });
        }
    }

    if invalid_type.is_none() && default_value.is_none() && !panic_on_invalid {
        panic!("EnvVar Enum must have either an Invalid variant or specify a variant with the #[default] attribute");
    }

    let invalid_value = if let Some(v) = default_value {
        if panic_on_invalid {
            quote! { panic!("Invalid environment variable value") }
        } else {
            quote! { #enum_name::#v }
        }
    } else {
        if panic_on_invalid {
            quote! { panic!("Invalid environment variable value") }
        } else {
            quote! { #enum_name::Invalid }
        }
    };

    let expanded = quote! {
        impl #enum_name {
            fn get() -> Self {
                #(#check_variants)*

                #invalid_value
            }

            fn get_result() -> Result<Self, String> {
                #(#check_variants_result)*

                Err("Invalid environment variable value".to_string())
            }

            fn default() -> Self {
                #invalid_value
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

fn get_var_name(attr: &[Attribute]) -> Option<String> {
    for attr in attr {
        if let Ok(Meta::NameValue(meta_value)) = attr.parse_meta() {
            if meta_value.path.is_ident("var_name") {
                match meta_value.lit {
                    syn::Lit::Str(ref s) => return Some(s.value()),
                    _ => panic!("Invalid var_name specified"),
                }
            }
        }
    }
    None
}

fn get_case_conversion(attrs: &[Attribute]) -> (CaseConversion, bool) {
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
                                "uppercase" => return (CaseConversion::Uppercase, true),
                                "lowercase" => return (CaseConversion::Lowercase, true),
                                "exact" => return (CaseConversion::Exact, true),
                                "any" => return (CaseConversion::Any, true),
                                _ => panic!("Invalid case conversion specified"),
                            }
                        }
                    }
                }
            }
        }
    }

    (CaseConversion::Exact, false)
}

fn get_empty_path_attribute(attrs: &[Attribute], path: &str) -> bool {
    for attr in attrs {
        if let Ok(Meta::Path(meta_path)) = attr.parse_meta() {
            if meta_path.is_ident(path) {
                return true;
            }
        }
    }
    false
}

fn get_default_value(attrs: &[Attribute]) -> Option<String> {
    for attr in attrs {
        if let Ok(Meta::List(meta_list)) = attr.parse_meta() {
            if meta_list.path.is_ident("default") {
                for nested_meta in meta_list.nested {
                    if let syn::NestedMeta::Lit(Lit::Str(value)) = nested_meta {
                        return Some(value.value());
                    }
                }
            }
        }
    }
    None
}

#[derive(Debug)]

enum PrimitiveType {
    String,
    Number,
    Bool,
    ImplementedEnum,
}

fn get_implemented_enum_ident(ty: &syn::Type) -> String {
    match ty {
        syn::Type::Path(type_path) => type_path.clone().into_token_stream().to_string(),
        _ => panic!("Invalid type"),
    }
}

fn get_function_primitive_type(ty: &syn::Type, attributes: &[Attribute]) -> PrimitiveType {
    match ty {
        syn::Type::Path(type_path) => {
            let type_name = match type_path.clone().into_token_stream().to_string() {
                s if s == "String" => Some(PrimitiveType::String),
                s if s == "i32"
                    || s == "u8"
                    || s == "u16"
                    || s == "u32"
                    || s == "u64"
                    || s == "u128"
                    || s == "usize"
                    || s == "i8"
                    || s == "i16"
                    || s == "i32"
                    || s == "i64"
                    || s == "i128"
                    || s == "isize"
                    || s == "f32"
                    || s == "f64" =>
                {
                    Some(PrimitiveType::Number)
                }
                s if s == "bool" => Some(PrimitiveType::Bool),
                _ => None,
            };

            if let Some(t) = type_name {
                return t;
            } else {
                if let Some(segment) = type_path.clone().path.segments.last() {
                    if segment.arguments.is_empty() {
                        if let Some(_attr) = attributes.clone().iter().find(|attr| {
                            if let Ok(meta) = attr.parse_meta() {
                                if let syn::Meta::Path(path) = meta {
                                    path.is_ident("enumerated")
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        }) {
                            return PrimitiveType::ImplementedEnum;
                        } else {
                            panic!("Invalid type")
                        }
                    }
                }
                panic!("Invalid type")
            }
        }
        _ => panic!("Invalid type"),
    }
}

/// This proc macro derives the `ConfigStruct` trait for a struct, enabling easy extraction of
/// fields from environment variables and parsing them to the correct type.
///
/// The macro supports the following attributes for struct fields:
///
/// - `default`: Sets a default value for the field. If not provided, the macro will panic if the
///              environment variable is not set.
/// - `env_var`: Sets the name of the environment variable to use for the field. If not provided,
///              the macro will use the name of the field in uppercase as the environment variable
///              name.
/// - `enumerated`: Identifies an enum that implements the `EnvVar` trait. The macro will parse the
///                 environment variable to the enum type.
///
/// Example usage:
///
/// ```rust
/// use env_extract::ConfigStruct;
/// use env_extract::EnvVar;
/// #[derive(Debug, EnvVar)]
/// #[var_name = "DATABASE_TYPE"]
/// #[panic_on_invalid]
/// #[case(convert = "lowercase")]
/// enum DatabaseType {
///     Postgres,
///     Mysql,
///     Sqlite,
/// }
///
/// #[derive(ConfigStruct, Debug)]
/// struct Config {
///     db_host: String,
///     db_port: u16,
///     use_tls: bool,
///
///     #[enumerated]
///     db_type: DatabaseType,
/// }
///
/// fn main() {
///     std::env::set_var("DB_HOST", "localhost");
///     std::env::set_var("DB_PORT", "5432");
///     std::env::set_var("USE_TLS", "true");
///     std::env::set_var("DATABASE_TYPE", "postgres");
///
///     let config = Config::get();
///
///     assert_eq!(config.db_host, "localhost");
///     assert_eq!(config.db_port, 5432);
///     assert_eq!(config.use_tls, true);
///     assert!(matches!(config.db_type, DatabaseType::Postgres));
/// }
/// ```
///
/// In the example above, the `ConfigStruct` trait is derived for the `Config` struct, allowing
/// easy extraction of fields from environment variables. The `db_host`, `db_port`, and `use_tls`
/// fields are extracted as `String`, `u16`, and `bool` types, respectively. The `db_type` field is
/// extracted as an enum type `DatabaseType`, which is parsed from the environment variable named
/// `DATABASE_TYPE` and converted to lowercase.
#[proc_macro_derive(ConfigStruct, attributes(default, enumerated, var_name))]
pub fn env_for_struct(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = &input.ident;
    let fields = match input.data {
        syn::Data::Struct(s) => s.fields,
        _ => panic!("StructVar only supports structs."),
    };

    let mut check_fields = Vec::new();
    for field in fields {
        let field_type = get_function_primitive_type(&field.ty, &field.attrs);
        let field_ident = field.ident.unwrap();

        let default_value_or_panic = match get_default_value(&field.attrs) {
            Some(v) => match field_type {
                PrimitiveType::String => quote! { #v.to_string() },
                PrimitiveType::Number => quote! { #v.to_string().parse().unwrap() },
                PrimitiveType::Bool => quote! { #v.to_string().parse().unwrap() },
                PrimitiveType::ImplementedEnum => quote! {},
            },
            None => {
                quote! { panic!("No environment variable or default value found for '{}'", stringify!(#field_ident)) }
            }
        };

        let var_name_to_check_for = match get_var_name(&field.attrs) {
            Some(v) => v,
            None => field_ident.to_token_stream().to_string().to_uppercase(),
        };

        let enum_ident: syn::Ident;
        match field_type {
            PrimitiveType::ImplementedEnum => {
                enum_ident =
                    syn::parse_str(&get_implemented_enum_ident(&field.ty).as_str()).unwrap()
            }
            _ => enum_ident = field_ident.clone(),
        };

        check_fields.push(match field_type {
            PrimitiveType::Bool => quote! {
                 #field_ident: match std::env::var(#var_name_to_check_for) {
                    Ok(v) => match v.to_string().parse() {
                        Ok(v) => v,
                        Err(..) => false
                    },
                    Err(..) => false
                 },
            },
            PrimitiveType::String => quote! {
                 #field_ident: match std::env::var(#var_name_to_check_for) {
                    Ok(v) => v.to_string(),
                    Err(..) => #default_value_or_panic
                 },
            },
            PrimitiveType::ImplementedEnum => quote! {
                #field_ident: match #enum_ident::get_result() {
                    Ok(v) => v,
                    Err(e) =>  #enum_ident::default()
                },
            },
            PrimitiveType::Number => quote! {
                 #field_ident: match std::env::var(#var_name_to_check_for) {
                    Ok(v) => match v.to_string().trim().parse() {
                        Ok(v) => v,
                        Err(..) => #default_value_or_panic
                    },
                    Err(..) => #default_value_or_panic
                 },
            },
        });
    }

    let expanded = quote! {
        impl #struct_name {
            pub fn get() -> Self {
                Self {
                    #(#check_fields)*
                }
            }
        }
    };

    expanded.into()
}
