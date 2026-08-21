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
    ecdsa::{signature::DigestSigner, SigningKey, VerifyingKey, RecoveryId, Signature},
};
use rand::{
    TryRng,
    rngs::SysRng
};
use std::io::Write;
use sha2::{Sha256, Digest};

//Device struct (data received from device, first pass)
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

//API for handling cryptography 
#[derive(Clone)]
pub struct CryptoClient {
    pub signing_key: SigningKey,
    pub verifying_key: VerifyingKey,
}

impl CryptoClient {
    pub fn new(signing_key: SigningKey, verifying_key: VerifyingKey) -> Self {
       Self { signing_key: signing_key, verifying_key: verifying_key } 
    }

    pub fn gen_server_challenge() -> Result<u32, ServerError> {
        Ok(SysRng.try_next_u32()?)
    }

    pub fn gen_signature_base(&self, device_id: &[u8; 6], nonce: &u32, server_challenge: &u32) -> Result<Vec<u8>, ServerError> {
        let mut signature_base = Vec::new();
        write!(&mut signature_base, "{:?}{}{}{:?}", device_id, nonce, server_challenge, self.verifying_key)?;
        Ok(signature_base)
    }
    
    pub fn gen_signature(&self, signature_base: Vec<u8>) -> Result<(Signature, RecoveryId), ServerError>{
        let (signature, recovery_id) = self.signing_key.sign_digest(|hash_handle: &mut Sha256| {hash_handle.update(&signature_base)});
        Ok((signature, recovery_id))
    }
}
