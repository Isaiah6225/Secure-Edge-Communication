//Read efuse mac 
use esp_hal::efuse::Efuse; 
use log::info;
//use esp_hal::rom::spiflash::esp_rom_spiflash_write;

pub fn read_mac() -> [u8; 6]{
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

    mac_address
}

