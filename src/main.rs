use env_extract::EnvVar;

#[derive(EnvVar)]
enum LogLevel {
    Error,
    #[case(convert = "any")]
    Warning,
    Info,
}

fn main() {
    std::env::set_var("LOGLEVEL", "poaiweh");

    let log_level = LogLevel::get().unwrap();

    match log_level {
        LogLevel::Error => eprintln!("An error occurred"),
        LogLevel::Warning => eprintln!("Warning: Something may not be right"),
        LogLevel::Info => eprintln!("Informational message"),
    }
}
