use serde::Deserialize;
use serde_big_array::BigArray;
use crate::common::{
    errors::ServerError,
    enums::{DBOps, DBSave},
};
use tokio::sync::{
    mpsc::Sender,
    oneshot,
};
use p256::{
    ecdsa::{SigningKey, VerifyingKey},
};
use rand::{
    TryRng,
    rngs::SysRng
};

//DeviceStdComm struct (data received from device, second pass)
#[derive(Debug, Deserialize, Copy, Clone)]
pub struct DeviceStdComm {
    #[serde(rename = "device_id")]
    pub device_id: [u8; 6],
    #[serde(rename = "device_pub", with = "BigArray")]
    pub device_pub: [u8; 33],
    #[serde(rename = "nonce")]
    pub nonce: u32 
}

impl DeviceStdComm {
    pub fn new<T: AsRef<str>>(string: T) -> Result<Self, ServerError>{
        let res = serde_json::from_str(string.as_ref())?;
        Ok(res)
    }
}

//DeviceEnrl struct (data received from device, first pass)
#[derive(Debug, Deserialize, Copy, Clone)]
pub struct DeviceEnrl {
    #[serde(rename = "device_id")]
    pub device_id: [u8; 6],
    #[serde(rename = "device_pub", with = "BigArray")]
    pub device_pub: [u8; 33],
    #[serde(rename = "nonce")]
    pub nonce: u32 
}

impl DeviceEnrl {
    pub fn new<T: AsRef<str>>(string: T) -> Result<Self, ServerError>{
        let res = serde_json::from_str(string.as_ref())?;
        Ok(res)
    }
}

//Parse header byte
#[derive(Debug, Deserialize, Copy, Clone)]
pub struct Device {
    #[serde(rename = "header_byte")]
    pub header_byte: u8,
}

impl Device {
    pub fn new<T: AsRef<str>>(string: T) -> Result<Self, ServerError>{
        let res = serde_json::from_str(string.as_ref())?;
        Ok(res)
    }
}

//payload structs for threads to send to manage_db
pub struct CheckDevicePayload {
    pub device_id: [u8; 6],
    pub device_pub: [u8; 33],
}

pub struct SaveDevicePayload {
    pub device_id: [u8; 6],
    pub device_pub: [u8; 33],
    pub nonce: u32,
    pub save_op: DBSave,
}

//API for interacting with the Database task 
#[derive(Debug, Clone)]
pub struct DBClient {
    db_sender_handle: Sender<DBOps>
}

impl DBClient {
    pub fn new (db_sender_handle:Sender<DBOps>) -> Self {
        Self { db_sender_handle: db_sender_handle }
    }

    pub async fn check_dev_db(&mut self, device_id: &[u8; 6], device_pub: &[u8; 33]) -> Result<(), ServerError> {
        let (tx, rx) = oneshot::channel();
        let check_dev_payload = CheckDevicePayload { device_id: *device_id, device_pub: *device_pub };
        self.db_sender_handle.send(DBOps::CheckDevice(tx, check_dev_payload)).await?;
        rx.await?
    }

    pub async fn save_dev_db(&mut self, device_id: &[u8; 6], device_pub: &[u8; 33], nonce: &u32, save_op: DBSave) -> Result<(), ServerError> {
        let (tx, rx) = oneshot::channel();
        let save_dev_payload = SaveDevicePayload { device_id: *device_id, device_pub: *device_pub, nonce: *nonce, save_op: save_op};
        self.db_sender_handle.send(DBOps::SaveDevice(tx, save_dev_payload)).await?;
        rx.await?
    }
}

#[derive(Clone)]
pub struct CryptoClient {
    pub signing_key: SigningKey,
    pub verifying_key: VerifyingKey,
}

impl CryptoClient {
    pub fn new(signing_key: SigningKey, verifying_key: VerifyingKey) -> Self {
       Self { signing_key: signing_key, verifying_key: verifying_key } 
    }

    pub fn gen_server_challenge() -> Result<u32, ServerError>{
        Ok(SysRng.try_next_u32()?)
    }

    
}
