#[derive(Debug)]
pub enum CliError {
    Invalid(&'static str, &'static str),
    Missing(&'static str),
    Unknown(String),
}

impl std::fmt::Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CliError::Invalid(arg, details) => {
                write!(f, "Invalid argument provided for `{}`: {}", arg, details)
            }
            CliError::Missing(arg) => write!(f, "Missing argument: `{}`", arg),
            CliError::Unknown(arg) => write!(f, "Unknown argument: `{}`", arg),
        }
    }
}

impl std::error::Error for CliError {}
