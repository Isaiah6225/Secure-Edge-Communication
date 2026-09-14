//Read efuse mac 
use esp_hal::efuse::{self, InterfaceMacAddress}; 

pub fn read_mac() -> [u8; 6]{
    let mut mac_output= [0u8; 6];
    let mac_address = efuse::interface_mac_address(InterfaceMacAddress::Station);
    mac_output.copy_from_slice(&mac_address.as_bytes());
    mac_output
}
