use embassy_net::tcp::TcpSocket;
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
    mut wc_rec0: ReceiverWatch<'static, CriticalSectionRawMutex, WifiConfigStatus, 1>
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
                        /*(
                        WifiCommand::AwaitCommand => {
                            info!("[wifi_task WifiCommand::Await] awaiting receiver from channel");
                            let new_data = 

                            info!("[wifi_task WifiCommand::Await] received packet from channel {:?}", new_data);
                            info!("[wifi_task WifiCommand::Await] sending response over the channel");

                            sender_handle.send(WifiCommand::ReceiveEnrl(1)).await;

                        },
                        */
                        EnrollmentSteps::Initial => {
                            info!("[wifi_task EnrollmentSteps::Initial]"); 
                            let init_packet = manage_wifi.gen_enrollment(&state); 

                            info!("[wifi_task EnrollmentSteps::Initial] created init packet: {:?}", init_packet);
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
