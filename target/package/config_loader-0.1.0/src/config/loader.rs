use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use dotenv::dotenv;
use std::env;
use crate::config::error::ConfigError;

/// Loads configuration from a file and overrides with environment variables.
pub fn load_config<T>(file_path: &str) -> Result<T, ConfigError>
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
    dotenv().ok(); // Load environment variables from .env file

    let path = Path::new(file_path);
    let mut file = File::open(path).map_err(|_| ConfigError::FileNotFound)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|e| ConfigError::ParseError(e.to_string()))?;

    let mut config: T = match path.extension().and_then(|ext| ext.to_str()) {
        Some("json") => serde_json::from_str(&contents).map_err(|e| ConfigError::ParseError(e.to_string()))?,
        Some("toml") => toml::from_str(&contents).map_err(|e| ConfigError::ParseError(e.to_string()))?,
        Some("yaml") | Some("yml") => serde_yaml::from_str(&contents).map_err(|e| ConfigError::ParseError(e.to_string()))?,
        _ => return Err(ConfigError::ParseError("Unsupported file format".to_string())),
    };

    // Skip environment variable overrides during testing
    if !cfg!(test) {
        override_with_env(&mut config)?;
    }

    Ok(config)
}

/// Overrides configuration fields with environment variables.
fn override_with_env<T>(config: &mut T) -> Result<(), ConfigError>
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
    let env_vars: HashMap<String, String> = env::vars()
        .filter(|(key, _)| key.starts_with("CONFIG_"))
        .map(|(key, value)| (key.trim_start_matches("CONFIG_").to_lowercase(), value))
        .collect();

    // Convert the config to a mutable JSON value
    let mut config_value = serde_json::to_value(&*config).map_err(|e| ConfigError::EnvVarError(e.to_string()))?;

    // Override fields with environment variables
    for (key, value) in env_vars {
        let parts: Vec<&str> = key.split('_').collect();
        if let Some(field) = find_field_mut(&mut config_value, &parts) {
            // Parse the value into the appropriate type
            match field {
                Value::Number(_) => {
                    if let Ok(number) = value.parse::<i64>() {
                        *field = Value::Number(number.into());
                    } else if let Ok(number) = value.parse::<f64>() {
                        *field = Value::Number(serde_json::Number::from_f64(number).unwrap());
                    }
                }
                Value::String(_) => {
                    *field = Value::String(value);
                }
                Value::Bool(_) => {
                    if let Ok(boolean) = value.parse::<bool>() {
                        *field = Value::Bool(boolean);
                    }
                }
                _ => {}
            }
        }
    }

    // Deserialize the updated JSON value back into the config
    *config = serde_json::from_value(config_value).map_err(|e| ConfigError::EnvVarError(e.to_string()))?;
    Ok(())
}

/// Recursively finds a nested field in a JSON value.
fn find_field_mut<'a>(value: &'a mut Value, parts: &[&str]) -> Option<&'a mut Value> {
    if parts.is_empty() {
        return Some(value);
    }

    match value {
        Value::Object(map) => {
            if let Some(next_value) = map.get_mut(parts[0]) {
                find_field_mut(next_value, &parts[1..])
            } else {
                None
            }
        }
        _ => None,
    }
}