//Format mac address, verifying key, and device_nonce into a struct
//for initial request from node.
use crate::{
    nonce::gen_nonce,
    common::{
        structs::SendPacketInitalEnrl,
        error::NodeError,
        enums::WifiCommand,
    },
    boot::{
        gen_ecc,
        read_id,
    },
    common::enums::{
        EnrollmentSteps,
    }
};
use esp_hal::rng::TrngSource;
use log::info;


pub fn format_enrollment(trng_source: &TrngSource<'static>, enrollment_steps: EnrollmentSteps) -> WifiCommand {
    match enrollment_steps {
        EnrollmentSteps::Initial => {
            //get values for packet struct
            let mac = read_id::read_mac();
            let sv_key_bytes = gen_ecc::gen_key_pair(&trng_source);
            let nonce =  gen_nonce::gen_nonce(&trng_source);

            let spi = SendPacketInitalEnrl {dev_mac_add: mac, serialized_vkey:sv_key_bytes, device_nonce: nonce};
            info!("{}", spi);

            return WifiCommand::SendEnrlInitial(SendPacketInitalEnrl { dev_mac_add: mac, serialized_vkey: sv_key_bytes, device_nonce: nonce})
        },

        EnrollmentSteps::FinalVerification => todo!(),
    }

}
