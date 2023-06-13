#[cfg(test)]
mod tests {
    use env_extract::EnvVar;
    use std::env;

    #[derive(EnvVar)]
    enum LogLevel {
        Error,
        Warning,
        Info,
    }

    #[test]
    fn test_exact_match() {
        env::set_var("LOGLEVEL", "Error");
        assert!(matches!(LogLevel::get().unwrap(), LogLevel::Error));

        env::set_var("LOGLEVEL", "Warning");
        assert!(matches!(LogLevel::get().unwrap(), LogLevel::Warning));

        env::set_var("LOGLEVEL", "Info");
        assert!(matches!(LogLevel::get().unwrap(), LogLevel::Info));
    }

    #[derive(EnvVar)]
    enum LogLevelUppercase {
        #[case(convert = "uppercase")]
        Error,
        #[case(convert = "uppercase")]
        Warning,
        #[case(convert = "uppercase")]
        Info,
    }

    #[test]
    fn test_case_conversion_uppercase() {
        env::set_var("LOGLEVELUPPERCASE", "ERROR");
        assert!(matches!(
            LogLevelUppercase::get().unwrap(),
            LogLevelUppercase::Error
        ));

        env::set_var("LOGLEVELUPPERCASE", "WARNING");
        assert!(matches!(
            LogLevelUppercase::get().unwrap(),
            LogLevelUppercase::Warning
        ));

        env::set_var("LOGLEVELUPPERCASE", "INFO");
        assert!(matches!(
            LogLevelUppercase::get().unwrap(),
            LogLevelUppercase::Info
        ));
    }

    #[derive(EnvVar)]
    enum LogLevelLowercase {
        #[case(convert = "lowercase")]
        Error,
        #[case(convert = "lowercase")]
        Warning,
        #[case(convert = "lowercase")]
        Info,
    }

    #[test]
    fn test_case_conversion_lowercase() {
        env::set_var("LOGLEVELLOWERCASE", "error");
        assert!(matches!(
            LogLevelLowercase::get().unwrap(),
            LogLevelLowercase::Error
        ));

        env::set_var("LOGLEVELLOWERCASE", "warning");
        assert!(matches!(
            LogLevelLowercase::get().unwrap(),
            LogLevelLowercase::Warning
        ));

        env::set_var("LOGLEVELLOWERCASE", "info");
        assert!(matches!(
            LogLevelLowercase::get().unwrap(),
            LogLevelLowercase::Info
        ));
    }

    #[derive(EnvVar)]
    enum LogLevelAny {
        #[case(convert = "any")]
        Error,
        #[case(convert = "any")]
        Warning,
        #[case(convert = "any")]
        Info,
    }

    #[test]
    fn test_case_conversion_any() {
        env::set_var("LOGLEVELANY", "ERROR");
        assert!(matches!(LogLevelAny::get().unwrap(), LogLevelAny::Error));

        env::set_var("LOGLEVELANY", "error");
        assert!(matches!(LogLevelAny::get().unwrap(), LogLevelAny::Error));

        env::set_var("LOGLEVELANY", "WaRnInG");
        assert!(matches!(LogLevelAny::get().unwrap(), LogLevelAny::Warning));

        env::set_var("LOGLEVELANY", "warning");
        assert!(matches!(LogLevelAny::get().unwrap(), LogLevelAny::Warning));

        env::set_var("LOGLEVELANY", "InFo");
        assert!(matches!(LogLevelAny::get().unwrap(), LogLevelAny::Info));

        env::set_var("LOGLEVELANY", "info");
        assert!(matches!(LogLevelAny::get().unwrap(), LogLevelAny::Info));
    }

    #[test]
    fn test_invalid_value() {
        env::set_var("LOGLEVEL", "aaepofanepw");
        assert!(LogLevel::get().is_err());
    }
}
