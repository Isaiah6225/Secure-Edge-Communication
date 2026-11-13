use embassy_net::Ipv4Address;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::watch::Receiver as ReceiverWatch;
use log::info;
use embassy_time::{Timer, Duration};
use crate::{
    wifi_task::wifi_executor,
    common::{
        communicators::WTCManager,
        structs::WifiManager,
        enums::{WifiConfigStatus, WifiResponse}, 
    },
};


#[embassy_executor::task]
pub async fn wifi_task(
    wtc_communicator: WTCManager, 
    mut wc_rec0: ReceiverWatch<'static, CriticalSectionRawMutex, WifiConfigStatus, 1>,
    ip_address: Ipv4Address, 
    manage_wifi: WifiManager,
) {


    info!("[wifi_task] starting wifi set up and send process");
    'main: loop {
        //check wifi config
        info!("[wifi_task] checking config_state from wifi_config"); 
        let config_state = wc_rec0.get().await;
        info!("[wifi_task] config_state: {:?}", config_state); 

        match config_state {
            WifiConfigStatus::Up => {
                info!("[wifi_task WifiConfigStatus::Up]"); 
                manage_wifi.check_stack().await;

                if let Err(e) = wifi_executor::wifi_executor(&wtc_communicator, &manage_wifi, ip_address).await {
                    info!("[wifi_task] error from socket connect: {e:?}");
                    info!("[wifi_task] sending response to GSC to retry EnrollmentSteps::Initial");
                    wtc_communicator.send_initial_enrollment(WifiResponse::Failure).await;
                };
            },

            WifiConfigStatus::Down => {
                info!("[wifi_task WifiConfigStatus::Down]"); 
                Timer::after(Duration::from_secs(30)).await
            },
        }
    }
}
