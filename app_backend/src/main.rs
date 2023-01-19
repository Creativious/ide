#[allow(unused_must_use)]
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_imports)]

mod utils;
use utils::datastorage::BackendData;
#[tokio::main]
async fn main() {
    let mut _backend_data: BackendData = BackendData::new();

    // @TODO: Add networking support

    // let listener =_backend_data.networking_data.active_listeners.add_listener("localhost".to_string(), 3365);

    // println!("{}", listener.as_ref().unwrap().listener_uuid.unwrap().as_urn())
    // let listener = TcpListener::bind("localhost:3365").await.unwrap();

    // let (mut socket, _addr) = listener.accept().await.unwrap();

    // let mut buffer = [0u8; 1024];

    // let bytes_read = socket.read(&mut buffer).await.unwrap();

    // socket.write_all(&buffer[..bytes_read]).await.unwrap();

    
}