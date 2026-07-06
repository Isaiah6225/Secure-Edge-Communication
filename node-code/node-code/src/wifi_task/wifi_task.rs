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
        enums::{EnrollmentSteps, WifiConfigStatus, WifiCommand},
        structs::{SendPacketInitialEnrl, WifiManager},
    },
};
use log::info;
use embassy_time::{Duration, Timer};

#[embassy_executor::task]
pub async fn wifi_task(
    manage_wifi: WifiManager,
    gsc_receiver_handle: Receiver<'static, CriticalSectionRawMutex, EnrollmentSteps, 8>,
    wtc_sender_handle: Sender<'static, CriticalSectionRawMutex, WifiCommand, 8>,
    mut wc_rec0: ReceiverWatch<'static, CriticalSectionRawMutex, WifiConfigStatus, 1>,
    ip_address: Ipv4Address
) {
    //set socket buffers and setting write retry count
    let mut rx_buffer = [0; 1536];
    let mut tx_buffer = [0; 1536];
    let mut read_buffer = [0u8; 1024];
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
                let state = gsc_receiver_handle.receive().await;
                loop {
                    //create socket with buffers
                    let mut tcp_socket = TcpSocket::new(manage_wifi.stack, &mut rx_buffer, &mut tx_buffer);
                    tcp_socket.set_timeout(Some(Duration::from_secs(10)));
                    match state {
                        EnrollmentSteps::Initial(ecdsa_pub_key) => {
                            info!("[wifi_task EnrollmentSteps::Initial]"); 
                            
                            // format initial packet 
                            let init_packet = manage_wifi.gen_enrollment_initial(ecdsa_pub_key); 
                            info!("[wifi_task EnrollmentSteps::Initial] created init packet: {:?}", init_packet);
                            
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
                                wtc_sender_handle.send(WifiCommand::Initial).await;
                                break;
                            }
                            
                            info!("[wifi_task] EnrollmentSteps::Initial] remote endpoint successfully connected to moving to send packet");
                            loop {
                                info!("[wifi_task] EnrollmentSteps::Initial] write retry count before entering if statement: {}", write_retry_count);
                                //check write retry_count
                                if write_retry_count < 3 {
                                    let mut init_send_buffer = String::<512>::new();
                                    info!("[wifi_task] EnrollmentSteps::Initial] created buffer to send data to remote server");

                                    if let Ok(_) = write!(
                                        init_send_buffer,
                                        "device_id: {:?}, device_pub: {:?}, nonce: {}",
                                        init_packet.dev_mac_add, init_packet.serialized_vkey, init_packet.device_nonce
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
                                            wtc_sender_handle.send(WifiCommand::FinalVerification).await;
                                        }
                                    } else {
                                        info!("[wifi_task] EnrollmentSteps::Initial] write failed procing write retry count: {}", write_retry_count);
                                        write_retry_count += 1;
                                    }
                                } else {
                                    info!("[wifi_task EnrollmentSteps::Initial] write failed after 3 attempts. sending error response to GSC to retry EnrollmentSteps::Initial");
                                    write_retry_count = 0;
                                    wtc_sender_handle.send(WifiCommand::Initial).await;
                                    break;
                                }
                            }
                        },

                        EnrollmentSteps::FinalVerification => {
                            info!("[wifi_task EnrollmentSteps::FinalVerification] awaitng bytes in rx buf");
                            match tcp_socket.read(&mut read_buffer).await {
                                Ok(0) => {
                                    info!("[wifi_task EnrollmentSteps::FinalVerification] 0 bytes from read");
                                }

                                Ok(len) => {
                                    let received_data = &read_buffer[..len];
                                    if let Ok(s) = core::str::from_utf8(received_data) {
                                        info!("[wifi_task EnrollmentSteps::FinalVerification] received data from remote server with {:?}", s); 
                                    }
                                }
                                
                                Err(e) => {
                                    info!("[wifi_task EnrollmentSteps::FinalVerification] read error: {:?}", e);
                                }
                            }
                        },
                        EnrollmentSteps::VerifyKeys => todo!()
                    };
                }
            },

            WifiConfigStatus::Down => {
                info!("[wifi_task WifiConfigStatus::Down]"); 
                Timer::after(Duration::from_secs(30)).await
            },
        }
    }   
}
