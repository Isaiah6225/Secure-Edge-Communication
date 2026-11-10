use serde::Deserialize;
use serde_big_array::BigArray;
use crate::{
    parse::parse_packet,
    common::{
        errors::ServerError,
        enums::{DBOps, DBSave, ParsedStruct},
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
    ecdsa::{signature::DigestSigner, SigningKey, VerifyingKey, RecoveryId, Signature, signature::Verifier},
    ecdh::EphemeralSecret,
    Sec1Point, PublicKey, 
    elliptic_curve::Generate
};
use rand::{
    TryRng,
    rngs::SysRng
};
use std::{
    io::{Write, ErrorKind},
};
use sha2::{Sha256, Digest};
use aes_gcm::{
    Tag, Aes128Gcm, KeyInit, AeadInOut, Nonce
};
use hkdf::GenericHkdf;
use hybrid_array::Array;
/*DEVICE INITIAL DATA STRUCTS*/
//DeviceStdComm struct (data received from device, second pass)
#[derive(Debug, Deserialize, Copy, Clone)]
pub struct DeviceStdComm {
    #[serde(rename = "device_id")]
    pub device_id: [u8; 6],
    #[serde(rename = "device_ecdh_pub", with = "BigArray")]
    pub device_ecdh_pub: [u8; 65],
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
#[derive(Debug, Deserialize, Copy, Clone)]
pub struct SecureKeySetup {
    #[serde(rename = "encrypted_buf")]
    pub encrypted_buf: [u8; 19],
    #[serde(rename = "a_tag")]
    pub a_tag: [u8; 16],
    #[serde(rename = "nonce")]
    pub nonce: [u8; 12]
}

impl SecureKeySetup {
    pub fn new<T: AsRef<str>>(string: T) -> Result<Self, ServerError>{
        let res = serde_json::from_str(string.as_ref())?;
        Ok(res)
    }
}

#[derive(Debug, Deserialize, Copy, Clone)]
pub struct FinalVeriCleanUp {
    #[serde(rename = "device_signature", with="BigArray")]
    pub device_signature: [u8; 64],
    #[serde(rename = "server_challenge")]
    pub server_challenge: u32,
    #[serde(rename = "nonce")]
    pub nonce: u32
}

impl FinalVeriCleanUp {
    pub fn new<T: AsRef<str>>(string: T) -> Result<Self, ServerError>{
        let res = serde_json::from_str(string.as_ref())?;
        Ok(res)
    }
}

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
}

pub struct SaveDevicePayload {
    pub device_id: [u8; 6],
    pub save_op: DBSave,
}

pub struct UpdateDeviceStatusPayload {
    pub device_id: [u8; 6], 
    pub device_pub: [u8; 33],
    pub save_op: DBSave, 
}

pub struct IsDeviceVerifiedPayload {
    pub device_id: [u8; 6],
}

pub struct DBEnrollmentStatus {
    pub save_op: String
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

    pub async fn check_dev_db(&mut self, device_id: &[u8; 6]) -> Result<(), ServerError> {
        let (tx, rx) = oneshot::channel();
        let check_dev_payload = CheckDevicePayload { device_id: *device_id };
        self.db_sender_handle.send(DBOps::CheckDevice(tx, check_dev_payload)).await?;
        match rx.await {
            Ok(res) => {
                println!("[db_client::check_dev_db] {:?}", res);
                res
            },
            Err(e)=> {
                println!("[db_client::check_dev_db] receive oneshot error: {:?}", e);
                return Err(ServerError::OneshotRecvErr(e))
            },
        }
    }

    pub async fn save_dev_db(&mut self, device_id: &[u8; 6], save_op: DBSave) -> Result<(), ServerError> {
        let (tx, rx) = oneshot::channel();
        let save_dev_payload = SaveDevicePayload { device_id: *device_id, save_op: save_op};
        self.db_sender_handle.send(DBOps::SaveDevice(tx, save_dev_payload)).await?;
        rx.await?
    }

    pub async fn is_dev_verified_db(&mut self, device_id: &[u8; 6]) -> Result<(), ServerError > {
        let (tx, rx) = oneshot::channel();
        let is_dev_verified_payload = IsDeviceVerifiedPayload { device_id: *device_id };
        self.db_sender_handle.send(DBOps::IsDeviceVerified(tx, is_dev_verified_payload)).await?;
        match rx.await {
            Ok(res) => {
                println!("[db_client::is_dev_verified] {:?}", res);
                res
            },
            Err(e) => {
                println!("[db_client::check_dev_db] receive oneshot error: {:?}", e);
                return Err(ServerError::OneshotRecvErr(e))
            },
        }
    }

    pub async fn update_dev_db(&mut self, device_id: &[u8; 6], device_pub: &[u8; 33], save_op: DBSave) -> Result<(), ServerError> {
        let (tx, rx) = oneshot::channel(); 
        let update_dev_status_payload = UpdateDeviceStatusPayload { device_id: *device_id, device_pub: *device_pub, save_op: save_op };
        self.db_sender_handle.send(DBOps::UpdateDeviceStatus(tx, update_dev_status_payload)).await?;
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

    //generate ecdh public key from ephemeral secret
    pub fn gen_ecdh_pub() -> Result<(EphemeralSecret, [u8; 65]), ServerError> {
        let mut ecdh_pub_output = [0u8; 65];
        let secret = EphemeralSecret::generate();
        let pub_key = Sec1Point::from(secret.public_key());

        let pub_key_bytes = pub_key.as_bytes();
        ecdh_pub_output.copy_from_slice(pub_key_bytes);
        Ok((secret, ecdh_pub_output))
    }

    //generate server challenge
    pub fn gen_server_challenge() -> Result<u32, ServerError> {
        Ok(SysRng.try_next_u32()?)
    }

    //generate signature base from given device pub, device id, nonce and server challenge
    pub fn gen_signature_base(device_pub: &[u8; 33], device_id: &[u8; 6], nonce: &u32, server_challenge: &u32) -> Result<Vec<u8>, ServerError> {
        let mut signature_base = Vec::new();
        write!(&mut signature_base, "{:?}{}{}{:?}", device_id, nonce, server_challenge, device_pub)?;
        Ok(signature_base)
    }

    //generate signature base from given server ecdsa pub, device ecdh pub, nonce and server ecdh pub 
    pub fn gen_sigunature_base_session(nonce: &u32, server_ecdh_pub: &[u8; 65], device_ecdh_pub: &[u8; 65], server_ecdsa_pub: &[u8; 65]) -> Result<Vec<u8>, ServerError> {
        let mut signature_base_session = Vec::new();
        write!(&mut signature_base_session, "{}{:?}{:?}", nonce, server_ecdh_pub, device_ecdh_pub)?;
        println!("[gen_signature_base_session] signature_base_session: {:?}", signature_base_session);
        Ok(signature_base_session)
    }
    
    //generate server signature 
    pub fn gen_signature(&self, signature_base: &Vec<u8>) -> Result<(Signature, RecoveryId), ServerError>{
        let (signature, recovery_id) = self.signing_key.sign_digest(|hash_handle: &mut Sha256| {hash_handle.update(&signature_base)});
        Ok((signature, recovery_id))
    }
    
    //generate public key as bytes
    pub fn gen_pub_key_bytes(&self) -> Result<[u8; 65], ServerError>{
        let mut vkey_output = [0u8; 65];
        let vkey_bytes = self.verifying_key.to_sec1_bytes();
        vkey_output.copy_from_slice(&vkey_bytes);
        Ok(vkey_output)
    }

    //verify device signature
    //TODO extract struct out of enum in a different method
    pub fn verify_ecdsa_signature(device_pub_bytes: &[u8; 33], device_signature_bytes: &[u8; 64], signature_base: u32) -> Result<(), ServerError> {
        let device_signature = Signature::from_slice(device_signature_bytes)?;
        let signature_base_bytes = signature_base.to_be_bytes();

        let device_pub = VerifyingKey::from_sec1_bytes(device_pub_bytes)?;
        device_pub.verify(&signature_base_bytes, &device_signature)?;
        Ok(())
    }

    pub fn gen_session_key(vkey_bytes: &[u8; 65], e_secret: EphemeralSecret, device_ecdh: &[u8; 65], device_id: &[u8; 6]) -> Result<Aes128Gcm, ServerError> {
        //get shared secret from device public key
        let device_ecdh_public = PublicKey::from_sec1_bytes(device_ecdh)?;
        let shared_secret = e_secret.diffie_hellman(&device_ecdh_public);
        
        let mut okm = [0u8; 16];
        let hk: GenericHkdf<_> = shared_secret.extract::<Sha256>(Some(device_id));
        hk.expand(device_id, &mut okm)?;
        
        let cipher = Aes128Gcm::new_from_slice(&okm)?;
        Ok(cipher)
    }

    pub fn decrypt_symmetric_final(cipher: &Aes128Gcm, a_tag_slice: &mut [u8], nonce_slice: &[u8; 12], encrypted_buf: &mut [u8], vkey_bytes: &[u8; 65]) -> Result<(), ServerError>{
        //let nonce = Nonce::try_from(nonce_slice)?;
        let nonce = Nonce::from_slice(nonce_slice);
        let a_tag = Tag::from_slice(a_tag_slice);
        cipher.decrypt_inout_detached(nonce, vkey_bytes, encrypted_buf.into(), &a_tag)?;
        Ok(())
    }

    pub fn encrypt_symmetric_final<'a>(cipher: &Aes128Gcm, vkey_bytes: &[u8; 65], data: &'a mut [u8]) -> Result<(Tag, [u8; 12], &'a mut [u8]), ServerError> {
        let server_nonce = Nonce::generate();
        let server_a_tag = cipher.encrypt_inout_detached(&server_nonce, vkey_bytes, data.into())?;

        let mut nonce_bytes = [0u8; 12];
        nonce_bytes.copy_from_slice(server_nonce.as_slice());
        Ok((server_a_tag, nonce_bytes, data))
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

    pub async fn read_data(&self) -> Result<ParsedStruct, ServerError>{
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
                            return Ok(data)
                        },
                        Err(e) => {                     
                            println!("[network_client] received error while trying to read");
                            match e.kind() {
                                ErrorKind::WouldBlock => {
                                    println!("[network_client] would have blocked");
                                    continue;
                                },
                                _ => { 
                                    println!("[network_client] error: {e:?}");
                                    continue;
                                }
                            }
                        },
                    }
                },
                Err(e) => { return Err(ServerError::IoErr(e)) }
            }
        }
    }
}

pub struct EnrollmentClient; 

impl EnrollmentClient {
    pub fn is_valid_enrollment(parsed_struct: ParsedStruct) -> Result<(), ServerError> {
        match parsed_struct {
            ParsedStruct::DeviceReceiveInitialEnrlParsed(data) => {
                println!("[enrollment_client] checking value data");
                if data.is_valid == 0 { 
                    println!("[enrollment_client] valid initial enrollment received from device");
                    return Ok(()) 
                } else { 
                    println!("[enrollment_client] invalid initial enrollment received from device");
                    return Err(ServerError::InvalidReceiveEnrollment)
                };
            }
            _=> { return Err(ServerError::InvalidStruct) }
        }
    }
}
