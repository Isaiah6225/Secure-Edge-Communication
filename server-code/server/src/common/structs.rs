pub struct EnrollmentReceiveInital {
    pub serialized_vkey: [u8; 33],
    pub dev_mac_add: [u8; 6], 
    pub device_nonce: u32,
}
