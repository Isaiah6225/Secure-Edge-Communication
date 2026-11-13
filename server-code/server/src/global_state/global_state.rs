use crate::{
    format_signature_data,
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
use heapless::{Vec as HVec, String};
use std::{
    fmt::Write as FmtWrite,
    io::{Write, Read},
};

pub async fn manage_enrollment(stream: TcpStream, data_parsed: DeviceEnrl, mut db_client: DBClient) -> Result<(), ServerError>{
    //set up crypto client
    println!("[manage_enrollment] setting up crypto client");
    let read_verifying_key = VerifyingKey::read_public_key_pem_file("./pub_key.pem")?;
    let read_signing_key = SigningKey::read_pkcs8_pem_file("./priv_key.pem")?;
    let crypto_client = CryptoClient::new(read_signing_key, read_verifying_key); 

    //set up network client
    println!("[manage_enrollment] setting up network client");
    let network_client = NetworkClient::new(&stream); 

    //set signature base  and send buffers 
    let mut signature_base = Vec::new();
    let mut send_buffer = String::<2048>::new();

    //complete enrollment checks
    println!("[manage_enrollment] starting enrollment checks");

    //enrollment_time::check_window()?;
    check_device_id::check_id(&data_parsed.device_id)?;

    db_client.check_dev_db(&data_parsed.device_id).await?;
    db_client.save_dev_db(&data_parsed.device_id, DBSave::Pending).await?;

    //respond initial enrollment
    let server_challenge = CryptoClient::gen_server_challenge()?;
    format_signature_data!(&mut signature_base, &server_challenge, &data_parsed.device_id, &data_parsed.nonce);
    let (init_signature, recovery_id) = crypto_client.gen_signature(&signature_base)?;
    let init_signature_bytes = &init_signature.to_vec();

    //write 
    if let Err(e) = write!(
        send_buffer,
        r#"{{"signature": {:?}, "signature_base": {:?}, "server_challenge": {:?}, "device_nonce": {:?}}}"#,
        init_signature_bytes, &signature_base, server_challenge, &data_parsed.nonce
    ){
        println!("[manage_enrollment] error from write {:?}", e);
    };
    println!("[manage_enrollment] init_send_buffer: {:?}", send_buffer);
    stream.try_write(send_buffer.as_bytes())?;
    send_buffer.clear();
    signature_base.clear();

    //final verification
    println!("[manage_enrollment] waiting for device response"); 
    let final_veri = network_client.read_data().await?;
    println!("[manage_enrollment] received response with: {:?}", final_veri); 
    Ok(())
}
    
    /*
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
*/

pub async fn manage_standard_communication(mut stream: TcpStream, data_parsed: DeviceStdComm, mut db_client: DBClient) -> Result<(), ServerError> {
    println!("testing 123");
    Ok(())
}

/*
    let mut session_send_buffer = HVec::<u8, 2048>::new();

    //set up crypto client
    println!("[manage_session] setting up crypto client");
    let read_verifying_key = VerifyingKey::read_public_key_pem_file("./pub_key.pem")?;
    let read_signing_key = SigningKey::read_pkcs8_pem_file("./priv_key.pem")?;
    let crypto_client = CryptoClient::new(read_signing_key, read_verifying_key);

    //set up network client
    println!("[manage_session] setting up network client");
    let network_client = NetworkClient::new(&stream); 

    //check if device is verified
    check_device_id::check_id(&data_parsed.device_id)?;
    db_client.is_dev_verified_db(&data_parsed.device_id).await?;

    if let Err(e) = write!(
        session_send_buffer, 
        r#"{{"device_nonce": {:?}}}"#,
        &data_parsed.nonce
    ){ 
        println!("[manage_enrollment] error from write {:?}", e); 
    };

    println!("[manage_session] initial secure session message length: {:?}", session_send_buffer.len());
    println!("[manage_session] initial secure session message: {:?}", session_send_buffer);
    stream.try_write(session_send_buffer.as_mut_slice())?;
    session_send_buffer.clear();

    println!("[manage_session] waiting for device response"); 
    let initial_sec_response = network_client.read_data().await?;
    println!("[manage_session] received response with: {:?}", initial_sec_response);
    EnrollmentClient::is_valid_enrollment(initial_sec_response)?;

    //write init secure sesson 
    let server_pub_key_bytes = crypto_client.gen_pub_key_bytes()?;
    let (secret, server_ecdh_pub) = CryptoClient::gen_ecdh_pub()?;
    let signature_base_session = CryptoClient::gen_sigunature_base_session(&data_parsed.nonce, &server_ecdh_pub, &data_parsed.device_ecdh_pub, &server_pub_key_bytes)?;

    let (signature_session, recovery_id) = crypto_client.gen_signature(&signature_base_session)?;
    let signature_bytes_session = &signature_session.to_vec();

    println!("[manage_session] signature_bytes_session length: {:?}", signature_bytes_session.len());
    println!("[manage_session] signature_base_session: {:?}", signature_base_session);
    println!("[manage_session] signature_bytes_session length: {:?}", server_ecdh_pub.len());

    if let Err(e) = write!(
        session_send_buffer, 
        r#"{{"signature_base": {:?}, "signature_bytes": {:?}, "server_ecdh": {:?}}}"#,
        signature_base_session, signature_bytes_session, server_ecdh_pub
    ){ 
        println!("[manage_enrollment] error from write {:?}", e); 
    };

    println!("[manage_session] initial secure session message length: {:?}", session_send_buffer.len());
    println!("[manage_session] initial secure session message: {:?}", session_send_buffer);
    stream.try_write(session_send_buffer.as_mut_slice())?;
    session_send_buffer.clear();

    println!("[manage_session] waiting for device response"); 
    let secure_ks = network_client.read_data().await?;
    let mut secure_ks_parsed = parse_packet::parse_secure_ks(secure_ks)?;


    //generate cipher
    let cipher = CryptoClient::gen_session_key(
        &server_pub_key_bytes, 
        secret, 
        &data_parsed.device_ecdh_pub,
        &data_parsed.device_id,
    )?; 

    let mut encrypted_data: &mut [u8] = &mut secure_ks_parsed.encrypted_buf;
    let mut auth_tag: &mut [u8] = &mut secure_ks_parsed.a_tag;

    //try decrypting the message
    CryptoClient::decrypt_symmetric_final(&cipher, &mut auth_tag, &secure_ks_parsed.nonce, &mut encrypted_data, &server_pub_key_bytes)?; 

    //try encrypting the message
    let data_str = "confirmed";
    let mut data_output = [0u8; 9];
    data_output.copy_from_slice(&data_str.as_bytes());
    let mut data: &mut [u8] = &mut data_output;

    let (server_a_tag, nonce, server_encrypted_buf) = CryptoClient::encrypt_symmetric_final(&cipher, &server_pub_key_bytes, data)?;

    //send encrypted message
    if let Err(e) = write!(
        session_send_buffer, 
        r#"{{"server_a_tag": {:?}, "server_nonce": {:?}, "server_encrypted_buf": {:?}}}"#,
        server_a_tag.as_slice(), nonce, server_encrypted_buf, 
    ){ 
        println!("[manage_enrollment] error from write {:?}", e); 
    };

    println!("[manage_session]  secure communication test send buffer: {:?}", session_send_buffer);
    stream.try_write(session_send_buffer.as_mut_slice())?;
    session_send_buffer.clear();
    
    //adding await to give device time to read final packet
    println!("[manage_session] waiting for device response"); 
    let wait = network_client.read_data().await?;
    Ok(())
}
*/
