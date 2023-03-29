use super::config::Config;
use uuid::Uuid;
use super::networking::NetworkingData;

pub struct BackendData {
    pub config: Config,
}

impl BackendData {
    pub fn new() -> Self {
        BackendData {
            config: Config::new(),
        }
    }
}
