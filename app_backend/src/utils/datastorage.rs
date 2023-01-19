use super::config::Config;
use std::default;
use std::default::Default;
use std::fs;
use std::io::Error as IoError;
use toml::map::IntoIter;
use uuid::Uuid;

// use serde::{Serialize, Deserialize};

#[derive(Clone)]
pub struct Listener { // @TODO: Add a queue system for each listener for both S2C and C2S (Server to Client & Client to Server)
    pub active_connection: bool,
    pub connected_address: Option<String>,
    pub connected_port: Option<u16>,

}


impl Listener { 
    fn new (addr: String, port: u16) -> Self{
        Listener {
            active_connection: true,
            connected_address: Some(addr),
            connected_port: Some(port)
            }
    }
    
}

pub struct NetworkListeners {
    listeners: [Option<Listener>; 10],
    listener_count: u8,
}

impl NetworkListeners { // @TODO: Add an implementation, AFTER a queue system is added to Listeners, for adding to the queue and reading to it and all that fun stuff
    fn new() -> Self{
        NetworkListeners {
             listeners: Default::default(),
             listener_count: 0,
         }
    }

    pub fn get_listener_count(&self) -> u8 {
        self.listener_count
    }

    pub fn _create_listener(&mut self, addr: String, port: u16) -> usize {
        let i = 0;
        for listener in &mut self.listeners {
            if listener.is_some() {
                &i + 1;
                continue;
            } else {
                self.listeners[0] = Some(Listener::new(addr, port));
                break;
            }
        }
        return i;
    }

    pub fn get_listener(&mut self, id: usize) -> &mut Option<Listener>{
        return &mut self.listeners[id];
    }

    pub fn remove_listener(&mut self, id: usize) {
        self.listeners[0] = None;
    }
}


pub struct NetworkingData {
    pub active_listeners: NetworkListeners,
}

impl Default for NetworkingData {
    fn default() -> Self {
        return NetworkingData {
            active_listeners: NetworkListeners::new(),
        }
    }
}


pub struct BackendData {
    pub config: Config,
    pub networking_data: NetworkingData,
}

impl BackendData {

    pub fn new() -> Self{
        return BackendData {
            config: Config::new(),
            networking_data: NetworkingData::default(),
        };
    }
}