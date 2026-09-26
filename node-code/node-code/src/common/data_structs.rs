use serde::Deserialize;
use serde_big_array::BigArray;



// Data structs 
pub struct SendPacketFinalEnrl {
    pub header_byte: u8, 
    pub node_signature_bytes: [u8; 64],
    pub server_challenge: u32, 
}

// receive final verification
#[derive(Debug, Deserialize)]
pub struct ReceivePacketFinVeri {
    #[serde(rename = "signature_bytes", with = "BigArray")]
    pub signature_bytes: [u8; 64],
    #[serde(rename = "signature_base", with = "BigArray")]
    pub signature_base: [u8; 1500],
    #[serde(rename = "server_challenge")]
    pub server_challenge: u32,
}

impl ReceivePacketFinVeri{
    pub fn new<T: AsRef<str>>(string: T) -> Result<Self, NodeError>{
        let res = serde_json::from_str(string.as_ref())?;
        Ok(res)
    }
}

pub struct SendConfirmationEnrl {
    pub header_byte: u8, 
    pub is_valid: u8,
}

impl Display for SendConfirmationEnrl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "header_byte: {:?}, is_valid: {:?}", self.is_valid, self.header_byte)
    }
}

#[derive(Debug, Deserialize)]
pub struct ReceivePacketInitialEnrl {
    #[serde(rename = "server_pub_key", with = "BigArray")]
    pub server_pub_key: [u8; 65],
}

impl ReceivePacketInitialEnrl {
    pub fn new<T: AsRef<str>>(string: T) -> Result<Self, NodeError>{
        let res = serde_json::from_str(string.as_ref())?;
        Ok(res)
    }
}


//Enrollment Packets Struct 
#[derive(Debug)]
pub struct SendPacketInitialEnrl {
    pub header_byte: u8,
    pub serialized_vkey: [u8; 33],
    pub dev_mac_add: [u8; 6], 
    pub device_nonce: u32, 
}

impl Display for SendPacketInitialEnrl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "serialized_vkey: {:?}, dev_mac_add: {:?}, device_nonce: {}", self.serialized_vkey, self.dev_mac_add, self.device_nonce)
    }
}
