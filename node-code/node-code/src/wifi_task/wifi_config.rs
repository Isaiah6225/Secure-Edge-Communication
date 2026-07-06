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
                info!("[wifi_config] the sta is in the started state");
                controller.wait_for_event(WifiEvent::StaDisconnected).await;
                Timer::after(Duration::from_millis(5000)).await
            }
            WifiStaState::Disconnected => {
                warn!("[wifi_config] the sta disconnected");
                match controller.connect_async().await {
                    Ok(()) => {
                        info!("[wifi_config] connection up");
                        sen0.send(WifiConfigStatus::Up);
                    }

                    Err(e) => {
                        info!("[wifi_config] connection down with error: {:?}", e);
                        sen0.send(WifiConfigStatus::Down);
                        Timer::after(Duration::from_millis(5000)).await
                    }
               }
            }
            _ => {}
        }

        //TODO Need to consider all the unwraps and make this into a function  
        if !matches!(controller.is_started(), Ok(true)) {
            info!("[wifi_config] setting client config");
            let client_config = 
                ModeConfig::Client(ClientConfig::default()
                                   .with_ssid("SEC".into())
                                   .with_password(wifi_password.into())
                );
            controller.set_config(&client_config).unwrap();

            info!("[wifi_config] starting wifi_controller");
            controller.start_async().await.unwrap();
            info!("[wifi_config] controller started");

            info!("[wifi_config] connecting to AP");
            match controller.connect_async().await {
                Ok(()) => {
                    info!("[wifi_config] connection up");
                    sen0.send(WifiConfigStatus::Up);
                }

                Err(e) => {
                    info!("[wifi_config] connection down with error: {:?}", e);
                    sen0.send(WifiConfigStatus::Down);
                    Timer::after(Duration::from_millis(5000)).await
                }
            }
        } else {
            //controller.wait_for_event(WifiEvent::StaDisconnected).await;
            Timer::after(Duration::from_millis(5000)).await
        }
    }
}
