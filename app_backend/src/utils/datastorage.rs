use super::config::Config;

pub struct BackendData {
    config: Config,
}

impl BackendData {

    pub fn new() -> Self{
        return BackendData {
            config: Config::new(),
        };
    }
}