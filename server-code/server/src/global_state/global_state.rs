use crate::{
    common::{
        structs::{DeviceStdComm, DeviceEnrl, DBClient, CryptoClient},
        enums::DBSave, 
        errors::ServerError
    },
    enrollment_checks::{
        enrollment_time,
        check_device_id
    },
};
use tokio::{
    net::TcpStream,
};
use p256::{
    ecdsa::{VerifyingKey, SigningKey},
    pkcs8::{DecodePrivateKey, DecodePublicKey},
};

pub async fn manage_enrollment(mut stream: TcpStream, data_parsed: DeviceEnrl, mut db_client: DBClient) -> Result<(), ServerError>{


pub async fn manage_enrollment(stream: TcpStream, data_parsed: Device, mut db_client: DBClient) -> Result<(), ServerError>{
    //set up crypto client
    let read_verifying_key = VerifyingKey::read_public_key_pem_file("../pub_key.pem")?;
    let read_signing_key = SigningKey::read_pkcs8_pem_file("../priv_key.pem")?;
    let crypto_client = CryptoClient::new(read_signing_key, read_verifying_key); 

    //complete enrollment checks
    enrollment_time::check_window()?;
    check_device_id::check_id(&data_parsed.device_id)?;
    db_client.check_dev_db(&data_parsed.device_id, &data_parsed.device_pub).await?;
    db_client.save_dev_db(&data_parsed.device_id, &data_parsed.device_pub, &data_parsed.nonce, DBSave::Pending).await?;
    

    Ok(())
}

pub async fn manage_standard_communication(mut stream: TcpStream, data_parsed: DeviceStdComm, mut db_client: DBClient) -> Result<(), ServerError>{
    todo!(); 
}
