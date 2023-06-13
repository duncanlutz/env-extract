# env-extract

This crate provides a procedural macro for deriving the `EnvVar` trait for an enum in Rust. The `EnvVar` trait allows you to easily retrieve an enum variant based on the value of an environment variable.

## Usage

To use the `EnvVar` macro, add `env-extract` as a dependency in your `Cargo.toml` file:

```toml
[dependencies]
env-extract = "0.1.2"
```

Then, in your Rust code, import the procedural macro by adding the following line:

```rust
use env_extract::EnvVar;
```

## Deriving EnvVar

The `EnvVar` trait can be derived for an enum using the `#[derive(EnvVar)]` attribute. Each variant of the enum represents a possible value for the environment variable.

By default, the macro performs an exact case-sensitive comparison between the environment variable value and the enum variant names. However, you can specify a case conversion for individual enum variants using the `#[case]` attribute.

## Case Conversion

The `#[case]` attribute accepts a `convert` parameter with the following options:

- `"uppercase"`: Converts the environment variable value to uppercase before comparison.
- `"lowercase"`: Converts the environment variable value to lowercase before comparison.
- `"exact"`: Performs an exact case-sensitive comparison (default).
- `"any"`: Skips case comparison, treating any value of the environment variable as a match.

## Examples

```rust
use env_extract::EnvVar;

#[derive(EnvVar)]
enum LogLevel {
    #[case(convert = "uppercase")]
    Error,
    #[case(convert = "uppercase")]
    Warning,
    #[case(convert = "uppercase")]
    Info,
}

fn main() {
    match LogLevel::get() {
        Ok(LogLevel::Error) => eprintln!("An error occurred"),
        Ok(LogLevel::Warning) => eprintln!("Warning: Something may not be right"),
        Ok(LogLevel::Info) => eprintln!("Informational message"),
        Err(err) => eprintln!("Invalid log level: {}", err),
    }
}
```

In this example, the `LogLevel` enum is derived using the `EnvVar` macro. Each variant is annotated with the `#[case]` attribute and set to convert the environment variable value to uppercase before comparison. The `get()` method is then used to retrieve the appropriate log level based on the environment variable value. If the environment variable matches one of the enum variants, the corresponding action is performed. Otherwise, an error message is printed indicating an invalid log level.
