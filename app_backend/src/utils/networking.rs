#[allow(unused_must_use)]
#[allow(dead_code)]
#[allow(unused_variables)]
pub mod config;

use bytes::{BytesMut, BufMut};

pub async fn _send_message(addr: String, port:i8, message:BytesMut) -> bool {
    return true;
}