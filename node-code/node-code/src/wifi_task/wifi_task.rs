use esp_radio::wifi::{
    WifiController,
    ModeConfig,
    WifiEvent,
    ClientConfig

};
use embassy_net::tcp::TcpSocket;
use embassy_time::{Duration, Timer};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::{Receiver, Sender};
use crate::{
    common::{
        enums::EnrollmentSteps,
        structs::{SendPacketInitialEnrl, WifiManager},
    },
};
use log::info;


#[embassy_executor::task]
pub async fn wifi_task(
    mut controller: WifiController<'static>,
    manage_wifi: WifiManager,
    receiver_handle: Receiver<'static, CriticalSectionRawMutex, EnrollmentSteps, 16>,
    sender_handle: Sender<'static, CriticalSectionRawMutex, EnrollmentSteps, 16>
) {
    //set socket buffers
    let mut rx_buffer = [0; 1536];
    let mut tx_buffer = [0; 1536];

    info!("[wifi_task] starting wifi set up and send process");
    loop {
        //TODO Need to consider all the unwraps and make this into a function  
        if !matches!(controller.is_started(), Ok(true)) {
            let client_config = 
                ModeConfig::Client(ClientConfig::default().with_ssid("SEC".into()));
            controller.set_config(&client_config).unwrap();

            info!("[wifi_task] starting wifi_controller");
            controller.start_async().await.unwrap();
            info!("[wifi_task] controller started");

        } else {
            controller.wait_for_event(WifiEvent::ApStop).await;
            Timer::after(Duration::from_millis(5000)).await
        }
        
        loop {
            //check stack is up 
            let state = receiver_handle.receive().await;
            manage_wifi.check_stack().await;

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
        }   
    }
}
