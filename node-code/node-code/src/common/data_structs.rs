use serde::Deserialize;
use serde_big_array::BigArray;
use core::{
    fmt, 
    fmt::Display
};
use alloc::{
    vec::Vec,
};
use crate::common::error::NodeError;

extern crate alloc; 
/* 
DATA STRUCTS SECURE SESSION
*/
#[derive(Debug)]
pub struct SendPacketSecureKS<'a> {
    pub header_byte: u8, 
    pub nonce: [u8; 12], 
    pub a_tag: &'a [u8], 
    pub encrypted_buf: &'a [u8],
}

impl<'a> Display for SendPacketSecureKS<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "header_byte: {:?}, nonce: {:?}, a_tag: {:?}, encrypted_buf: {:?}", self.header_byte, self.nonce, self.a_tag, self.encrypted_buf)
    }
}

#[derive(Debug, Deserialize)]
pub struct ReceivePacketSecSesInit {
    #[serde(rename = "signature_base")]
    pub signature_base: Vec<u8>,
    #[serde(rename = "signature_bytes", with="BigArray")]
    pub signature_bytes: [u8; 64],
    #[serde(rename = "server_ecdh", with="BigArray")]
    pub server_ecdh_pub: [u8; 65],
}

impl ReceivePacketSecSesInit {
    pub fn new<T: AsRef<str>>(string: T) -> Result<Self, NodeError>{
        let res = serde_json::from_str(string.as_ref())?;
        Ok(res)
    }
}

#[derive(Debug)]
pub struct SendPacketSecSesInit {
    pub header_byte: u8,
    pub nonce: u32,
    pub ecdh_pub_key: [u8; 65], 
    pub device_id: [u8; 6]
}

impl Display for SendPacketSecSesInit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "header_byte: {:?}, nonce: {:?}, ecdh_pub_key:{:?}, device_id: {:?}", self.header_byte, self.nonce, self.ecdh_pub_key, self.device_id)
    }
}

/* 
 DATA STRUCTS ENROLLMENT
 */

#[derive(Debug, Deserialize)]
pub struct ReceivePacketFinVeriClean {
    #[serde(rename = "signature_bytes", with = "BigArray")]
    pub signature_bytes: [u8; 64],
    #[serde(rename = "signature_base")]
    pub signature_base: Vec<u8>,
    #[serde(rename = "is_verify")]
    pub is_verify: u8,
}

impl ReceivePacketFinVeriClean {
    pub fn new<T: AsRef<str>>(string: T) -> Result<Self, NodeError>{
        let res = serde_json::from_str(string.as_ref())?;
        Ok(res)
    }
}

pub struct SendPacketFinalVerification {
    pub header_byte: u8, 
    pub device_signature: [u8; 64], 
    pub server_challenge: u32,
    pub nonce: u32, 
}

impl Display for SendPacketFinalVerification {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "header_byte: {:?}, device_signature: {:?}, server_challenge:{:?}, nonce: {:?}", self.header_byte, self.device_signature, self.server_challenge, self.nonce)
    }
}

#[derive(Debug, Deserialize)]
pub struct ReceivePacketFinVeri {
    #[serde(rename = "signature", with = "BigArray")]
    pub signature_bytes: [u8; 64],
    #[serde(rename = "signature_base")]
    pub signature_base: Vec<u8>,
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

#[derive(Debug, Deserialize, PartialEq)]
pub struct ReceivePacketInitialEnrl {
    #[serde(rename = "device_nonce")]
    pub device_nonce: u32,
    #[serde(rename = "signature", with = "BigArray")]
    pub signature_bytes: [u8; 64],
    #[serde(rename = "signature_base", with = "BigArray")]
    pub signature_base: [u8; 72],
    #[serde(rename = "server_challenge")]
    pub server_challenge: u32,
}

impl ReceivePacketInitialEnrl {
    pub fn new<T: AsRef<str>>(string: T) -> Result<Self, NodeError>{
        let res = serde_json::from_str(string.as_ref())?;
        Ok(res)
    }
}


//Enrollment Packets Struct 
#[derive(Debug, PartialEq)]
pub struct SendPacketInitialEnrl {
    pub header_byte: u8,
    pub dev_mac_add: [u8; 6], 
    pub dev_nonce: u32, 
}

impl Display for SendPacketInitialEnrl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "header_byte: {:?}, dev_mac_add: {:?}, device_nonce: {}", self.header_byte, self.dev_mac_add, self.dev_nonce)
    }
}
