use esp_radio::wifi::{
    WifiController,
    ModeConfig,
    WifiEvent,
    ClientConfig,
    sta_state,
    WifiStaState,
};
use embassy_time::{Duration, Timer};
use log::{warn, info};
use embassy_sync::{
    watch::Sender, 
    blocking_mutex::raw::CriticalSectionRawMutex,
};
use crate::{
    common::enums::WifiConfigStatus,
};

#[embassy_executor::task]
pub async fn wifi_config(
    wifi_password: &'static str,
    mut controller: WifiController<'static>,
    sen0: Sender<'static, CriticalSectionRawMutex, WifiConfigStatus, 1>
) {
    loop {
        match sta_state() {
            WifiStaState::Started => {
                controller.wait_for_event(WifiEvent::StaStop).await;
                Timer::after(Duration::from_millis(5000)).await
            }
            WifiStaState::Stopped => {
                warn!("[wifi_connection] the sta stopped set up.");
                sen0.send(WifiConfigStatus::Down);
            }
            _ => {}
        }

        //TODO Need to consider all the unwraps and make this into a function  
        if !matches!(controller.is_started(), Ok(true)) {
            let client_config = 
                ModeConfig::Client(ClientConfig::default()
                                   .with_ssid("SEC".into())
                                   .with_password(wifi_password.into())
                );
            controller.set_config(&client_config).unwrap();

            info!("[wifi_connection] starting wifi_controller");
            controller.start_async().await.unwrap();
            info!("[wifi_connection] controller started");

            info!("[wifi_connection] connecting to AP");
            match controller.connect_async().await {
                Ok(()) => {
                    sen0.send(WifiConfigStatus::Up);
                }

                Err(_) => {
                    sen0.send(WifiConfigStatus::Down);
                    Timer::after(Duration::from_millis(5000)).await
                }
            }
        } else {
            controller.wait_for_event(WifiEvent::ApStop).await;
            sen0.send(WifiConfigStatus::Down);
            Timer::after(Duration::from_millis(5000)).await
        }
    }
}
