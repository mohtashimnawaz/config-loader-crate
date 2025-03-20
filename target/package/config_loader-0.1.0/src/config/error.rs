use std::fmt;

#[derive(Debug)]
pub enum ConfigError {
    FileNotFound,
    ParseError(String),
    MissingField(String),
    EnvVarError(String),
    ValidationError(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::FileNotFound => write!(f, "Configuration file not found"),
            ConfigError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            ConfigError::MissingField(field) => write!(f, "Missing required field: {}", field),
            ConfigError::EnvVarError(msg) => write!(f, "Environment variable error: {}", msg),
            ConfigError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl std::error::Error for ConfigError {}