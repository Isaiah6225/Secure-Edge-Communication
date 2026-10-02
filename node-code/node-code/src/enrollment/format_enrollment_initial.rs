//Format mac address, verifying key, and device_nonce into a struct
//for initial request from node.
use crate::{
    common::{
        structs::{SendPacketInitialEnrl, SendConfirmationEnrl, SendPacketFinalVerification},
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
    return SendConfirmationEnrl { header_byte: header_byte, is_valid: is_valid }
}

pub fn format_enrollment_final_verification(header_byte: u8, device_signature: [u8; 64], server_challenge: u32, nonce: u32) -> SendPacketFinalVerification {
    let spf = SendPacketFinalVerification { header_byte: header_byte, device_signature: device_signature, server_challenge: server_challenge, nonce: nonce };
    info!("[format_enrollment] final send packet: {}", spf);
    return SendPacketFinalVerification { header_byte: header_byte, device_signature: device_signature, server_challenge: server_challenge, nonce: nonce }
}
