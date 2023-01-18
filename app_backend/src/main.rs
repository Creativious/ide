#[allow(unused_must_use)]
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_imports)]

mod modules;
use modules::{config, config::Config};
#[tokio::main]
async fn main() {
    let mut _backend_data: BackendData = BackendData::new();

    // let listener = TcpListener::bind("localhost:3365").await.unwrap();

    // let (mut socket, _addr) = listener.accept().await.unwrap();

    // let mut buffer = [0u8; 1024];

    // let bytes_read = socket.read(&mut buffer).await.unwrap();

    // socket.write_all(&buffer[..bytes_read]).await.unwrap();

    
}

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