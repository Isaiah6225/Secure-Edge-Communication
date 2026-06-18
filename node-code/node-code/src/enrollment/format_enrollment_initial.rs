//Format mac address, verifying key, and device_nonce into a struct
//for initial request from node.
use crate::{
    common::{
        structs::SendPacketInitialEnrl,
        enums::WifiData,
    },
};
use log::info;


pub fn format_enrollment_initial(mac: [u8; 6], sv_key_bytes: [u8; 33], nonce: u32) -> WifiData {
    let spi = SendPacketInitialEnrl {dev_mac_add: mac, serialized_vkey:sv_key_bytes, device_nonce: nonce};
    info!("[format_enrollment] initial packet: {}", spi);
    return WifiData::SendEnrlInitial(SendPacketInitialEnrl { dev_mac_add: mac, serialized_vkey: sv_key_bytes, device_nonce: nonce})
}
