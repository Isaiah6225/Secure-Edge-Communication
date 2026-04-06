//Read efuse mac 
use esp_hal::efuse::Efuse; 
use log::info;
//use esp_hal::rom::spiflash::esp_rom_spiflash_write;

pub fn read_mac() -> [u8; 6]{
    let mac_address = Efuse::read_base_mac_address();
    mac_address
}

