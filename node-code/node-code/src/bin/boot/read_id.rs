//Read efuse mac and set constants flash
use esp_hal::efuse::Efuse; 
use esp_hal::rom::spiflash::esp_rom_spi_flash_write

let mac_add = Efuse::read_base_mac_address();


