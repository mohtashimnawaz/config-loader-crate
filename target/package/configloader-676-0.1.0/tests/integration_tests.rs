use config_loader::{load_config, validate_config, Validatable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct DatabaseConfig {
    host: String,
    port: u16, // Ensure this is u16
}

#[derive(Debug, Deserialize, Serialize)]
struct ServerConfig {
    host: String,
    port: u16, // Ensure this is u16
}

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    database: DatabaseConfig,
    server: ServerConfig,
}

impl Validatable for Config {
    fn validate(&self) -> Result<(), config_loader::ConfigError> {
        if self.database.host.is_empty() {
            return Err(config_loader::ConfigError::ValidationError("Database host is required".to_string()));
        }
        if self.server.port == 0 {
            return Err(config_loader::ConfigError::ValidationError("Server port must be non-zero".to_string()));
        }
        Ok(())
    }
}

#[test]
fn test_load_json_config() {
    let config: Config = load_config("tests/test_config.json").unwrap();
    assert_eq!(config.database.host, "localhost"); // Updated to expect "localhost"
    assert_eq!(config.database.port, 5432);
    assert_eq!(config.server.host, "0.0.0.0");
    assert_eq!(config.server.port, 8080);
}

#[test]
fn test_load_toml_config() {
    let config: Config = load_config("tests/test_config.toml").unwrap();
    assert_eq!(config.database.host, "localhost"); // Updated to expect "localhost"
    assert_eq!(config.database.port, 5432);
    assert_eq!(config.server.host, "0.0.0.0");
    assert_eq!(config.server.port, 8080);
}

#[test]
fn test_load_yaml_config() {
    let config: Config = load_config("tests/test_config.yaml").unwrap();
    assert_eq!(config.database.host, "localhost"); // Updated to expect "localhost"
    assert_eq!(config.database.port, 5432);
    assert_eq!(config.server.host, "0.0.0.0");
    assert_eq!(config.server.port, 8080);
}

#[test]
fn test_custom_validation() {
    let config = Config {
        database: DatabaseConfig {
            host: "".to_string(),
            port: 5432,
        },
        server: ServerConfig {
            host: "0.0.0.0".to_string(),
            port: 0,
        },
    };
    let result = validate_config(&config);
    assert!(result.is_err());
}