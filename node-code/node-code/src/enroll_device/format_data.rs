//Format mac address, verifying key, and device_nonce into a struct
//for initial request from node.
use crate::{
    SendPacketInital,
    NodeError,
    nonce::gen_nonce,
    boot::{
        gen_ecc,
        read_id,
    },
};
use esp_hal::rng::TrngSource;
use log::info;


pub fn format_packet(trng_source: TrngSource<'static>) -> Result<SendPacketInital, NodeError> {
    //get values for packet struct
    let mac = read_id::read_mac();
    let sv_key_bytes = gen_ecc::gen_key_pair(&trng_source)?;
    let nonce =  gen_nonce::gen_nonce(&trng_source)?;

    let spi = SendPacketInital {dev_mac_add: mac, serialized_vkey:sv_key_bytes, device_nonce: nonce};
    info!("{}", spi);

    Ok(SendPacketInital {
        dev_mac_add: mac,
        serialized_vkey: sv_key_bytes,
        device_nonce: nonce,
    })
}
