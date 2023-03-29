use bytes::{BytesMut, BufMut};
use uuid::Uuid;
use super::config::Config;

#[derive(Clone)]
pub struct Listener {
    pub active_connection: bool,
    pub connected_address: Option<String>,
    pub connected_port: Option<u16>,
    pub listener_uuid: Uuid,
}

impl Listener {
    fn new(addr: String, port: u16) -> Self {
        Listener {
            active_connection: true,
            connected_address: Some(addr),
            connected_port: Some(port),
            listener_uuid: Uuid::new_v4(),
        }
    }
}

pub struct NetworkListeners {
    listeners: [Option<Listener>; 10],
    listener_count: u8,
}

impl NetworkListeners {
    fn new() -> Self {
        NetworkListeners {
            listeners: Default::default(),
            listener_count: 0,
        }
    }

    pub fn get_listener_count(&self) -> u8 {
        self.listener_count
    }

    pub fn create_listener(&mut self, addr: String, port: u16) -> Option<usize> {
        if self.listener_count >= self.listeners.len() as u8 {
            return None;
        }
        let index = self.listener_count as usize;
        self.listeners[index] = Some(Listener::new(addr, port));
        self.listener_count += 1;
        Some(index)
    }

    pub fn get_listener_by_uuid(&mut self, uuid: Uuid) -> Option<&mut Listener> {
        self.listeners.iter_mut().flatten().find(|l| l.listener_uuid == uuid)
    }

    pub fn get_listener(&mut self, id: usize) -> &mut Option<Listener> {
        &mut self.listeners[id]
    }

    pub fn remove_listener(&mut self, id: usize) {
        self.listeners[id] = None;
    }
}

pub struct NetworkingData {
    pub active_listeners: NetworkListeners,
}

impl Default for NetworkingData {
    fn default() -> Self {
        NetworkingData {
            active_listeners: NetworkListeners::new(),
        }
    }
}

pub struct Networking {
    pub networking_data: NetworkingData,
}

impl Networking {
    pub fn new() -> Self{
        Networking {
            networking_data: NetworkingData::default(),
        }
    }

    pub fn create_listener(&mut self, addr: String, port: u16) -> Uuid {
        let listener_index = self.networking_data.active_listeners.create_listener(
            addr,
            port,
        ).unwrap();

        self.networking_data.active_listeners
            .get_listener(listener_index)
            .as_ref()
            .unwrap()
            .listener_uuid
    }
}


pub async fn _send_message(addr: String, port: u16, message: BytesMut) -> bool {
    true
}

pub async fn _create_listener(addr: String, port: u16) -> bool {
    false
}
