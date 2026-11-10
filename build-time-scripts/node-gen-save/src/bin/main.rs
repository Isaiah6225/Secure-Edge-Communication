#![no_std]
#![no_main]
use esp_hal::clock::CpuClock;
use esp_hal::{main, rng::Trng, efuse::{self, InterfaceMacAddress}};
use esp_hal::time::{Duration, Instant};
use log::{error, info};
use esp_storage::FlashStorage;
use esp_nvs::{Nvs, Key};
use alloc::string::String;
use p256::{
    ecdsa::{VerifyingKey, SigningKey},
    pkcs8::DecodePublicKey,
};

#[panic_handler]
fn panic(panic_info: &core::panic::PanicInfo) -> ! {
    error!("{}", panic_info);
    loop {}
}

extern crate alloc;
esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]

#[main]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);
    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 73744);

    //parition and nvs init   
    let partition_offset = 0x390000;
    let partition_size = 0x32000;

    let storage = FlashStorage::new(peripherals.FLASH); 
    let mut nvs = Nvs::new(partition_offset, partition_size, storage).expect("failed to create nvs");

    let mut vkey_output = [0u8; 65];
    let mut skey_output = [0u8; 32];

    let node_signing_key = "fill_from_set_keys_output";
    let node_verifying_key = "fill_from_set_keys_output";

    vkey_output.copy_from_slice(&node_verifying_key);
    skey_output.copy_from_slice(&node_signing_key);

    //save vkey to env file and key pair to nvs
    let namespace = const {Key::from_str("ecdsa_keys")};
    
    let priv_key = const {Key::from_str("priv")};
    let pub_key = const {Key::from_str("pub")};
    
    let priv_key_value: &[u8] = &skey_output;
    let pub_key_value: &[u8] = &vkey_output;

    nvs.set(&namespace, &priv_key, priv_key_value).expect("failed to set private key");
    nvs.set(&namespace, &pub_key, pub_key_value).expect("failed to set public key");

    let mut mac_output= [0u8; 6];
    let mac_address = efuse::interface_mac_address(InterfaceMacAddress::Station);
    mac_output.copy_from_slice(&mac_address.as_bytes());
    info!("mac address: {:?}", mac_output);


    loop {
        info!("data set in nvs close program");
    }
}
