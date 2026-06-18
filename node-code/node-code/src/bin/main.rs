#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget
)]
use core::{net::Ipv4Addr, str::FromStr};
use embassy_executor::Spawner;
use embassy_sync::{
    channel::Channel,
    watch::Watch,
    blocking_mutex::raw::CriticalSectionRawMutex,
};
use embassy_net::{
    StackResources,
};
use esp_hal::{
    clock::CpuClock,
    timer::timg::TimerGroup,
    rng::TrngSource,
    rng::Rng,
};
use esp_radio::{
    Controller
};
use esp_storage::FlashStorage;
use node_code::{
    mk_static,
    boot::create_nvs_handle,
    global_state::global_state,
    common::{
        structs::{StorageManager, WifiManager, GSCManager},
        enums::{EnrollmentSteps, WifiConfigStatus, WifiCommand},
        structs
    },
    wifi_task::{wifi_task, wifi_config},
};
use log::info;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

esp_bootloader_esp_idf::esp_app_desc!();
const WIFI_PASSWORD: &'static str = env!("WIFI_PASSWORD");
const REMOTE_IP: &'static str = env!("REMOTE_IP");

static GSC: Channel<CriticalSectionRawMutex, EnrollmentSteps, 8> = Channel::new();
static WTC: Channel<CriticalSectionRawMutex, WifiCommand, 8> = Channel::new();
static WC: Watch<CriticalSectionRawMutex, WifiConfigStatus, 1> = Watch::new();


#[esp_rtos::main]
async fn main(spawner: Spawner) {
    //set logger, config, and peripherals 
    esp_println::logger::init_logger_from_env();
    
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);
    
    //RAM for wifi 
    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 73744);
    esp_alloc::heap_allocator!(size: 64 * 1024);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_rtos::start(timg0.timer0);
    info!("Embassy initialized!");
    
    //set up wifi resources
    //let radio_init = esp_radio::init().expect("Failed to initialize Wi-Fi controller");
    let radio_init = &*mk_static!(Controller<'static>, esp_radio::init().unwrap());
    let (wifi_controller, interfaces) =
        esp_radio::wifi::new(&radio_init, peripherals.WIFI, Default::default()).unwrap();

    //set wifi int
    let wifi_int = interfaces.sta;

    //set seed to prevent port collisions
    let rng = Rng::new(); 
    let seed = (rng.random() as u64) << 32 | rng.random() as u64;

    //config
    //let pass: &'static str = env!("wifi_password");
    let config = embassy_net::Config::dhcpv4(Default::default()); 

    let (stack, runner) = embassy_net::new(
        wifi_int, 
        config,
        mk_static!(StackResources<3>, StackResources::<3>::new()),
        seed,
    );

    //ip parsing 
    let ip_address = Ipv4Addr::from_str(REMOTE_IP).expect("failed to parse gateway IP");

    //set up TrngSource
    let trng_source = TrngSource::new(peripherals.RNG, peripherals.ADC1);

    //set wifi manager 
    let wifi_manager = WifiManager::new(stack, trng_source);
    let gsc_manager = GSCManager::new(GSC.sender(), WTC.receiver());

    //set up NVS partition and handle
    let storage = FlashStorage::new(peripherals.FLASH); 
    let nvs = create_nvs_handle::set_nvs_handle(storage).expect("NVS failed setup. Panicking as program requires NVS to be set.");
    let storage_manager = StorageManager::new(nvs);
    
    //wifi config watch messaging channel
    let rcv0 = WC.receiver().unwrap();
    let sen0 = WC.sender();

    spawner.spawn(wifi_config(WIFI_PASSWORD, wifi_controller, sen0)).ok();
    spawner.spawn(wifi_task::wifi_task(wifi_manager, GSC.receiver(), WTC.sender(), rcv0, ip_address)).ok();
    spawner.spawn(structs::net_task(runner)).ok();
    spawner.spawn(global_state::manage_global_state(storage_manager, gsc_manager)).ok();
}
