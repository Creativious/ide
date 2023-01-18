use std::default;
use std::default::Default;
use std::fs;
use std::io::Error as IoError;
use toml;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct ConfigTomlNetworking {
    backend_address: Option<String>,
    backend_port: Option<u16>,
}

impl Default for ConfigTomlNetworking {
    fn default() -> Self {
        ConfigTomlNetworking { 
            backend_address: Some("localhost".to_string()),
            backend_port: Some(3365) }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct ConfigToml {
    networking: Option<ConfigTomlNetworking>,
}

impl Default for ConfigToml {
    fn default() -> Self {
        ConfigToml { networking: Some(ConfigTomlNetworking::default()) }
    }
}


#[derive(Debug)]
pub struct Config {
    pub backend_address: String,
    pub backend_port: u16,
}

impl Config {
    pub fn new() -> Self {

        let config_filepaths: [&str; 1] = [
            "app_config.toml"
        ];

        let mut content: String = "".to_owned();

        for filepath in config_filepaths {
            let result: Result<String, IoError> = fs::read_to_string(filepath);

            if result.is_ok() {
                content = result.unwrap();
                break;
            }
        }

        let config_toml: ConfigToml = toml::from_str(&content).unwrap_or_else(|_| {return ConfigToml::default();});

        let (backend_address, backend_port): (String, u16) = match config_toml.networking {
            Some(networking) => (networking.backend_address.unwrap_or_else(|| {ConfigTomlNetworking::default().backend_address.unwrap()}), networking.backend_port.unwrap_or_else(|| {ConfigTomlNetworking::default().backend_port.unwrap()})),
            None => (ConfigTomlNetworking::default().backend_address.unwrap(), ConfigTomlNetworking::default().backend_port.unwrap()),
        };

        return Config {
            backend_address: backend_address,
            backend_port: backend_port
        };
    }
}

pub fn setup() -> Config{
    return Config::new();
}