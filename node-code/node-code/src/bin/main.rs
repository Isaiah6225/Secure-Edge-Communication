#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget
)]

use embassy_executor::Spawner;
use esp_hal::{
    clock::CpuClock,
    timer::timg::TimerGroup,
    rng::TrngSource,
};
use node_code::{
    enrollment::enrollment,
    boot::create_nvs_handle,
    global_state::global_state,
    common::structs::StorageManager,
};
use esp_storage::FlashStorage;
use log::info;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

extern crate alloc;
esp_bootloader_esp_idf::esp_app_desc!();

#[esp_rtos::main]
async fn main(spawner: Spawner) {
    //set logger, config, and peripherals 
    esp_println::logger::init_logger_from_env();
    
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);
    
    //RAM for wifi TODO set up wifi 
    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 73744);
    esp_alloc::heap_allocator!(size: 64 * 1024);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_rtos::start(timg0.timer0);
    info!("Embassy initialized!");

    //set up TrngSource
    let trng_source = TrngSource::new(peripherals.RNG, peripherals.ADC1);

    //set up NVS partition and handle
    let storage = FlashStorage::new(peripherals.FLASH); 
    let nvs = create_nvs_handle::set_nvs_handle(storage).expect("NVS failed setup. Panicking as program requires NVS to be set.");
    let storage_manager = StorageManager::new(nvs);

    //spawner.spawn(enrollment::enroll(trng_source)).ok();
    spawner.spawn(global_state::manage_global_state(storage_manager)).ok();
}
