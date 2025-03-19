# Config Loader

A Rust library for loading configuration files (JSON, TOML, YAML) into Rust structs with environment variable overrides and validation.

## Features

- Load configuration from JSON, TOML, or YAML files.
- Override configuration values with environment variables.
- Validate configuration using custom rules.

## Usage

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
config_loader = "0.2"

##Example Usage

```toml
use config_loader::{load_config, validate_config, Validatable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    database: DatabaseConfig,
    server: ServerConfig,
}

fn main() {
    let config: Config = load_config("config.json").unwrap();
    validate_config(&config).unwrap();
    println!("{:?}", config);
}