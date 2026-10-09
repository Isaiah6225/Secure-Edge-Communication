use crate::{
    parse::parse_packet,
    common::{
        structs::{DeviceStdComm, DeviceEnrl, DBClient, CryptoClient, NetworkClient, EnrollmentClient},
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
use heapless::String;
use std::fmt::Write;

pub async fn manage_enrollment(stream: TcpStream, data_parsed: DeviceEnrl, mut db_client: DBClient) -> Result<(), ServerError>{
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
    
    //complete enrollment cryptography (server_challenge)
    println!("[manage_enrollment] completing initial enrollment cryptography");

    //write initial response to device
    let mut init_send_buffer = String::<2048>::new();
    if let Err(e) = write!(
        init_send_buffer,
        r#"{{"device_nonce": {:?}}}"#,
        &data_parsed.nonce
    ){
        println!("[manage_enrollment] error from write {:?}", e);
    };
    println!("[manage_enrollment] init_send_buffer: {:?}", init_send_buffer);
    stream.try_write(init_send_buffer.as_bytes())?;
    init_send_buffer.clear();

    //read initial response
    println!("[manage_enrollment] waiting for device response"); 
    let initial_response_data = network_client.read_data().await?;
    println!("[manage_enrollment] received response with: {:?}", initial_response_data); 
    EnrollmentClient::is_valid_enrollment(initial_response_data)?;

    //write final veri to device
    let server_pub_key = crypto_client.gen_pub_key_bytes()?;
    let server_challenge = CryptoClient::gen_server_challenge()?;
    let signature_base = CryptoClient::gen_signature_base(&data_parsed.device_pub, &data_parsed.device_id, &data_parsed.nonce, &server_challenge)?;
    let (signature, recovery_id) = crypto_client.gen_signature(&signature_base)?;
    let signature_bytes = &signature.to_vec();
    if let Err(e) = write!(
        init_send_buffer, 
        r#"{{"signature_bytes": {:?}, "signature_base": {:?}, "server_challenge": {:?}}}"#,
        signature_bytes, signature_base, server_challenge
    ){ 
        println!("[manage_enrollment] error from write {:?}", e); 
    };
    println!("[manage_enrollment] len of write: {:?}", init_send_buffer.len());
    stream.try_write(init_send_buffer.as_bytes())?;
    init_send_buffer.clear();

    //read final veri from device 
    println!("[manage_enrollment] waiting for device response"); 
    let finalveri_response_data = network_client.read_data().await?;
    println!("[manage_enrollment] received response with: {:?}", finalveri_response_data);

    println!("[manage_enrollment] parsing packet and checking device signature");
    let finalveri_parsed = parse_packet::parse_final_verification_parsed(finalveri_response_data)?;
    CryptoClient::verify_ecdsa_signature(&data_parsed.device_pub, &finalveri_parsed.device_signature, finalveri_parsed.server_challenge)?;
    
    //write final veri clean up to device 
    let is_verify: u8 = 0; 
    let server_challenge_final = CryptoClient::gen_server_challenge()?;
    let signature_base_final = CryptoClient::gen_signature_base(&data_parsed.device_pub, &data_parsed.device_id, &finalveri_parsed.nonce, &server_challenge_final)?;
    let (signature_final,  _) = crypto_client.gen_signature(&signature_base_final)?;
    let signature_bytes_final = &signature_final.to_vec();

    //update device record
    db_client.update_dev_db(&data_parsed.device_id, &data_parsed.device_pub, DBSave::Verified).await?;

    if let Err(e) = write!(
        init_send_buffer, 
        r#"{{"signature_bytes": {:?}, "signature_base": {:?}, "is_verify": {:?}}}"#,
        signature_bytes_final, signature_base_final, is_verify
    ){ 
        println!("[manage_enrollment] error from write {:?}", e); 
    };
    println!("[manage_enrollment] final verification clean up write length: {:?}", init_send_buffer.len());
    stream.try_write(init_send_buffer.as_bytes())?;
    init_send_buffer.clear();
    Ok(())
}

pub async fn manage_standard_communication(mut stream: TcpStream, data_parsed: DeviceStdComm, mut db_client: DBClient) -> Result<(), ServerError> {
    //set up crypto client
    println!("[manage_enrollment] setting up crypto client");
    let read_verifying_key = VerifyingKey::read_public_key_pem_file("./pub_key.pem")?;
    let read_signing_key = SigningKey::read_pkcs8_pem_file("./priv_key.pem")?;
    let crypto_client = CryptoClient::new(read_signing_key, read_verifying_key);

    //set up network client
    println!("[manage_enrollment] setting up network client");
    let network_client = NetworkClient::new(&stream); 

    //check if device is verified
    check_device_id::check_id(&data_parsed.device_id)?;
    db_client.is_dev_verified_db(&data_parsed.device_id).await?;

    //write init secure sesson 
    let server_pub_key_bytes = crypto_client.gen_pub_key_bytes()?;
    let server_ecdh_pub = CryptoClient::gen_ecdh_pub()?;
    let signature_base_session = CryptoClient::gen_sigunature_base_session(&data_parsed.nonce, &server_ecdh_pub, &data_parsed.device_pub, &server_pub_key_bytes)?;

    let signature_session = crypto_client.gen_signature(&signature_base_session)?;
    let signature_bytes_session = &signature_base_session.to_vec();

    let mut init_send_buffer = String::<2048>::new();
    if let Err(e) = write!(
        init_send_buffer, 
        r#"{{"signature_bytes": {:?}, "signature_base": {:?}, "nonce": {:?}, "server_ecdh_pub": {:?}}}"#,
        signature_bytes_session, signature_base_session, data_parsed.nonce, server_ecdh_pub
    ){ 
        println!("[manage_enrollment] error from write {:?}", e); 
    };
    println!("[manage_enrollment] initial secure session message length: {:?}", init_send_buffer.len());
    stream.try_write(init_send_buffer.as_bytes())?;
    init_send_buffer.clear();

    //read final veri from device 
    println!("[manage_enrollment] waiting for device response"); 
    let securekeyset_response = network_client.read_data().await?;
    println!("[manage_enrollment] received response with: {:?}", securekeyset_response);

    Ok(())
}
