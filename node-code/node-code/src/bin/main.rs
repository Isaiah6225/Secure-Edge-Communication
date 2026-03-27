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
use crate::boot::{
    read_id::read_mac,
    gen_ecc::priv_key,
};
use log::info;

pub mod boot;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

extern crate alloc;
esp_bootloader_esp_idf::esp_app_desc!();

#[esp_rtos::main]
async fn main(spawner: Spawner) {

    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 73744);
    // COEX needs more RAM - so we've added some more
    esp_alloc::heap_allocator!(size: 64 * 1024);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_rtos::start(timg0.timer0);

    info!("Embassy initialized!");

    //set up TrngSource
    let trng_source = TrngSource::new(peripherals.RNG, peripherals.ADC1);

    // TODO: Spawn some tasks
    spawner.spawn(read_mac()).ok();
    spawner.spawn(priv_key(trng_source)).ok();
}
