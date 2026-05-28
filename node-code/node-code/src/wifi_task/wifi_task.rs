use esp_radio::wifi::WifiController;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Receiver;
use crate::{
    common::{
        enums::WifiCommand
    },
};
use log::info;

#[embassy_executor::task]
pub async fn wifi_task(
    wifi_controller: WifiController<'static>,
    mut receiver_handle: Receiver<'static, CriticalSectionRawMutex, WifiCommand, 16>
) {
    let mut state = WifiCommand::AwaitCommand;
    loop {
        match state {
            WifiCommand::AwaitCommand => {
                info!("[WifiCommand: Await] awaiting receiver from channel");
                let new_data = receiver_handle.receive().await;
                info!("{:?}", new_data);
            },
            WifiCommand::Connect | WifiCommand::SendEnrlInitial(_) => todo!()
        };
    }
}
