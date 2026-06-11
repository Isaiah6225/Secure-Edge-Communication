use core::net::Ipv4Addr;
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
        enums::{EnrollmentSteps, WifiConfigStatus},
        structs::{SendPacketInitialEnrl, WifiManager},
    },
};
use log::{info, warn};
use embassy_time::{Duration, Timer};

#[embassy_executor::task]
pub async fn wifi_task(
    manage_wifi: WifiManager,
    gsc_receiver_handle: Receiver<'static, CriticalSectionRawMutex, EnrollmentSteps, 16>,
    wtc_sender_handle: Sender<'static, CriticalSectionRawMutex, EnrollmentSteps, 16>,
    mut wc_rec0: ReceiverWatch<'static, CriticalSectionRawMutex, WifiConfigStatus, 1>,
    ip_address: Ipv4Address
) {
    //set socket buffers
    let mut rx_buffer = [0; 1536];
    let mut tx_buffer = [0; 1536];

    info!("[wifi_task] starting wifi set up and send process");
    loop {
        //check stack is up 
        let config_state = wc_rec0.get().await;

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
                        EnrollmentSteps::Initial => {
                            info!("[wifi_task EnrollmentSteps::Initial]"); 
                            let init_packet = manage_wifi.gen_enrollment(&state); 
                            info!("[wifi_task EnrollmentSteps::Initial] created init packet: {:?}", init_packet);
                            
                            info!("[wifi_task] EnrollmentSteps::Initial] trying to connect to remote endpoint");
                            let response = tcp_socket.connect(
                                IpEndpoint{
                                    addr: IpAddress::Ipv4(ip_address),
                                    port:7979,
                                }
                            ).await;
                            
                            if let Err(e) = response {
                                info!("[wifi_task] EnrollmentSteps::Initial] error from socket connect: {e:?}");
                                break;
                            }
                        },

                        EnrollmentSteps::FinalVerification => todo!()
                    };
                }
            },

            WifiConfigStatus::Down => {
                info!("[wifi_task WifiConfigStatus::Down]"); 
                Timer::after(Duration::from_millis(5000)).await
            },
        }
    }   
}
