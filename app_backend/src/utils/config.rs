use std::default;
use std::default::Default;
use std::fs;
use std::io::Error as IoError;
use toml;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct ConfigTomlExperimentalNetworking {
    max_network_listeners: Option<u8>,
}

impl Default for ConfigTomlExperimentalNetworking {
    fn default() -> Self {
        ConfigTomlExperimentalNetworking { max_network_listeners: Some(1) }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct ConfigTomlNetworking {
    backend_address: Option<String>,
    backend_port: Option<u16>,
    experimental: Option<ConfigTomlExperimentalNetworking>,
}

impl Default for ConfigTomlNetworking {
    fn default() -> Self {
        ConfigTomlNetworking { 
            backend_address: Some("localhost".to_string()),
            backend_port: Some(3365),
            experimental: Some(ConfigTomlExperimentalNetworking::default()),
    }
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
    pub max_network_listeners: u8,
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

        let (backend_address, backend_port): (String, u16) = match &config_toml.networking {
            Some(networking) => (networking.backend_address.clone().unwrap_or_else(|| {ConfigTomlNetworking::default().backend_address.unwrap()}), networking.backend_port.unwrap_or_else(|| {ConfigTomlNetworking::default().backend_port.unwrap()})),
            None => (ConfigTomlNetworking::default().backend_address.unwrap(), ConfigTomlNetworking::default().backend_port.unwrap()),
        };

        let max_network_listeners: u8 = match &config_toml.networking.unwrap_or_default().experimental {
            Some(experimental) => experimental.max_network_listeners.unwrap_or_else(|| ConfigTomlExperimentalNetworking::default().max_network_listeners.unwrap()),
            None => ConfigTomlExperimentalNetworking::default().max_network_listeners.unwrap(),
        };
        return Config {
            backend_address: backend_address,
            backend_port: backend_port,
            max_network_listeners: max_network_listeners,
        };
    }
}