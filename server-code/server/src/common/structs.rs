use serde::Deserialize;
use serde_big_array::BigArray;
use crate::{
    parse::parse_packet,
    common::{
        errors::ServerError,
        enums::{DBOps, DBSave},
    }
};
use tokio::{
    io::Interest,
    net::TcpStream,
    sync::{
        mpsc::Sender,
        oneshot,
    },
};
use p256::{
   ecdsa::{signature::DigestSigner, SigningKey, VerifyingKey, RecoveryId, Signature},
};
use rand::{
    TryRng,
    rngs::SysRng
};
use std::{
    io::{Write, ErrorKind},
};
use sha2::{Sha256, Digest};
/*DEVICE INITIAL DATA STRUCTS*/
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
pub struct HeaderByte {
    #[serde(rename = "header_byte")]
    pub header_byte: u8,
}

impl HeaderByte {
    pub fn new<T: AsRef<str>>(string: T) -> Result<Self, ServerError>{
        let res = serde_json::from_str(string.as_ref())?;
        Ok(res)
    }
}
//END

/*
DEVICE AND SERVER GLOBAL STRUCTS
*/
//Device confirmation in enrollment initial 
#[derive(Debug, Deserialize, Copy, Clone)]
pub struct IsValid {
    #[serde(rename = "is_valid")]
    pub is_valid: u8,
}

impl IsValid {
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
//END

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
        match rx.await {
            Ok(res) => {
                println!("[db_client::check_dev_db] {:?}", res);
                res
            },
            Err(e)=> {
                println!("not cool man");
                return Err(ServerError::OneshotRecvErr(e))
            },
        }
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
    
    pub fn gen_signature(&self, signature_base: &Vec<u8>) -> Result<(Signature, RecoveryId), ServerError>{
        let (signature, recovery_id) = self.signing_key.sign_digest(|hash_handle: &mut Sha256| {hash_handle.update(&signature_base)});
        Ok((signature, recovery_id))
    }

    pub fn gen_pub_key_bytes(&self) -> Result<[u8; 65], ServerError>{
        let mut vkey_output = [0u8; 65];
        let vkey_bytes = self.verifying_key.to_sec1_bytes();
        vkey_output.copy_from_slice(&vkey_bytes);
        Ok(vkey_output)
    }
}

//API for handling networking in global state
#[derive(Clone)]
pub struct NetworkClient<'a>{
    pub stream: &'a TcpStream, 
}

impl<'a> NetworkClient<'a> {
    pub fn new(stream: &'a TcpStream) -> Self {
        Self { stream: stream }
    }

    pub async fn read_data(&self) -> Result<usize, ServerError>{
        let mut response_buf = [0u8; 512];
        println!("[network_client] awaiting until stream is readable");
        loop {
            match self.stream.ready(Interest::READABLE).await {
                Ok(_) => {
                    println!("[network_client] attempting to read device");
                    match self.stream.try_read(&mut response_buf) {
                        Ok(0) => { 
                            println!("[network_client] empty response from device");
                            return Err(ServerError::EmptyReceiveErr) 
                        },
                        Ok(n) => { 
                            println!("[network_client] received data from device: {:?}", n);
                            let parse_string = str::from_utf8(&response_buf[..n])?;
                            let data = parse_packet::parse(parse_string)?;
                            println!("[network_client] parsed string: {:?}", data);
                            return Ok(n)
                        },
                        Err(e) => {                     
                            println!("[network_client] received error while trying to read");
                            match e.kind() {
                                ErrorKind::WouldBlock => {
                                    println!("[network_client] would have blocked");
                                    continue;
                                },
                                _ => return Err(ServerError::IoErr(e)) 
                            }
                        },
                    }
                },
                Err(e) => { return Err(ServerError::IoErr(e)) }
            }
        }
    }
}
