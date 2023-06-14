# env-extract

The `env-extract` crate provides convenient methods of extracting environment variables into different data types.

The crate includes two proc macros: `ConfigStruct` and `EnvVar`, which can be used to derive traits and implement automatic extraction of values from environment variables.

## Usage

To use the `EnvVar` and `ConfigStruct` macros, add `env-extract` as a dependency in your `Cargo.toml` file:

```toml
[dependencies]
env-extract = "0.1.22"
```

Then, in your Rust code, import the procedural macros by adding the following line:

```rust
use env_extract::{EnvVar, ConfigStruct};
```

## ConfigStruct Macro

The `ConfigStruct` macro is applied to structs and derives the `ConfigStruct` trait. This trait allows for the easy retrieval of a struct from environment variables by pulling each field from the environment variables by name. The following types are valid for fields of a struct:

- `String`
- `bool`
- `u8`, `u16`, `u32`, `u64`, `u128`
- `i8`, `i16`, `i32`, `i64`, `i128`
- `f32`, `f64`
- An enum that derives `EnvVar`

The `ConfigStruct` macro supports the following attributes on the fields in the struct:

- `default`: Sets a default value for the field. If this is not provided, the macro will panic if the environment variable is not set.
- `env_var`: Sets the name of the environment variable to use for this field. If this is not provided, the macro will use the name of the field in uppercase as the environment variable name.
- `enumerated`: Identifies an enum that implements the `EnvVar` trait. The macro will parse the environment variable to the enum type.

## EnvVar Macro

The `EnvVar` macro is applied to enums and implements the `EnvVar` trait, which provides a `.get()` method to retrieve a value of type `T` from an environment variable. The macro parses the environment variable to the enum type.

The `EnvVar` macro requires one of the following conditions to be met for the enum:

- A variant called "Invalid", which will be returned if the environment variable does not match any of the variants.
- A variant marked with `#[default]`, which will be returned if the environment variable does not match any of the variants.
- The enum to be marked with `#[panic_on_invalid]`, which will panic if the environment variable does not match any of the variants.

The `EnvVar` macro allows for the following attributes on the enum itself:

- `#[env_var = "FOO"]`: Set a custom environment variable name to search for. Defaults to the name of the enum in uppercase.
- `#[panic_on_invalid]`: Panics if a valid variant is not found.
- `#[case(convert = "[uppercase|lowercase|exact|any]")]`: Converts all environment variable values to a specific case before comparing them to map the valid variant. This attribute is overwritten if the variant also contains this attribute.

The `EnvVar` macro also supports the following attributes on the enum variants:

- `#[case = "[uppercase|lowercase|exact|any]"]`: Specifies case conversion for the annotated enum variant. The `uppercase` and `lowercase` options convert the environment variable value to uppercase or lowercase before comparing it to the variant name. The `exact` option compares the environment variable value to the variant name without any case conversion. The `any` option converts both the environment variable value and the variant name to lowercase before comparing them.
- `#[default]`: Specifies the default enum variant.
- `#[ignore_variant]`: Ignores the annotated enum variant when checking for a match.

## Example Usage

```rust
use env_extract::{ConfigStruct, EnvVar};

#[derive(Debug, EnvVar)]
#[var_name = "DATABASE_TYPE"]
#[panic_on_invalid]
#[case(convert = "lowercase")]
enum DatabaseType {
    Postgres,
    Mysql,
    Sqlite,
}

#[derive(ConfigStruct, Debug)]
struct Config {
    db_host: String,
    db_port: u16,
    use_tls: bool,

    #[enumerated]
    db_type: DatabaseType,
}

fn main() {
    std::env::set_var("DB_HOST", "localhost");
    std::env::set_var("DB_PORT", "5432");
    std::env::set_var("USE_TLS", "true");
    std::env::set_var("DATABASE_TYPE", "postgres");

    let config = Config::get();

    assert_eq!(config.db_host, "localhost");
    assert_eq!(config.db_port, 5432);
    assert_eq!(config.use_tls, true);
    assert!(matches!(config.db_type, DatabaseType::Postgres));
}
```

In the example above, the `ConfigStruct` macro is used to derive the `ConfigStruct` trait for the `Config` struct, enabling easy retrieval of values from environment variables. The `EnvVar` trait is derived for the `DatabaseType` enum using the `EnvVar` macro, allowing the extraction of the enum variant from the "DATABASE_TYPE" environment variable. The environment variable values are parsed and converted according to the specified case conversions. Finally, the `Config` struct is populated with values retrieved from environment variables, and assertions are used to validate the extracted values.
