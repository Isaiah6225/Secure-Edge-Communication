use serde::Deserialize;
use serde_big_array::BigArray;
use crate::common::errors::ServerError;

#[derive(Debug, Deserialize, Copy, Clone)]
pub struct Device {
    #[serde(rename = "device_id")]
    pub device_id: [u8; 6],
    #[serde(rename = "device_pub", with = "BigArray")]
    pub device_pub: [u8; 33],
    #[serde(rename = "nonce")]
    pub nonce: u32 
}

impl Device {
    pub fn new<T: AsRef<str>>(string: T) -> Result<Self, ServerError>{
        let res = serde_json::from_str(string.as_ref())?;
        Ok(res)
    }
}
