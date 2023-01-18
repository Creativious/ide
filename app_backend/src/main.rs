use std::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:3365");
}