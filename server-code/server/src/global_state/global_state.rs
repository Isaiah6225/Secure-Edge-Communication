use crate::{
    common::{
        structs::{DeviceStdComm, DeviceEnrl, DBClient, CryptoClient, NetworkClient},
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
    io::{AsyncWriteExt, Interest},
};
use p256::{
    ecdsa::{VerifyingKey, SigningKey},
    pkcs8::{DecodePrivateKey, DecodePublicKey},
};
use heapless::String;
use std::fmt::Write;

pub async fn manage_enrollment(mut stream: TcpStream, data_parsed: DeviceEnrl, mut db_client: DBClient) -> Result<(), ServerError>{
    //set up crypto client
    println!("[manage_enrollment] setting up crypto client");
    let read_verifying_key = VerifyingKey::read_public_key_pem_file("./pub_key.pem")?;
    let read_signing_key = SigningKey::read_pkcs8_pem_file("./priv_key.pem")?;
    let crypto_client = CryptoClient::new(read_signing_key, read_verifying_key); 

    //set up network client
    println!("[manage_enrollment] setting up network client");
    let network_client = NetworkClient::new(&stream); 

    //complete enrollment checks
    println!("[manage_enrollment] starting enrollment checks");
    //enrollment_time::check_window()?;
    check_device_id::check_id(&data_parsed.device_id)?;
    db_client.check_dev_db(&data_parsed.device_id, &data_parsed.device_pub).await?;
    db_client.save_dev_db(&data_parsed.device_id, &data_parsed.device_pub, &data_parsed.nonce, DBSave::Pending).await?;
    
    //complete enrollment cryptography (server signature, signature base, and server_challenge)
    println!("[manage_enrollment] completing initial enrollment cryptography");
    let server_challenge = CryptoClient::gen_server_challenge()?;
    let signature_base = crypto_client.gen_signature_base(&data_parsed.device_id, &data_parsed.nonce, &server_challenge)?;
    let (signature, recovery_id) = crypto_client.gen_signature(&signature_base)?;
    let server_pub_key = crypto_client.gen_pub_key_bytes()?;
    let signature_bytes = &signature.to_vec();

    //write response to device
    let mut init_send_buffer = String::<2048>::new();
    if let Err(e) = write!(
        init_send_buffer,
        r#"{{"server_pub_key": {:?}}}"#,
        server_pub_key
    ){
        println!("[manage_enrollment] error from write {:?}", e);
    };
    println!("[manage_enrollment] init_send_buffer: {:?}", init_send_buffer);
    stream.try_write(init_send_buffer.as_bytes())?;

    //read initial response
    println!("[manage_enrollment] waiting for device response"); 
    let initial_response_data = network_client.read_data()?;
    println!("[manage_enrollment] received response with: {:?}", initial_response_data); 
    Ok(())
}

pub async fn manage_standard_communication(mut stream: TcpStream, data_parsed: DeviceStdComm, mut db_client: DBClient) -> Result<(), ServerError>{
    todo!(); 
}
