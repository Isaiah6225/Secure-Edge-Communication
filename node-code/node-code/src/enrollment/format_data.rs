//Format mac address, verifying key, and device_nonce into a struct
//for initial request from node.
use crate::{
    nonce::gen_nonce,
    common::{
        structs::SendPacketInitialEnrl,
        enums::WifiData,
    },
    boot::{
        gen_ecc,
        read_id,
    },
    common::enums::{
        EnrollmentSteps,
    }
};
use log::info;


pub fn format_enrollment(enrollment_steps: &EnrollmentSteps) -> WifiData {
    match enrollment_steps {
        EnrollmentSteps::Initial => {
            //get values for packet struct
            let mac = read_id::read_mac();
            let sv_key_bytes = gen_ecc::gen_key_pair();
            let nonce =  gen_nonce::gen_nonce();

            let spi = SendPacketInitialEnrl {dev_mac_add: mac, serialized_vkey:sv_key_bytes, device_nonce: nonce};
            info!("[format_enrollment] initial packet: {}", spi);

            return WifiData::SendEnrlInitial(SendPacketInitialEnrl { dev_mac_add: mac, serialized_vkey: sv_key_bytes, device_nonce: nonce})
        },

        EnrollmentSteps::FinalVerification => todo!(),
    }

}
