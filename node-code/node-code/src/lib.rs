#![no_std]

pub struct SendPacketInital  {
    pub serialized_vkey: f32,
    pub dev_mac_add: [u8; 6], 
    pub device_nonce: u64, 
}


pub mod boot;
pub mod enroll_device;

