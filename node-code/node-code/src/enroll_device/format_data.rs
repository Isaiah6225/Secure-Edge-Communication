//Format mac address, verifying key, and device_nonce into a struct
use crate::boot::{
    gen_ecc,
    read_id,
};
use crate::SendPacketInital;
use esp_hal::rng::TrngSource;

pub async fn format_packet(trng_source: TrngSource<'static>) -> SendPacketInital {
    let mac = read_id::read_mac();
    let sv_key = gen_ecc::gen_key_pair(trng_source);

    Ok(SendPacketInital {
        dev_mac_add: mac,
        serialized_vkey: sv_key,
    })
}
