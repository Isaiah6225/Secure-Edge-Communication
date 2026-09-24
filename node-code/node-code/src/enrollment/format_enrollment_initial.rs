//Format mac address, verifying key, and device_nonce into a struct
//for initial request from node.
use crate::{
    common::{
        structs::{SendPacketInitialEnrl, SendConfirmationEnrl},
    },
};
use log::info;


pub fn format_enrollment_initial(header_byte: u8, mac: [u8; 6], sv_key_bytes: [u8; 33], nonce: u32) -> SendPacketInitialEnrl {
    let spi = SendPacketInitialEnrl {dev_mac_add: mac, serialized_vkey:sv_key_bytes, device_nonce: nonce, header_byte: header_byte};
    info!("[format_enrollment] initial packet: {}", spi);
    return SendPacketInitialEnrl { dev_mac_add: mac, serialized_vkey: sv_key_bytes, device_nonce: nonce, header_byte: header_byte }
}

pub fn format_enrollment_initial_confirmation(is_valid: u8, header_byte: u8) -> SendConfirmationEnrl{
    let sce = SendConfirmationEnrl { header_byte: header_byte, is_valid: is_valid };
    info!("[format_enrollment] confirmation initial packet: {}", sce);
    return SendConfirmationEnrl { header_byte: header_byte, is_valid: is_valid };
}
