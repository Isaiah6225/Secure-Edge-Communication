//Read efuse mac and set constants flash
use esp_hal::efuse::Efuse; 
use log::info;
//use esp_hal::rom::spiflash::esp_rom_spiflash_write;

#[embassy_executor::task]
pub async fn read_mac (){
    let mac_address = Efuse::read_base_mac_address();
    info!(
        "ESP MAC from efuse: {:#X}:{:#X}:{:#X}:{:#X}:{:#X}:{:#X}",
        mac_address[0],
        mac_address[1],
        mac_address[2],
        mac_address[3],
        mac_address[4],
        mac_address[5]
    );
}

