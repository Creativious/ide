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
        let config_filepaths = ["app_config.toml"];

        let content = config_filepaths.iter()
            .filter_map(|path| fs::read_to_string(path).ok())
            .next()
            .unwrap_or_default();

        let config_toml: ConfigToml = toml::from_str(&content).unwrap_or_default();

        let networking = config_toml.networking.unwrap_or_default();

        let backend_address = networking.backend_address.unwrap_or("localhost".to_string());
        let backend_port = networking.backend_port.unwrap_or(3365);
        let max_network_listeners = networking.experimental
            .map_or(1, |experimental| experimental.max_network_listeners.unwrap_or(1));

        Config {
            backend_address,
            backend_port,
            max_network_listeners,
        }
    }
}

