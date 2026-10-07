use crate::common::structs::{SendPacketSecSesInit};
use log::info;


pub fn format_secure_session_init(header_byte: u8, nonce: u32, ecdh_pub_key: &[u8; 65], device_id: [u8; 6]) -> SendPacketSecSesInit {
    let spssi = SendPacketSecSesInit { header_byte: header_byte, nonce: nonce, ecdh_pub_key: *ecdh_pub_key, device_id: device_id };
    info!("[format_enrollment] initial packet: {}", spssi);
    return SendPacketSecSesInit { header_byte: header_byte, nonce: nonce, ecdh_pub_key: *ecdh_pub_key, device_id: device_id }
}
