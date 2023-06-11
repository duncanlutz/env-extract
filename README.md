env_extract
env_extract is a Rust library that provides a convenient way to extract environment variables into a more usable format. It is particularly useful for extracting environment variables into enums.

Usage
To use env_extract, add it as a dependency in your Cargo.toml file:

```toml
[dependencies]
env_extract = "0.1.0"
```

Then, you can import the necessary types from the env_extract crate into your Rust code:

```rust
use env_extract::{EnumVariableBuilder, AnyVariableBuilder};
```

Examples
Here are a couple of examples to demonstrate how to use env_extract:

Extracting an Enum Environment Variable

```rust
use env_extract::EnumVariableBuilder;

std::env::set_var("foo", "bar");

#[derive(Debug, Default, Clone)]
enum Foo {
    #[default]
    Bar,
    Baz,
}

let foo = EnumVariableBuilder::default()
    .name("foo".to_string())
    .add_option("bar".to_string(), Foo::Bar)
    .add_option("baz".to_string(), Foo::Baz)
    .build()
    .unwrap();

assert!(matches!(foo, Foo::Bar));
```

Extracting Any Environment Variable

```rust
use env_extract::AnyVariableBuilder;

std::env::set_var("foo", "bar");

let foo = AnyVariableBuilder::default()
   .name("foo".to_string())
   .build()
   .unwrap();

assert_eq!(foo, "bar");
```

Crate Features
None
For more details and additional options, please refer to the full documentation.

License
This crate is distributed under the terms of the MIT license.

Please feel free to contribute by opening issues or submitting pull requests on the GitHub repository.

If you have any questions or need further assistance, don't hesitate to reach out.

Enjoy using env_extract!
