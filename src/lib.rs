//! # env_extract
//! A simple library for extracting environment variables into a more usable format.
//! This library is especially useful for extracting environment variables into enums.
//! 
//! # Examples
//! 
//! ```
//! use env_extract::EnumVariableBuilder;
//! 
//! std::env::set_var("foo", "bar");
//! 
//! #[derive(Debug, Default, Clone)]
//! enum Foo {
//!     #[default]
//!     Bar,
//!     Baz,
//! }
//! 
//! let foo = EnumVariableBuilder::default()
//!     .name("foo".to_string())
//!     .add_option("bar".to_string(), Foo::Bar)
//!     .add_option("baz".to_string(), Foo::Baz)
//!     .build()
//!     .unwrap();
//! 
//! assert!(matches!(foo, Foo::Bar));
//! ```
//! 
//! ```
//! use env_extract::AnyVariableBuilder;
//! 
//! std::env::set_var("foo", "bar");
//! 
//! let foo = AnyVariableBuilder::default()
//!    .name("foo".to_string())
//!    .build()
//!    .unwrap();
//! 
//! assert_eq!(foo, "bar");
//! ```

pub use crate::types::{AnyEnvironmentVariable, AnyVariableBuilder, EnumEnvironmentVariable, EnumVariableBuilder, EnvironmentVariable};

pub mod types {

    /// Validates that a given environment variable is set to a valid value, and returns the matching Enum value.
    /// The valid arguments are a vector of tuples containing the string value and the Enum value.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use env_extract::EnumEnvironmentVariable;
    /// 
    /// std::env::set_var("foo", "bar");
    /// 
    /// #[derive(Debug, Default, Clone)]
    /// enum Foo {
    ///    #[default]
    ///    Bar,
    ///    Baz,
    /// }
    /// 
    /// let foo = EnumEnvironmentVariable::new("foo".to_string(), vec![
    ///    ("bar".to_string(), Foo::Bar),
    ///    ("baz".to_string(), Foo::Baz),
    /// ]).validate().unwrap();
    /// 
    /// assert!(matches!(foo, Foo::Bar));
    /// ```
    #[derive(Debug, Default)]
    pub struct EnumEnvironmentVariable<T> {
        pub name: String,
        pub valid_options: Vec<(String, T)>,
    }

    /// Creates a new EnumEnvironmentVariable. Use this if you want to use the builder pattern.
    /// The valid arguments are a vector of tuples containing the string value and the Enum value.
    ///
    /// # Examples
    ///
    /// ```
    /// use env_extract::EnumVariableBuilder;
    ///
    /// std::env::set_var("foo", "bar");
    ///
    /// #[derive(Debug, Default, Clone)]
    /// enum Foo {
    ///     #[default]
    ///     Bar,
    ///     Baz,
    /// }
    ///
    /// let foo = EnumVariableBuilder::default()
    ///     .name("foo".to_string())
    ///     .add_option("bar".to_string(), Foo::Bar)
    ///     .add_option("baz".to_string(), Foo::Baz)
    ///     .build()
    ///     .unwrap();
    ///
    /// assert!(matches!(foo, Foo::Bar));
    #[derive(Debug, Default)]
    pub struct EnumVariableBuilder<T> {
        pub name: String,
        pub valid_options: Vec<(String, T)>,
    }

    /// Validates that a given environment variable is set to a valid value.
    ///
    /// # Examples
    ///
    /// ```
    /// use env_extract::AnyEnvironmentVariable;
    ///
    /// std::env::set_var("foo", "bar");
    ///
    /// let foo = AnyEnvironmentVariable::new("foo".to_string());
    ///
    /// assert_eq!(foo.validate().unwrap(), "bar");
    #[derive(Debug)]
    pub struct AnyEnvironmentVariable {
        pub name: String,
    }

    /// Creates a new AnyEnvironmentVariable. Use this if you want to use the builder pattern.
    ///
    /// # Examples
    ///
    /// ```
    /// use env_extract::AnyVariableBuilder;
    ///
    /// std::env::set_var("foo", "bar");
    ///
    /// let foo = AnyVariableBuilder::default()
    ///     .name("foo".to_string())
    ///     .build()
    ///     .unwrap();
    ///
    /// assert_eq!(foo, "bar");
    #[derive(Debug, Default)]
    pub struct AnyVariableBuilder {
        pub name: String,
    }

    /// Generic struct for creating and validating environment variables.
    /// It is recommended to use either the AnyVariableBuilder or EnumVariableBuilder instead of this struct.
    ///
    /// # Examples
    ///
    /// ```
    /// use env_extract::EnvironmentVariable;
    ///
    /// std::env::set_var("foo", "bar");
    ///
    /// let foo = EnvironmentVariable::<String>::new_any_value_variable("foo".to_string()).unwrap();
    ///
    /// assert_eq!(foo, "bar");
    #[derive(Debug, Default)]
    pub struct EnvironmentVariable<T> {
        pub name: String,
        pub valid_options: Option<Vec<(String, T)>>,
    }
}

mod environment_variables {
    pub use std::{cmp, error::Error};
    use crate::types::{AnyEnvironmentVariable, AnyVariableBuilder, EnumEnvironmentVariable, EnumVariableBuilder, EnvironmentVariable};

    impl<T: Clone> EnumEnvironmentVariable<T> {
        /// Creates a new EnumEnvironmentVariable with the specified name and valid arguments.
        /// The valid arguments are a vector of tuples containing the string value and the Enum value.
        ///
        /// # Examples
        ///
        /// ```
        /// use env_extract::EnumEnvironmentVariable;
        ///
        /// std::env::set_var("foo", "bar");
        ///
        /// #[derive(Debug, Default, Clone)]
        /// enum Foo{
        ///     #[default]
        ///     Bar,
        ///     Baz,
        /// }
        ///
        /// let foo = EnumEnvironmentVariable::new("foo".to_string(), vec![
        ///     ("bar".to_string(), Foo::Bar),
        ///     ("baz".to_string(), Foo::Baz),
        ///     ]);
        ///
        /// assert!(foo.validate().is_ok());
        pub fn new(name: String, valid_options: Vec<(String, T)>) -> Self {
            Self {
                name: name,
                valid_options: valid_options,
            }
        }

        /// Validates a given environment variable and returns the matching Enum value if it is valid.
        ///
        /// # Errors
        ///
        /// Returns an error if the environment variable is not set, or if the value is not a valid value.
        ///
        /// # Examples
        ///
        /// ```
        /// use env_extract::EnumVariableBuilder;
        ///
        /// std::env::set_var("foo", "bar");
        ///
        /// #[derive(Debug, Default, Clone)]
        /// enum Foo {
        ///     #[default]
        ///     Bar,
        ///     Baz,
        /// }
        ///
        /// let foo = EnumVariableBuilder::default()
        ///     .name("foo".to_string())
        ///     .add_option("bar".to_string(), Foo::Bar)
        ///     .add_option("baz".to_string(), Foo::Baz)
        ///     .build()
        ///     .unwrap();
        ///
        /// assert!(matches!(foo, Foo::Bar));
        /// ```
        pub fn validate(&self) -> Result<T, Box<dyn std::error::Error>> {
            let variable = match std::env::var(&self.name) {
                Ok(name) => name,
                Err(e) => return Err(Box::from(e)),
            };

            let result: Option<T> = match &self.valid_options.len().cmp(&0) {
                cmp::Ordering::Equal => {
                    return Err(Box::from(format!(
                        "Length of valid_options was zero in environment variable '{}'",
                        &self.name
                    )))
                }
                cmp::Ordering::Less => {
                    return Err(Box::from(format!(
                "Length of ValidArguments was somehow less than zero in environment variable '{}'",
                &self.name
            )))
                }
                cmp::Ordering::Greater => {
                    let filtered_args: Vec<&(String, T)> = self
                        .valid_options
                        .iter()
                        .filter(|(n, _e)| n == &variable)
                        .collect();
                    match filtered_args.len().cmp(&1) {
                        cmp::Ordering::Equal => Some(filtered_args[0].1.clone()),
                        cmp::Ordering::Greater => return Err(Box::from(format!("Found more than one ValidArgument with the same variable name for environment variable '{}'", &self.name))),
                        cmp::Ordering::Less => return Err(Box::from(format!("Could not find valid Enum value with result '{}' for environment variable '{}'. Check that variable is set to a valid value.", variable, &self.name)))
                    }
                }
            };

            match result {
                Some(v) => Ok(v),
                None => {
                    return Err(Box::from(format!(
                        "Found invalid value '{}' for environment variable '{}'",
                        variable, &self.name
                    )))
                }
            }
        }
    }

    impl<T: Clone> EnumVariableBuilder<T> {
        /// Creates a new EnumVariableBuilder with the specified name and valid arguments.
        /// The valid arguments are a vector of tuples containing the string value and the Enum value.
        ///
        /// # Examples
        ///
        /// ```
        /// use env_extract::EnumVariableBuilder;
        ///
        /// std::env::set_var("foo", "bar");
        ///
        /// #[derive(Debug, Default, Clone)]
        /// enum Foo{
        ///     #[default]
        ///     Bar,
        ///     Baz,
        /// }
        ///
        /// let foo = EnumVariableBuilder::new("foo".to_string(), vec![
        ///     ("bar".to_string(), Foo::Bar),
        ///     ("baz".to_string(), Foo::Baz),
        ///     ])
        ///     .build()
        ///     .unwrap();
        ///
        /// assert!(matches!(foo, Foo::Bar));
        pub fn new(name: String, valid_options: Vec<(String, T)>) -> Self {
            Self {
                name: name,
                valid_options: valid_options,
            }
        }

        /// Sets the name of the environment variable.
        pub fn name(&mut self, value: String) -> &mut Self {
            self.name = value;
            self
        }

        /// Sets the valid arguments of the environment variable (this overwrites any current arguments with a new Vector).
        pub fn set_arguments(&mut self, values: Vec<(String, T)>) -> &mut Self {
            self.valid_options = values;
            self
        }

        /// Adds a valid argument to the environment variable.
        pub fn add_option(&mut self, string_value: String, enum_value: T) -> &mut Self {
            self.valid_options.push((string_value, enum_value));
            self
        }

        /// Builds the EnumEnvironmentVariable, then validates it.
        pub fn build(&self) -> Result<T, Box<dyn Error>> {
            EnvironmentVariable::new_enumerated_variable(
                self.name.clone(),
                self.valid_options.clone(),
            )
        }
    }

    impl AnyEnvironmentVariable {
        /// Creates a new AnyEnvironmentVariable with the specified name.
        pub fn new(name: String) -> Self {
            Self { name }
        }

        /// Validates the environment variable, returning the value if it is valid.
        pub fn validate(&self) -> Result<String, Box<dyn Error>> {
            match std::env::var(&self.name) {
                Ok(val) => Ok(val),
                Err(e) => Err(Box::from(e)),
            }
        }
    }

    impl AnyVariableBuilder {
        /// Creates a new AnyVariableBuilder with the specified name.
        pub fn new(name: String) -> Self {
            Self { name: name }
        }

        /// Sets the name of the environment variable.
        pub fn name(&mut self, value: String) -> &mut Self {
            self.name = value;
            self
        }

        /// Sets the name of the environment variable.
        pub fn build(&self) -> Result<String, Box<dyn Error>> {
            EnvironmentVariable::<String>::new_any_value_variable(self.name.clone())
        }
    }

    impl<T: Clone> EnvironmentVariable<T> {
        /// Creates a new EnumEnvironmentVariable with the specified name and valid arguments, then validates it.
        ///
        /// # Examples
        ///
        /// ```
        /// use env_extract::EnvironmentVariable;
        ///
        /// std::env::set_var("foo", "bar");
        ///
        /// #[derive(Debug, Default, Clone)]
        /// enum Foo{
        ///     #[default]
        ///     Bar,
        ///     Baz,
        /// }
        ///
        /// let foo = EnvironmentVariable::<Foo>::new_enumerated_variable("foo".to_string(), vec![
        ///    ("bar".to_string(), Foo::Bar),
        ///    ("baz".to_string(), Foo::Baz),
        /// ]).unwrap();
        ///
        /// assert!(matches!(foo, Foo::Bar));
        pub fn new_enumerated_variable(
            name: String,
            valid_options: Vec<(String, T)>,
        ) -> Result<T, Box<dyn Error>> {
            match EnumEnvironmentVariable::new(name, valid_options).validate() {
                Ok(v) => Ok(v),
                Err(e) => Err(e),
            }
        }

        /// Creates a new AnyEnvironmentVariable with the specified name, then validates it.
        ///
        /// # Examples
        ///
        /// ```
        /// use env_extract::EnvironmentVariable;
        ///
        /// std::env::set_var("foo", "bar");
        ///
        /// let foo = EnvironmentVariable::<String>::new_any_value_variable("foo".to_string()).unwrap();
        ///
        /// assert_eq!(foo, "bar");
        pub fn new_any_value_variable(name: String) -> Result<String, Box<dyn Error>> {
            match AnyEnvironmentVariable::new(name).validate() {
                Ok(v) => Ok(v),
                Err(e) => Err(e),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{AnyVariableBuilder, EnumVariableBuilder};

    #[derive(Debug, Default, Clone)]
    enum Hello {
        #[default]
        World,
        There,
        Honey,
    }

    #[test]
    fn enum_variable_returns_correct() {
        std::env::set_var("hello", "world");

        assert!(matches!(
            EnumVariableBuilder::default()
                .name(String::from("hello"))
                .add_option(String::from("world"), Hello::World)
                .add_option(String::from("there"), Hello::There)
                .add_option(String::from("honey"), Hello::Honey)
                .build()
                .unwrap(),
            Hello::World
        ));
    }

    #[test]
    fn invalid_enum_variable_value_fails() {
        std::env::set_var("hello", "ap;wo3infapowei");

        assert!(EnumVariableBuilder::default()
            .name(String::from("hello"))
            .add_option(String::from("world"), Hello::World)
            .add_option(String::from("there"), Hello::There)
            .add_option(String::from("honey"), Hello::Honey)
            .build()
            .is_err())
    }

    #[test]
    fn any_type_variable_returns_correct() {
        std::env::set_var("hello", "honey");

        assert_eq!(
            AnyVariableBuilder::new(String::from("hello"))
                .build()
                .unwrap(),
            String::from("honey")
        );
    }
}
