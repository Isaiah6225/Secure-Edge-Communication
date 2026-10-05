use heapless::String;
use core::fmt::Write;
use embassy_net::{
    IpEndpoint,
    IpAddress,
    Ipv4Address,
    tcp::TcpSocket,
};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::{
    channel::{Receiver, Sender},
    watch::Receiver as ReceiverWatch
};
use crate::{
    common::{
        enums::{SendSteps, StdCommSteps, EnrollmentSteps, WifiConfigStatus, WifiCommand},
        structs::{SendPacketInitialEnrl, WifiManager, ReceivePacketInitialEnrl, CryptoClient, ReceivePacketFinVeri, ReceivePacketFinVeriClean},
    },
};
use log::info;
use embassy_time::{Duration, Timer};

#[embassy_executor::task]
pub async fn wifi_task(
    manage_wifi: WifiManager,
    gsc_receiver_handle: Receiver<'static, CriticalSectionRawMutex, SendSteps, 8>,
    wtc_sender_handle: Sender<'static, CriticalSectionRawMutex, WifiCommand, 8>,
    mut wc_rec0: ReceiverWatch<'static, CriticalSectionRawMutex, WifiConfigStatus, 1>,
    ip_address: Ipv4Address, 
    crypto_client: CryptoClient
) {
    //set socket buffers and setting write retry count
    //1792
    let mut rx_buffer = [0; 2048];
    let mut tx_buffer = [0; 1536];
    let mut read_buffer = [0; 2048];
    let mut write_retry_count = 0;

    info!("[wifi_task] starting wifi set up and send process");
    loop {
        //check wifi config
        info!("[wifi_task] checking config_state from wifi_config"); 
        let config_state = wc_rec0.get().await;
        info!("[wifi_task] config_state: {:?}", config_state); 

        match config_state {
            WifiConfigStatus::Up => {
                info!("[wifi_task WifiConfigStatus::Up]"); 
                manage_wifi.check_stack().await;
                
                //receive the state from global state communicator (GSC)
                let state = gsc_receiver_handle.receive().await;
                'session: loop {
                    match state {
                        SendSteps::Enroll(EnrollmentSteps::Enrollment(ecdsa_priv_key, ecdsa_pub_key)) => {
                            /*
                            INITIAL SEND
                            */

                            info!("[wifi_task EnrollmentSteps::Enrollment]"); 
                            //create socket with buffers
                            let mut tcp_socket = TcpSocket::new(manage_wifi.stack, &mut rx_buffer, &mut tx_buffer);
                            tcp_socket.set_timeout(Some(Duration::from_secs(10)));

                            //format initial packet 
                            let init_packet = WifiManager::gen_enrollment_initial(ecdsa_pub_key); 
                            info!("[wifi_task EnrollmentSteps::Initial] created init packet: {:?}", init_packet);

                            //connect to endpoint
                            info!("[wifi_task] EnrollmentSteps::Initial] trying to connect to remote endpoint");
                            let response = tcp_socket.connect(
                                IpEndpoint{
                                    addr: IpAddress::Ipv4(ip_address),
                                    port:7979,
                                }
                            ).await;
                             
                            //handle error in case of connect error 
                            if let Err(e) = response {
                                info!("[wifi_task] EnrollmentSteps::Initial] error from socket connect: {e:?}");
                                info!("[wifi_task] EnrollmentSteps::Initial] sending response to GSC to retry EnrollmentSteps::Initial");
                                wtc_sender_handle.send(WifiCommand::Failure).await;
                                break 'session;
                            }

                            //start initial send 
                            info!("[wifi_task] EnrollmentSteps::Initial] remote endpoint successfully connected to moving to send packet");
                            'initial_send: loop {
                                info!("[wifi_task] EnrollmentSteps::Initial] write retry count before entering if statement: {}", write_retry_count);
                                //check write retry_count
                                if write_retry_count < 3 {
                                    let mut init_send_buffer = String::<512>::new();
                                    info!("[wifi_task] EnrollmentSteps::Initial] created buffer to send data to remote server");

                                    if let Ok(_) = write!(
                                        init_send_buffer,
                                        r#"{{"device_id": {:?}, "device_pub": {:?}, "nonce": {}, "header_byte": {}}}"#,
                                        init_packet.dev_mac_add, init_packet.serialized_vkey, init_packet.device_nonce, init_packet.header_byte
                                    ) {
                                        info!("[wifi_task] EnrollmentSteps::Initial] successfully wrote data to buffer");
                                        info!("[wifi_task] EnrollmentSteps::Initial] buffer: {:?}", init_send_buffer);
                                        let init_request = tcp_socket.write(init_send_buffer.as_bytes()).await;

                                        if let Err(e) = init_request {
                                            info!("[wifi_task] EnrollmentSteps::Initial] buffer failed to create with: {e:?}. Retrying the buffer creation to send");
                                            write_retry_count += 1;
                                            info!("[wifi_task] EnrollmentSteps::Initial] write retry count: {}", write_retry_count);
                                        } else {
                                            info!("[wifi_task] EnrollmentSteps::Initial] successfully sent written buffer and keeping buffer alive");
                                            break 'initial_send; 
                                        }
                                    } else {
                                        info!("[wifi_task] EnrollmentSteps::Initial] write failed procing write retry count: {}", write_retry_count);
                                        write_retry_count += 1;
                                    }
                                } else {
                                    info!("[wifi_task EnrollmentSteps::Initial] write failed after 3 attempts. sending failure response to GSC to retry EnrollmentSteps::Enrollment");
                                    write_retry_count = 0;
                                    wtc_sender_handle.send(WifiCommand::Failure).await;
                                    break 'session;
                                }
                            }
                            
                            /*
                             Initial Read
                            */
                            info!("[wifi_task EnrollmentSteps::InitalRead] awaitng bytes in rx buf");
                            match tcp_socket.read(&mut read_buffer).await {
                                Ok(len) => {
                                    let received_data = &read_buffer[..len];
                                    if let Ok(s) = core::str::from_utf8(received_data) {
                                        info!("[wifi_task EnrollmentSteps::InitialRead] received data from remote server with {:?}", s); 
                                        let parsed_receive_data = ReceivePacketInitialEnrl::new(s);
                                        info!("[wifi_task EnrollmentSteps::InitialRead] parsed_data {:?}", parsed_receive_data);
                                        match parsed_receive_data {
                                            Ok(data) => {
                                                info!("{:?}", init_packet.device_nonce);
                                                let compare_result = CryptoClient::compare_nonce(&init_packet.device_nonce, &data.device_nonce);
                                                let init_read = WifiManager::gen_enrollment_initial_confirmation(compare_result); 
                                                let mut init_send_conf= String::<128>::new();
                                                if let Err(e) = write!(
                                                    init_send_conf,
                                                    r#"{{"header_byte": {:?}, "is_valid": {:?}}}"#,
                                                    init_read.header_byte, init_read.is_valid
                                                ){
                                                    info!("[wifi_task EnrollmentSteps::InitialRead] error from write {:?}", e);
                                                    wtc_sender_handle.send(WifiCommand::Failure).await;
                                                };
                                                info!("[wifi_task EnrollmentSteps::InitialRead] sending data back to server: {:?}", init_send_conf);
                                                tcp_socket.write(init_send_conf.as_bytes()).await;
                                            }
                                            Err(e) => {
                                                info!("[wifi_task EnrollmentSteps::InitialRead] failed to parse data with : {:?}", e);
                                                wtc_sender_handle.send(WifiCommand::Failure).await;
                                            }
                                        }
                                    }
                                }
                                
                                Err(e) => {
                                    info!("[wifi_task EnrollmentSteps::InitialRead] read error sending failure to GSC: {:?}", e);
                                    wtc_sender_handle.send(WifiCommand::Failure).await;
                                    break 'session;
                                }
                            }

                            /*
                            FINAL VERIFICATION READ 
                            */
                            WifiManager::clear_buffer(&mut read_buffer);
                            info!("[wifi_task EnrollmentSteps::FinalVerification] awaiting bytes in rx buf");
                            tcp_socket.wait_read_ready().await;
                            match tcp_socket.read(&mut read_buffer).await {
                                Ok(len) => {
                                    info!("[wifi_task EnrollmentSteps::FinalVerification] length of read_buffer: {:?}", read_buffer.len()); 
                                    let received_data = &read_buffer[..len];
                                    if let Ok(s) = core::str::from_utf8(received_data) {
                                        info!("[wifi_task EnrollmentSteps::FinalVerification] received data from remote server with {:?}", s); 
                                        let parsed_receive_data = ReceivePacketFinVeri::new(s);
                                        info!("[wifi_task EnrollmentSteps::FinalVerification] parsed_data {:?}", parsed_receive_data);
                                        match parsed_receive_data {
                                            Ok(data) => {
                                                info!("[wifi_task EnrollmentSteps::FinalVerification] received data from server yipee");
                                                crypto_client.check_server_signature(&data.signature_bytes, &data.signature_base).expect("server verification failed");
                                                match CryptoClient::sign_server_challenge(&data.server_challenge, &ecdsa_priv_key) {
                                                    Ok(signature_bytes) => {
                                                        let final_send = WifiManager::gen_enrollment_final(data.server_challenge, signature_bytes); 
                                                        let mut init_send_conf= String::<786>::new();
                                                        if let Err(e) = write!(
                                                            init_send_conf,
                                                            r#"{{"header_byte": {:?}, "device_signature": {:?}, "server_challenge": {:?}, "nonce": {:?}}}"#,
                                                            final_send.header_byte, final_send.device_signature, final_send.server_challenge, final_send.nonce
                                                        ){
                                                            info!("[wifi_task EnrollmentSteps::InitialRead] error from write {:?}", e);
                                                            wtc_sender_handle.send(WifiCommand::Failure).await;
                                                        };
                                                        info!("[wifi_task EnrollmentSteps::InitialRead] sending data back to server: {:?}", init_send_conf);
                                                        tcp_socket.write(init_send_conf.as_bytes()).await;
                                                    },
                                                    Err(e) => {
                                                        info!("[wifi_task EnrollmentSteps::FinalVerification] failed to sign server challenge with: {:?}", e);
                                                        wtc_sender_handle.send(WifiCommand::Failure).await;
                                                    }
                                                };
                                            }
                                            Err(e) => {
                                                info!("[wifi_task EnrollmentSteps::FinalVerification] failed to parse data with : {:?}", e);
                                                wtc_sender_handle.send(WifiCommand::Failure).await;
                                            }
                                        }
                                    }
                                }

                                Err(e) => {
                                    info!("[wifi_task EnrollmentSteps::FinalVerification] read error sending failure to GSC: {:?}", e);
                                    wtc_sender_handle.send(WifiCommand::Failure).await;
                                }
                            }


                            /*
                             FINAL VERIFICATION CLEAN UP
                             */
                            WifiManager::clear_buffer(&mut read_buffer);
                            info!("[wifi_task EnrollmentSteps::FinalVerificationCleanUp] awaiting bytes in rx buf");
                            tcp_socket.wait_read_ready().await;
                            match tcp_socket.read(&mut read_buffer).await {
                                Ok(len) => {
                                    let received_data = &read_buffer[..len];
                                    if let Ok(s) = core::str::from_utf8(received_data) {
                                        info!("[wifi_task EnrollmentSteps::FinalVerificationCleanUp] received data from remote server with {:?}", s); 
                                        let parsed_receive_data = ReceivePacketFinVeriClean::new(s);
                                        info!("[wifi_task EnrollmentSteps::FinalVerificationCleanUp] parsed_data {:?}", parsed_receive_data);
                                        match parsed_receive_data {
                                            Ok(data) => {
                                                info!("[wifi_task EnrollmentSteps::FinalVerificationCleanUp] received data from server yipee");
                                                crypto_client.check_server_signature(&data.signature_bytes, &data.signature_base).expect("server verification failed");
                                                match CryptoClient::is_verify(&data.is_verify) {
                                                    Ok(()) => {
                                                        info!("[wifi_task EnrollmentSteps::FinalVerificationCleanUp] success!");
                                                        wtc_sender_handle.send(WifiCommand::Success).await;
                                                    },
                                                    Err(e) => {
                                                        info!("[wifi_task EnrollmentSteps::FinalVerificationCleanUp] failed to parse data with : {:?}", e);
                                                        wtc_sender_handle.send(WifiCommand::Failure).await;
                                                    }
                                                };  
                                            }
                                            Err(e) => {
                                                info!("[wifi_task EnrollmentSteps::FinalVerificationCleanUp] failed to parse data with : {:?}", e);
                                                wtc_sender_handle.send(WifiCommand::Failure).await;
                                            }
                                        }
                                    }
                                }

                                Err(e) => {
                                    info!("[wifi_task EnrollmentSteps::FinalVerificationCleanUp] read error sending failure to GSC: {:?}", e);
                                    wtc_sender_handle.send(WifiCommand::Failure).await;
                                }
                            }
                        }
                        SendSteps::Enroll(EnrollmentSteps::VerifyKeys) => todo!(),
                        SendSteps::StdComm(StdCommSteps::StandardCommunication(ecdh_pub_key)) => {
                            info!(" ");
                        }
                    }
                    
                }
            }

            WifiConfigStatus::Down => {
                info!("[wifi_task WifiConfigStatus::Down]"); 
                Timer::after(Duration::from_secs(30)).await
            },
        }
    }
}
