#[allow(unused_must_use)]
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_imports)]

mod utils;
mod modules;
use utils::datastorage::BackendData;
use utils::networking::Networking;
use uuid::Uuid;


async fn gui_setup(networking: &mut Networking, backend_data: &BackendData) -> Uuid {
    let listener_uuid = networking.create_listener(
        backend_data.config.backend_address.clone(),
        backend_data.config.backend_port,
    );

    listener_uuid
}


#[tokio::main]
async fn main() {
    let mut backend_data = BackendData::new();
    let mut networking = Networking::new();

    let listener_uuid = gui_setup(&mut networking, &backend_data).await;

    // @TODO: Add networking support

    // let listener =_backend_data.networking_data.active_listeners.add_listener("localhost".to_string(), 3365);

    // println!("{}", listener.as_ref().unwrap().listener_uuid.unwrap().as_urn())
    // let listener = TcpListener::bind("localhost:3365").await.unwrap();

    // let (mut socket, _addr) = listener.accept().await.unwrap();

    // let mut buffer = [0u8; 1024];

    // let bytes_read = socket.read(&mut buffer).await.unwrap();

    // socket.write_all(&buffer[..bytes_read]).await.unwrap();
}
