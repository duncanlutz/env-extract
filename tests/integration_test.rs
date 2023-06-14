#[cfg(test)]
mod tests {
    use env_extract::{ConfigStruct, EnvVar};
    use std::env;

    #[derive(EnvVar)]
    enum LogLevel {
        Error,
        Warning,
        Info,
        Invalid,
    }

    #[test]
    fn test_exact_match() {
        env::set_var("LOGLEVEL", "Error");
        assert!(matches!(LogLevel::get(), LogLevel::Error));

        env::set_var("LOGLEVEL", "Warning");
        assert!(matches!(LogLevel::get(), LogLevel::Warning));

        env::set_var("LOGLEVEL", "Info");
        assert!(matches!(LogLevel::get(), LogLevel::Info));
    }

    #[derive(EnvVar)]
    enum LogLevelUppercase {
        #[case(convert = "uppercase")]
        Error,
        #[case(convert = "uppercase")]
        Warning,
        #[case(convert = "uppercase")]
        Info,
        Invalid,
    }

    #[test]
    fn test_case_conversion_uppercase() {
        env::set_var("LOGLEVELUPPERCASE", "ERROR");
        assert!(matches!(LogLevelUppercase::get(), LogLevelUppercase::Error));

        env::set_var("LOGLEVELUPPERCASE", "WARNING");
        assert!(matches!(
            LogLevelUppercase::get(),
            LogLevelUppercase::Warning
        ));

        env::set_var("LOGLEVELUPPERCASE", "INFO");
        assert!(matches!(LogLevelUppercase::get(), LogLevelUppercase::Info));
    }

    #[derive(EnvVar)]
    enum LogLevelLowercase {
        #[case(convert = "lowercase")]
        Error,
        #[case(convert = "lowercase")]
        Warning,
        #[case(convert = "lowercase")]
        Info,
        Invalid,
    }

    #[test]
    fn test_case_conversion_lowercase() {
        env::set_var("LOGLEVELLOWERCASE", "error");
        assert!(matches!(LogLevelLowercase::get(), LogLevelLowercase::Error));

        env::set_var("LOGLEVELLOWERCASE", "warning");
        assert!(matches!(
            LogLevelLowercase::get(),
            LogLevelLowercase::Warning
        ));

        env::set_var("LOGLEVELLOWERCASE", "info");
        assert!(matches!(LogLevelLowercase::get(), LogLevelLowercase::Info));
    }

    #[derive(EnvVar)]
    enum LogLevelAny {
        #[case(convert = "any")]
        Error,
        #[case(convert = "any")]
        Warning,
        #[case(convert = "any")]
        Info,
        Invalid,
    }

    #[test]
    fn test_case_conversion_any() {
        env::set_var("LOGLEVELANY", "ERROR");
        assert!(matches!(LogLevelAny::get(), LogLevelAny::Error));

        env::set_var("LOGLEVELANY", "error");
        assert!(matches!(LogLevelAny::get(), LogLevelAny::Error));

        env::set_var("LOGLEVELANY", "WaRnInG");
        assert!(matches!(LogLevelAny::get(), LogLevelAny::Warning));

        env::set_var("LOGLEVELANY", "warning");
        assert!(matches!(LogLevelAny::get(), LogLevelAny::Warning));

        env::set_var("LOGLEVELANY", "InFo");
        assert!(matches!(LogLevelAny::get(), LogLevelAny::Info));

        env::set_var("LOGLEVELANY", "info");
        assert!(matches!(LogLevelAny::get(), LogLevelAny::Info));
    }

    #[test]
    fn test_invalid_value() {
        env::set_var("LOGLEVEL", "aaepofanepw");
        assert!(matches!(LogLevel::get(), LogLevel::Invalid));
    }

    #[derive(EnvVar)]
    enum DefaultEnum {
        #[default]
        DefaultVariant,
        ValidVariant,
    }

    #[test]
    fn test_default_value_success() {
        std::env::set_var("DEFAULTENUM", "ValidVariant");
        let my_enum = match DefaultEnum::get() {
            DefaultEnum::ValidVariant => DefaultEnum::ValidVariant,
            _ => panic!("Invalid variant"),
        };

        // Verify the extracted enum variant
        assert!(matches!(my_enum, DefaultEnum::ValidVariant));
    }

    #[test]
    fn test_extract_enum_from_env_failure() {
        std::env::set_var("DEFAULTENUM", "InvalidVariant");
        let result = DefaultEnum::get();

        assert!(matches!(result, DefaultEnum::DefaultVariant));
    }

    #[derive(EnvVar)]
    #[var_name = "MY_ENUM_VAR"]
    enum RenamedVarEnum {
        #[default]
        DefaultVariant,
        ValidVariant,
        Invalid,
    }

    #[test]
    fn test_extract_enum_from_env_success_with_var_name() {
        std::env::set_var("MY_ENUM_VAR", "ValidVariant");
        let enum_value = RenamedVarEnum::get();
        assert!(matches!(enum_value, RenamedVarEnum::ValidVariant));
    }

    #[test]
    fn test_extract_enum_from_env_failure_with_var_name() {
        std::env::set_var("MY_ENUM_VAR", "InvalidValue");
        let enum_value = RenamedVarEnum::get();
        assert!(matches!(enum_value, RenamedVarEnum::DefaultVariant));
    }

    #[derive(EnvVar)]
    #[case(convert = "uppercase")]
    enum CasedEnum {
        #[default]
        DefaultVariant,
        ValidVariant,
        Invalid,
    }

    #[test]
    fn test_extract_enum_from_env_success_with_case_uppercase() {
        std::env::set_var("CASEDENUM", "VALIDVARIANT");
        let enum_value = CasedEnum::get();
        assert!(matches!(enum_value, CasedEnum::ValidVariant));
    }

    #[test]
    fn test_extract_enum_from_env_failure_with_case_uppercase() {
        std::env::set_var("CASEDENUM", "InvalidValue");
        let enum_value = CasedEnum::get();
        assert!(matches!(enum_value, CasedEnum::DefaultVariant));
    }

    #[derive(EnvVar)]
    #[panic_on_invalid]
    enum PanicEnum {
        VariantA,
        VariantB,
    }

    #[test]
    fn test_extract_enum_from_env_success_with_panic_on_invalid() {
        std::env::set_var("PANICENUM", "VariantA");
        let enum_value = PanicEnum::get();
        assert!(matches!(enum_value, PanicEnum::VariantA));
    }

    #[test]
    #[should_panic(expected = "Invalid environment variable value")]
    fn test_extract_enum_from_env_failure_with_panic_on_invalid() {
        std::env::set_var("PANICENUM", "InvalidValue");
        let _ = PanicEnum::get();
    }

    #[derive(Debug, EnvVar)]
    enum StructTestEnum {
        VariantA,
        VariantB,
        Invalid,
    }

    #[derive(Debug, ConfigStruct)]
    struct MyConfig {
        string_field: String,
        bool_field: bool,
        int_field: i32,

        #[enumerated]
        enum_field: StructTestEnum,
    }

    #[test]
    fn test_populate_struct_from_env_success() {
        // Set up the environment variables
        std::env::set_var("STRING_FIELD", "Hello, world!");
        std::env::set_var("BOOL_FIELD", "true");
        std::env::set_var("INT_FIELD", "42");
        std::env::set_var("STRUCTTESTENUM", "VariantA");

        // Populate the struct from the environment variables
        let config = MyConfig::get();

        // Verify the field values
        assert_eq!(config.string_field, "Hello, world!");
        assert_eq!(config.bool_field, true);
        assert_eq!(config.int_field, 42);

        println!("{:?}", config.enum_field);
        assert!(matches!(config.enum_field, StructTestEnum::VariantA));
    }

    #[test]
    #[should_panic(expected = "No environment variable or default value found for 'string_field'")]
    fn test_populate_struct_from_env_failure() {
        std::env::remove_var("STRING_FIELD"); // Missing string value
        std::env::remove_var("BOOL_FIELD"); // Missing bool value
        std::env::remove_var("INT_FIELD"); // Missing int value
        std::env::remove_var("STRUCTTESTENUM"); // Missing enum value
        MyConfig::get();
    }

    #[derive(ConfigStruct)]
    struct DefaultStruct {
        string_field: String,
        bool_field: bool,
        #[default(42)]
        int_field: i32,

        #[enumerated]
        enum_field: StructTestEnum,
    }

    #[test]
    fn test_populate_struct_from_env_with_default_success() {
        std::env::set_var("STRING_FIELD", "Hello, world!");
        std::env::set_var("BOOL_FIELD", "true");
        std::env::set_var("STRUCTTESTENUM", "VariantA");

        let config = DefaultStruct::get();

        assert_eq!(config.string_field, "Hello, world!");
        assert_eq!(config.bool_field, true);
        assert_eq!(config.int_field, 42);
        assert!(matches!(config.enum_field, StructTestEnum::VariantA));
    }
}
