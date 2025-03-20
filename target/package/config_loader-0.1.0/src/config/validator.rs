use crate::config::error::ConfigError;

/// Trait for custom validation rules.
pub trait Validatable {
    fn validate(&self) -> Result<(), ConfigError>;
}

/// Validates the configuration using custom rules.
pub fn validate_config<T: Validatable>(config: &T) -> Result<(), ConfigError> {
    config.validate()
}