//extracts data from serde_json Values and returns the actual values
/*
use serde_json::{Value, from_value};
use crate::common::{
    errors::ServerError,
    enums::Parse,
    structs::Device
};
pub fn extract_data(data_parsed: Value) -> Result<Device, ServerError> {
    //let (device_id, device_pub, device_nonce) = (data_parsed["device_id"], data_parsed["device_pub"], data_parsed["nonce"]);

    let con_mac: [u8; 6] = from_value(data_parsed["device_id"])?;
    let pub_key: Vec<u8> = from_value(data_parsed["device_pub"])?;
    let nonce: u32 = from_value(data_parsed["nonce"])?;

    let res_pub_key: [u8; 33] = pub_key.try_into().map_err(|v: Vec<u8>| ServerError::InvalidKeyLength(v.len()))?;
    
    Ok(Device { device_id: con_mac, device_pub: res_pub_key, nonce: nonce })
}
*/
